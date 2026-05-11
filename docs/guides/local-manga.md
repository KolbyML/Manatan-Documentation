---
title: Local manga
description: Supported local manga folder structures for Manatan.
---

# Local manga

Manatan can read manga stored locally on your device from image folders, chapter archives, and EPUB files. You can either organize a series folder with chapters inside it, or drop a single `.epub`, `.zip`, or `.cbz` directly into the root of your local manga folder.

## 1) Set your local manga folder

1. Open **Settings** > **Browse**.
2. Set **Local Manga source location**.
3. Open **Browse** > **Sources** > **Local source**.

If you add new chapters later, refresh the chapter list.

## 2) Supported folder structures

Inside your local manga folder, Manatan supports two layouts:

- one folder per series, with chapter folders or chapter archives inside
- one standalone `.epub`, `.zip`, or `.cbz` file in the root folder

### Standalone root archives

Put a single manga file directly in the local manga folder.

```text
📁 [Local Manga folder]
├─ 📦 Look Back.cbz
├─ 📦 My One Shot.zip
└─ 📘 Full Color Volume.epub
```

Manatan shows each root archive as its own manga entry. The file name becomes the manga title, so `Look Back.cbz` appears as `Look Back`. Opening it shows one chapter, also named from the file. EPUB manga uses the EPUB spine order when available; ZIP and CBZ files use natural image filename order.

This is the fastest option for one-shots, single-volume manga, or files you already keep as complete archives. It is less useful for ongoing series because every root archive appears as a separate manga instead of grouping volumes together.

### Series folder with chapter folders

```text
📁 [Local Manga folder]
├─ 📁 My Series Title
│   ├─ 🖼️ cover.jpg
│   ├─ 📁 001
│   │   ├─ 🖼️ 001.jpg
│   │   └─ 🖼️ 002.jpg
│   └─ 📁 002
│       ├─ 🖼️ 001.jpg
│       └─ 🖼️ 002.jpg
```

### Series folder with archive chapters

```text
📁 [Local Manga folder]
├─ 📁 My Series Title
│   ├─ 📦 001.cbz
│   └─ 📦 002.cbz
```

### Series folder with EPUB manga files

Use one EPUB per volume or chapter inside the series folder.

```text
📁 [Local Manga folder]
├─ 📁 My Series Title
│   ├─ 📘 Volume 01.epub
│   └─ 📘 Volume 02.epub
```

Supported local manga formats include image folders, `.zip`/`.cbz`, and `.epub` files. Image pages can be `.jpg`, `.jpeg`, `.jfif`, `.png`, `.webp`, `.gif`, `.bmp`, `.jxl`, or `.avif`.

## Which layout should I use?

Use a root `.epub`, `.zip`, or `.cbz` when:

- the file is a one-shot or complete single volume
- you want the least setup: drop the file in `local-manga` and refresh
- you do not need a custom `cover.jpg` or series-level metadata folder

Use a series folder when:

- you have multiple chapters or volumes that should appear under one manga
- you want to add `cover.jpg` or `ComicInfo.xml` for the whole series
- you expect to keep adding chapters over time

Use chapter image folders when you want to inspect or replace individual pages without rebuilding an archive. Use `.zip` or `.cbz` chapter archives when you want fewer files on disk and cleaner syncing. Use `.epub` when the manga already came packaged that way; Manatan reads the image pages from the EPUB, but text-heavy novel EPUBs are usually better handled by the EPUB/novel reader instead of Local Manga.

## Common issues

### My series does not show up

1. Confirm the folder path is correct.
2. Confirm the root file is `.epub`, `.zip`, or `.cbz`, or that chapter folders, archive files, or EPUB files are inside the series folder.
3. Refresh the list.

### Chapters are in the wrong order

Use zero-padded naming like `001`, `002`, `010`. For ZIP and CBZ archives, page order comes from the image filenames inside the archive. For EPUB manga, page order follows the EPUB spine when it is available.

### Android gallery shows manga images

Create an empty file named `.nomedia` in the local folder.

## Next steps

- [Getting started](/docs/guides/getting-started)
- [Troubleshooting](/docs/guides/troubleshooting)
