# Manatan-Documentation

Public Markdown docs for Manatan.

This repository is designed to be imported by the private website service as a Rust crate (`manatan-documentation`), so docs content can be updated here without editing HTML templates in the private repo.

## Repository layout

- `docs/` - all user-facing documentation pages in Markdown (`.md`)
- `src/lib.rs` - lightweight page index + frontmatter parser used by the website
- `src/bin/preview-docs.rs` - local docs-only preview server

## Preview docs locally

Run a docs-only local server so contributors can see rendered output before opening a PR:

```bash
cargo docs-preview
```

Then open:

```text
http://127.0.0.1:48924/docs
```

The preview auto-reloads when files in `docs/` change.

Each rendered page includes "Edit this page" and "Open an issue" prompts, similar to Mihon's docs flow.

You can change the host/port with `BIND_ADDR`, for example:

```bash
BIND_ADDR=0.0.0.0:48924 cargo docs-preview
```

## Contributing documentation

If you want to update docs, you only need to edit Markdown files in `docs/`.

### 1) Pick the page

- Guides: `docs/guides/*.md`
- FAQ: `docs/faq/*.md`
- Docs home: `docs/index.md`

### 2) Keep frontmatter at the top

Each page should start with:

```md
---
title: Your page title
description: One sentence summary for SEO and previews.
---
```

### 3) Write in plain Markdown

- Prefer short headings and bullet lists
- Use fenced code blocks for paths/commands
- Use internal links like `/docs/guides/local-manga`

### 4) Submit your changes

1. Create a branch
2. Open a pull request
3. Mention what changed and why

## Release flow

The private website consumes this crate. Once changes in this repo are pushed, update the dependency reference in the website repo (if pinned) and deploy.
