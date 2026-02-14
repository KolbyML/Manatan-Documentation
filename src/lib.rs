#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocPage {
    pub slug: &'static str,
    pub source: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavItem {
    pub slug: &'static str,
    pub label: &'static str,
    pub hint: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavSection {
    pub label: &'static str,
    pub items: &'static [NavItem],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedDoc<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
}

static DOC_ITEMS: [NavItem; 1] = [NavItem {
    slug: "",
    label: "Overview",
    hint: "Start here",
}];

static GUIDE_ITEMS: [NavItem; 6] = [
    NavItem {
        slug: "guides",
        label: "Guides home",
        hint: "Browse guides",
    },
    NavItem {
        slug: "guides/getting-started",
        label: "Getting started",
        hint: "Install + first steps",
    },
    NavItem {
        slug: "guides/local-manga",
        label: "Local manga",
        hint: "Folder structure + formats",
    },
    NavItem {
        slug: "guides/local-anime",
        label: "Local anime",
        hint: "Folder structure + formats",
    },
    NavItem {
        slug: "guides/novels",
        label: "Novels (EPUB)",
        hint: "Import + reading",
    },
    NavItem {
        slug: "guides/troubleshooting",
        label: "Troubleshooting",
        hint: "Common issues",
    },
];

static FAQ_ITEMS: [NavItem; 3] = [
    NavItem {
        slug: "faq",
        label: "FAQ home",
        hint: "Browse topics",
    },
    NavItem {
        slug: "faq/general",
        label: "General",
        hint: "Downloads + privacy",
    },
    NavItem {
        slug: "faq/local-files",
        label: "Local files",
        hint: "Not showing? ordering",
    },
];

static NAV_SECTIONS: [NavSection; 3] = [
    NavSection {
        label: "Docs",
        items: &DOC_ITEMS,
    },
    NavSection {
        label: "Guides",
        items: &GUIDE_ITEMS,
    },
    NavSection {
        label: "FAQ",
        items: &FAQ_ITEMS,
    },
];

static PAGES: [DocPage; 10] = [
    DocPage {
        slug: "",
        source: include_str!("../docs/index.md"),
    },
    DocPage {
        slug: "guides",
        source: include_str!("../docs/guides/index.md"),
    },
    DocPage {
        slug: "guides/getting-started",
        source: include_str!("../docs/guides/getting-started.md"),
    },
    DocPage {
        slug: "guides/local-manga",
        source: include_str!("../docs/guides/local-manga.md"),
    },
    DocPage {
        slug: "guides/local-anime",
        source: include_str!("../docs/guides/local-anime.md"),
    },
    DocPage {
        slug: "guides/novels",
        source: include_str!("../docs/guides/novels.md"),
    },
    DocPage {
        slug: "guides/troubleshooting",
        source: include_str!("../docs/guides/troubleshooting.md"),
    },
    DocPage {
        slug: "faq",
        source: include_str!("../docs/faq/index.md"),
    },
    DocPage {
        slug: "faq/general",
        source: include_str!("../docs/faq/general.md"),
    },
    DocPage {
        slug: "faq/local-files",
        source: include_str!("../docs/faq/local-files.md"),
    },
];

static NOT_FOUND_PAGE: DocPage = DocPage {
    slug: "404",
    source: include_str!("../docs/404.md"),
};

pub fn all_pages() -> &'static [DocPage] {
    &PAGES
}

pub fn nav_sections() -> &'static [NavSection] {
    &NAV_SECTIONS
}

pub fn find_page(slug: &str) -> Option<&'static DocPage> {
    let normalized = normalize_slug(slug);
    PAGES.iter().find(|page| page.slug == normalized)
}

pub fn not_found_page() -> &'static DocPage {
    &NOT_FOUND_PAGE
}

pub fn parse_doc(source: &str) -> ParsedDoc<'_> {
    let fallback_body = source.trim();
    let Some(frontmatter_block) = source.strip_prefix("---\n") else {
        return ParsedDoc {
            title: "Manatan Docs",
            description: "",
            body: fallback_body,
        };
    };

    let Some((frontmatter, body)) = frontmatter_block.split_once("\n---\n") else {
        return ParsedDoc {
            title: "Manatan Docs",
            description: "",
            body: fallback_body,
        };
    };

    let mut title = None;
    let mut description = None;

    for line in frontmatter.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let Some((raw_key, raw_value)) = trimmed.split_once(':') else {
            continue;
        };

        let key = raw_key.trim();
        let value = trim_wrapping_quotes(raw_value.trim());

        match key {
            "title" => title = Some(value),
            "description" => description = Some(value),
            _ => {}
        }
    }

    ParsedDoc {
        title: title.unwrap_or("Manatan Docs"),
        description: description.unwrap_or(""),
        body: body.trim_start(),
    }
}

fn normalize_slug(slug: &str) -> &str {
    slug.trim().trim_matches('/')
}

fn trim_wrapping_quotes(value: &str) -> &str {
    let stripped_double = value.strip_prefix('"').and_then(|v| v.strip_suffix('"'));
    if let Some(stripped) = stripped_double {
        return stripped;
    }

    let stripped_single = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\''));
    if let Some(stripped) = stripped_single {
        return stripped;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_docs_home_page_with_empty_slug() {
        let page = find_page("").expect("docs home page should exist");
        assert_eq!(page.slug, "");
    }

    #[test]
    fn parses_markdown_frontmatter() {
        let parsed = parse_doc("---\ntitle: Hello\ndescription: Hi there\n---\n\n# Body\n");

        assert_eq!(parsed.title, "Hello");
        assert_eq!(parsed.description, "Hi there");
        assert_eq!(parsed.body, "# Body\n");
    }
}
