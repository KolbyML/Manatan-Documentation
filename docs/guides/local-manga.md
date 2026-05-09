---
title: Local manga
description: Supported local manga folder structures for Manatan.
---

# Local manga

Manatan can read manga stored locally on your device from image folders, chapter archives, and EPUB files.

## 1) Set your local manga folder

1. Open **Settings** > **Browse**.
2. Set **Local Manga source location**.
3. Open **Browse** > **Sources** > **Local source**.

If you add new chapters later, refresh the chapter list.

## 2) Supported folder structures

Inside your local manga folder:

- one folder per series
- each chapter is a folder of images, one archive file, or one EPUB file

### Chapter folders (recommended)

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

### Archive chapters

```text
📁 [Local Manga folder]
├─ 📁 My Series Title
│   ├─ 📦 001.cbz
│   └─ 📦 002.cbz
```

### EPUB manga files

Use one EPUB per volume or chapter inside the series folder.

```text
📁 [Local Manga folder]
├─ 📁 My Series Title
│   ├─ 📘 Volume 01.epub
│   └─ 📘 Volume 02.epub
```

Supported local manga formats include image folders, `.zip`/`.cbz`, `.rar`/`.cbr`, and `.epub` files.

## Common issues

### My series does not show up

1. Confirm the folder path is correct.
2. Confirm chapter folders, archive files, or EPUB files are inside the series folder.
3. Refresh the list.

### Chapters are in the wrong order

Use zero-padded naming like `001`, `002`, `010`.

### Android gallery shows manga images

Create an empty file named `.nomedia` in the local folder.

## Next steps

- [Getting started](/docs/guides/getting-started)
- [Troubleshooting](/docs/guides/troubleshooting)
