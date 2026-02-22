use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{
        StatusCode,
        header::{CACHE_CONTROL, CONTENT_TYPE},
    },
    response::{Html, IntoResponse, Response},
    routing::get,
};
use manatan_documentation::{nav_sections, parse_doc};
use pulldown_cmark::{Options, Parser, html};
use std::{
    env, fs,
    net::SocketAddr,
    path::{Path as FsPath, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:48924";
const HOT_RELOAD_POLL_MS: u64 = 1000;
const DOCS_REPO_BASE_URL: &str = "https://github.com/KolbyML/Manatan-Documentation";

#[derive(Clone)]
struct AppState {
    latest_seen_version: Arc<AtomicU64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_string());
    let socket_addr: SocketAddr = bind_addr.parse()?;

    let initial_version = preview_version_value();
    let state = AppState {
        latest_seen_version: Arc::new(AtomicU64::new(initial_version)),
    };

    let app = Router::new()
        .route("/", get(docs_index_handler))
        .route("/docs", get(docs_index_handler))
        .route("/docs/", get(docs_index_handler))
        .route("/docs/{*path}", get(docs_page_handler))
        .route("/assets/{*path}", get(assets_handler))
        .route("/__preview/version", get(preview_version_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    println!("Preview running at http://{socket_addr}/docs");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn docs_index_handler() -> Response {
    render_docs_response("")
}

async fn docs_page_handler(Path(path): Path<String>) -> Response {
    render_docs_response(&path)
}

async fn preview_version_handler(State(state): State<AppState>) -> Response {
    let version = preview_version_value();
    let previous = state.latest_seen_version.swap(version, Ordering::SeqCst);
    if previous != version {
        println!("[preview] Rebuilding UI: docs changed (version {version})");
    }

    Response::builder()
        .header(CONTENT_TYPE, "text/plain; charset=utf-8")
        .header(CACHE_CONTROL, "no-store")
        .body(Body::from(version.to_string()))
        .unwrap()
}

async fn assets_handler(Path(path): Path<String>) -> Response {
    let asset_path = FsPath::new(env!("CARGO_MANIFEST_DIR")).join("assets").join(&path);
    
    match fs::read(&asset_path) {
        Ok(content) => {
            let content_type = match asset_path.extension().and_then(|s| s.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("svg") => "image/svg+xml",
                Some("webp") => "image/webp",
                _ => "application/octet-stream",
            };
            
            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, content_type)
                .body(Body::from(content))
                .unwrap()
        }
        Err(_) => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Asset not found"))
                .unwrap()
        }
    }
}

fn render_docs_response(slug: &str) -> Response {
    let normalized_slug = normalize_slug(slug);

    if let Some(source) = load_source_for_slug(normalized_slug) {
        return render_docs_page(StatusCode::OK, normalized_slug, &source);
    }

    let not_found_source = load_source_for_slug("404").unwrap_or_else(|| {
        "---\ntitle: Page not found\ndescription: The requested docs page could not be found.\n---\n\n# Page not found\n"
            .to_string()
    });
    render_docs_page(StatusCode::NOT_FOUND, "404", &not_found_source)
}

fn render_docs_page(status: StatusCode, page_slug: &str, source: &str) -> Response {
    let parsed_doc = parse_doc(source);
    let mut content_html = markdown_to_html(parsed_doc.body);
    content_html.push_str(&render_fix_prompt_html(page_slug));
    let sidebar_html = render_sidebar_html(page_slug);
    let toc_html = render_toc_html(page_slug);
    let title = if parsed_doc.title.trim().is_empty() {
        "Manatan Docs".to_string()
    } else {
        format!("{} - Manatan Docs", parsed_doc.title.trim())
    };

    let page_html = HTML_TEMPLATE
        .replace("{{TITLE}}", &escape_html(&title))
        .replace("{{SIDEBAR}}", &sidebar_html)
        .replace("{{TOC}}", &toc_html)
        .replace("{{CONTENT}}", &content_html)
        .replace("{{HOT_RELOAD_POLL_MS}}", &HOT_RELOAD_POLL_MS.to_string());

    if status == StatusCode::OK {
        Html(page_html).into_response()
    } else {
        (status, Html(page_html)).into_response()
    }
}

fn render_sidebar_html(active_slug: &str) -> String {
    let sections = nav_sections();
    let mut html = String::new();

    for section in sections {
        html.push_str(&format!("<div class=\"section\">{}</div>", escape_html(section.label)));

        for item in section.items {
            let active_class = if item.slug == active_slug { " active" } else { "" };
            html.push_str(&format!(
                "<a class=\"item{}\" href=\"{}\">{}<span>{}</span></a>",
                active_class,
                docs_path_for_slug(item.slug),
                escape_html(item.label),
                escape_html(item.hint),
            ));
        }
    }

    html
}

fn render_toc_html(active_slug: &str) -> String {
    let sections = nav_sections();
    if sections.is_empty() {
        return "<div class=\"label\">Explore</div><a href=\"/docs\">Docs overview<div class=\"small\">Start here</div></a>".to_string();
    }

    let active_section = sections
        .iter()
        .find(|section| section.items.iter().any(|item| item.slug == active_slug))
        .unwrap_or(&sections[0]);

    let mut html = String::new();
    html.push_str("<div class=\"label\">In this section</div>");

    for item in active_section.items {
        html.push_str(&format!(
            "<a href=\"{}\">{}<div class=\"small\">{}</div></a>",
            docs_path_for_slug(item.slug),
            escape_html(item.label),
            escape_html(item.hint),
        ));
    }

    html
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn docs_path_for_slug(slug: &str) -> String {
    let normalized = normalize_slug(slug);
    if normalized.is_empty() {
        "/docs".to_string()
    } else {
        format!("/docs/{normalized}")
    }
}

fn normalize_slug(slug: &str) -> &str {
    slug.trim().trim_matches('/')
}

fn render_fix_prompt_html(page_slug: &str) -> String {
    let Some(markdown_path) = markdown_path_for_slug(page_slug) else {
        return String::new();
    };

    let edit_url = format!("{DOCS_REPO_BASE_URL}/edit/main/{markdown_path}");
    let issue_url = format!("{DOCS_REPO_BASE_URL}/issues/new");

    format!(
        "<section class=\"fix-prompt\"><div class=\"callout tip\"><strong>Found a docs issue?</strong><div>Spotted outdated steps, typos, or missing details? Help improve this page for everyone.</div></div><div class=\"hero-actions\"><a class=\"btn btn-primary\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">Edit this page</a><a class=\"btn\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">Open an issue</a></div></section>",
        escape_html(&edit_url),
        escape_html(&issue_url)
    )
}

fn load_source_for_slug(slug: &str) -> Option<String> {
    let path = path_for_slug(slug)?;
    fs::read_to_string(path).ok()
}

fn path_for_slug(slug: &str) -> Option<PathBuf> {
    let relative = markdown_path_for_slug(slug)?;

    Some(FsPath::new(env!("CARGO_MANIFEST_DIR")).join(relative))
}

fn markdown_path_for_slug(slug: &str) -> Option<&'static str> {
    match normalize_slug(slug) {
        "" => Some("docs/index.md"),
        "guides" => Some("docs/guides/index.md"),
        "guides/getting-started" => Some("docs/guides/getting-started.md"),
        "guides/local-manga" => Some("docs/guides/local-manga.md"),
        "guides/local-anime" => Some("docs/guides/local-anime.md"),
        "guides/novels" => Some("docs/guides/novels.md"),
        "guides/jellyfin-setup" => Some("docs/guides/jellyfin-setup.md"),
        "guides/troubleshooting" => Some("docs/guides/troubleshooting.md"),
        "faq" => Some("docs/faq/index.md"),
        "faq/general" => Some("docs/faq/general.md"),
        "faq/local-files" => Some("docs/faq/local-files.md"),
        "404" => Some("docs/404.md"),
        _ => None,
    }
}

fn preview_version_value() -> u64 {
    latest_docs_timestamp(FsPath::new(env!("CARGO_MANIFEST_DIR")).join("docs"))
}

fn latest_docs_timestamp(path: PathBuf) -> u64 {
    latest_timestamp_recursive(&path).unwrap_or(0)
}

fn latest_timestamp_recursive(path: &FsPath) -> Option<u64> {
    let metadata = fs::metadata(path).ok()?;
    let mut latest = system_time_to_millis(metadata.modified().unwrap_or(UNIX_EPOCH));

    if metadata.is_dir() {
        let entries = fs::read_dir(path).ok()?;
        for entry in entries.filter_map(Result::ok) {
            if let Some(child_latest) = latest_timestamp_recursive(&entry.path()) {
                latest = latest.max(child_latest);
            }
        }
    }

    Some(latest)
}

fn system_time_to_millis(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

const HTML_TEMPLATE: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="color-scheme" content="dark" />
    <title>{{TITLE}} - Preview</title>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link
      href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@500;600;700&family=Inter:wght@400;500;600&display=swap"
      rel="stylesheet"
    />
    <style>
      :root {
        --bg: #0b0e14;
        --panel: rgba(255, 255, 255, 0.03);
        --border: rgba(255, 255, 255, 0.08);
        --text: #f0f4f8;
        --text-muted: #94a3b8;
        --link: #7dd3fc;
        --link-hover: #a7f3d0;
        --link-underline: rgba(125, 211, 252, 0.38);
        --link-underline-hover: rgba(167, 243, 208, 0.55);
        --radius: 18px;
        --shadow: 0 24px 60px -12px rgba(0, 0, 0, 0.5);
        --nav-height: 72px;
      }

      * { box-sizing: border-box; }
      html { scroll-behavior: smooth; }

      body {
        margin: 0;
        background: var(--bg);
        color: var(--text);
        font-family: "Inter", sans-serif;
        line-height: 1.7;
        overflow-x: hidden;
        -webkit-font-smoothing: antialiased;
      }

      body::before {
        content: "";
        position: fixed;
        top: -20%;
        left: -10%;
        width: 60%;
        height: 60%;
        background: radial-gradient(circle, rgba(46, 204, 113, 0.09), transparent 70%);
        pointer-events: none;
        z-index: -1;
      }

      body::after {
        content: "";
        position: fixed;
        bottom: -25%;
        right: -15%;
        width: 70%;
        height: 70%;
        background: radial-gradient(circle, rgba(52, 152, 219, 0.09), transparent 70%);
        pointer-events: none;
        z-index: -1;
      }

      a { color: inherit; text-decoration: none; }
      a:hover { text-decoration: underline; text-underline-offset: 3px; }
      h1, h2, h3 {
        font-family: "Space Grotesk", sans-serif;
        letter-spacing: -0.02em;
        margin: 0;
      }
      p { margin: 0; }

      .container { width: min(1200px, calc(100% - 40px)); margin: 0 auto; }

      .nav {
        position: sticky;
        top: 0;
        z-index: 100;
        height: var(--nav-height);
        display: flex;
        align-items: center;
        background: rgba(11, 14, 20, 0.72);
        backdrop-filter: blur(14px);
        -webkit-backdrop-filter: blur(14px);
        border-bottom: 1px solid var(--border);
      }

      .nav-inner {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 18px;
      }

      .brand {
        display: inline-flex;
        align-items: center;
        gap: 12px;
        font-weight: 700;
        font-size: 18px;
      }

      .logo-dot {
        width: 30px;
        height: 30px;
        border-radius: 999px;
        background: radial-gradient(circle at 30% 30%, #6ae7a6, #2ecc71 42%, #3498db 100%);
        box-shadow: 0 14px 34px -18px rgba(46, 204, 113, 0.55);
      }

      .nav-links {
        display: flex;
        align-items: center;
        gap: 18px;
        font-size: 14px;
        font-weight: 600;
        color: var(--text-muted);
      }

      .nav-links a:hover { color: var(--text); text-decoration: none; }

      .pill {
        display: inline-flex;
        align-items: center;
        padding: 8px 14px;
        border-radius: 999px;
        border: 1px solid var(--border);
        background: rgba(255, 255, 255, 0.04);
        font-size: 13px;
        color: var(--text);
        white-space: nowrap;
      }

      .pill:hover { background: rgba(255, 255, 255, 0.07); text-decoration: none; }

      .page { padding: 32px 0 84px; }

      .preview-note {
        margin-bottom: 14px;
        display: inline-flex;
        gap: 10px;
        align-items: center;
        padding: 6px 12px;
        border-radius: 999px;
        border: 1px solid rgba(125, 211, 252, 0.25);
        background: rgba(125, 211, 252, 0.1);
        color: #7dd3fc;
        font-size: 13px;
        font-weight: 600;
      }

      .doc-grid {
        margin-top: 10px;
        display: grid;
        grid-template-columns: 260px minmax(0, 1fr) 220px;
        gap: 26px;
        align-items: start;
      }

      .card {
        background: var(--panel);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
      }

      .sidebar {
        padding: 18px;
        position: sticky;
        top: calc(var(--nav-height) + 18px);
      }

      .section {
        font-size: 12px;
        letter-spacing: 0.08em;
        text-transform: uppercase;
        color: rgba(148, 163, 184, 0.85);
        margin-bottom: 12px;
      }

      .item {
        display: block;
        padding: 10px 12px;
        border-radius: 12px;
        color: var(--text);
        border: 1px solid transparent;
      }

      .item:hover {
        background: rgba(255, 255, 255, 0.04);
        border-color: rgba(255, 255, 255, 0.06);
        text-decoration: none;
      }

      .item span {
        display: block;
        color: rgba(148, 163, 184, 0.92);
        font-size: 12px;
        margin-top: 2px;
        font-weight: 500;
      }

      .item.active {
        background: rgba(255, 255, 255, 0.06);
        border-color: rgba(255, 255, 255, 0.12);
      }

      .content { padding: 26px; }
      .content > * + * { margin-top: 14px; }
      .content h1 { font-size: 34px; line-height: 1.15; margin-top: 6px; }
      .content h2 { font-size: 26px; margin-top: 34px; }
      .content h3 { font-size: 18px; margin-top: 22px; }
      .content p, .content li { color: rgba(240, 244, 248, 0.9); }
      .content ul, .content ol { margin: 10px 0 0; padding-left: 20px; }
      .content li { margin: 8px 0; }

      .content a {
        color: var(--link);
        text-decoration: underline;
        text-decoration-color: var(--link-underline);
        text-underline-offset: 3px;
        text-decoration-thickness: 2px;
      }

      .content a:hover {
        color: var(--link-hover);
        text-decoration-color: var(--link-underline-hover);
      }

      .content a.btn {
        text-decoration: none;
      }

      .content a.btn:not(.btn-primary) {
        color: var(--text);
      }

      .content a.btn:hover {
        text-decoration: none;
      }

      .content blockquote {
        margin: 12px 0 0;
        border-left: 3px solid rgba(125, 211, 252, 0.5);
        padding: 8px 14px;
        background: rgba(125, 211, 252, 0.06);
        border-radius: 10px;
      }

      .content code {
        font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
        font-size: 0.95em;
        background: rgba(255, 255, 255, 0.06);
        border: 1px solid rgba(255, 255, 255, 0.09);
        padding: 2px 8px;
        border-radius: 10px;
      }

      .content pre {
        margin: 10px 0 0;
        padding: 14px 16px;
        border-radius: 16px;
        background: rgba(0, 0, 0, 0.35);
        border: 1px solid rgba(255, 255, 255, 0.08);
        overflow: auto;
      }

      .content pre code { background: transparent; border: none; padding: 0; }

      .content img {
        max-width: 100%;
        height: auto;
        border-radius: 12px;
        margin: 10px 0;
      }

      .fix-prompt {
        margin-top: 26px;
        border-top: 1px solid rgba(255, 255, 255, 0.08);
        padding-top: 20px;
      }

      .callout {
        margin-top: 12px;
        border-radius: 16px;
        padding: 14px 16px;
        border: 1px solid rgba(255, 255, 255, 0.08);
        background: rgba(255, 255, 255, 0.03);
      }

      .callout strong {
        display: block;
        margin-bottom: 6px;
        font-family: "Space Grotesk", sans-serif;
        letter-spacing: -0.01em;
      }

      .callout.tip {
        border-color: rgba(46, 204, 113, 0.22);
        background: rgba(46, 204, 113, 0.06);
      }

      .hero-actions {
        margin-top: 14px;
        display: flex;
        flex-wrap: wrap;
        gap: 12px;
      }

      .btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 12px 18px;
        border-radius: 14px;
        font-weight: 700;
        font-size: 14px;
        border: 1px solid var(--border);
        background: var(--panel);
        transition: transform 0.2s, background 0.2s, border-color 0.2s;
      }

      .btn:hover {
        transform: translateY(-1px);
        background: rgba(255, 255, 255, 0.06);
        border-color: rgba(255, 255, 255, 0.12);
        text-decoration: none;
      }

      .btn-primary {
        border: none;
        background: linear-gradient(135deg, rgba(255, 255, 255, 0.92), rgba(255, 255, 255, 0.86));
        color: #0b0e14;
      }

      .btn-primary:hover {
        background: #fff;
      }

      .toc {
        position: sticky;
        top: calc(var(--nav-height) + 18px);
        padding: 18px;
      }

      .toc .label {
        font-size: 12px;
        letter-spacing: 0.08em;
        text-transform: uppercase;
        color: rgba(148, 163, 184, 0.85);
        margin-bottom: 10px;
      }

      .toc a {
        display: block;
        padding: 8px 10px;
        border-radius: 12px;
        color: rgba(240, 244, 248, 0.82);
        border: 1px solid transparent;
      }

      .toc a:hover {
        background: rgba(255, 255, 255, 0.04);
        border-color: rgba(255, 255, 255, 0.06);
        text-decoration: none;
      }

      .toc .small {
        font-size: 13px;
        color: rgba(148, 163, 184, 0.92);
        margin-top: 2px;
      }

      @media (max-width: 1050px) {
        .doc-grid { grid-template-columns: 260px minmax(0, 1fr); }
        .toc { display: none; }
      }

      @media (max-width: 820px) {
        .nav-links { display: none; }
        .doc-grid { grid-template-columns: 1fr; }
        .sidebar { position: static; }
        .content { padding: 20px; }
      }
    </style>
  </head>
  <body>
    <header class="nav">
      <div class="container nav-inner">
        <a class="brand" href="/docs">
          <span class="logo-dot" aria-hidden="true"></span>
          Manatan Docs Preview
        </a>
        <nav class="nav-links" aria-label="Primary">
          <a href="/docs" style="color: var(--text);">Docs</a>
          <a href="https://github.com/KolbyML/Manatan-Documentation" target="_blank" rel="noopener noreferrer">Repo</a>
        </nav>
        <a class="pill" href="https://github.com/KolbyML/Manatan-Documentation" target="_blank" rel="noopener noreferrer">Edit docs</a>
      </div>
    </header>

    <main class="page">
      <div class="container">
        <div class="preview-note">Local preview with live reload</div>
        <section class="doc-grid">
          <aside class="card sidebar">{{SIDEBAR}}</aside>
          <article class="card content">{{CONTENT}}</article>
          <aside class="card toc">{{TOC}}</aside>
        </section>
      </div>
    </main>

    <script>
      (() => {
        const intervalMs = Number("{{HOT_RELOAD_POLL_MS}}") || 1000;
        let previousVersion = null;

        async function checkForChanges() {
          try {
            const response = await fetch('/__preview/version', { cache: 'no-store' });
            const version = (await response.text()).trim();

            if (previousVersion === null) {
              previousVersion = version;
              return;
            }

            if (version !== previousVersion) {
              window.location.reload();
            }
          } catch (_) {
            // ignore transient polling errors
          }
        }

        setInterval(checkForChanges, intervalMs);
        checkForChanges();
      })();
    </script>
  </body>
</html>
"#;
