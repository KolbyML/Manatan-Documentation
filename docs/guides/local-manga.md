---
title: Local manga
description: Supported local manga folder structures for Manatan.
---

# Local manga

Manatan can read manga stored locally on your device.

## 1) Set your local manga folder

1. Open **Settings** > **Browse**.
2. Set **Local Manga source location**.
3. Open **Browse** > **Sources** > **Local source**.

If you add new chapters later, refresh the chapter list.

## 2) Supported folder structure

Inside your local manga folder:

- one folder per series
- each chapter is either a folder of images or one archive file

### Chapter folders (recommended)

```text
[Local Manga folder]/
  My Series Title/
    cover.jpg
    001/
      001.jpg
      002.jpg
    002/
      001.jpg
      002.jpg
```

### Archive chapters

```text
[Local Manga folder]/
  My Series Title/
    001.cbz
    002.cbz
```

Supported chapter formats include image folders and archive files such as `.zip`/`.cbz` and `.rar`/`.cbr`.

## Common issues

### My series does not show up

1. Confirm the folder path is correct.
2. Confirm chapter folders/files are inside the series folder.
3. Refresh the list.

### Chapters are in the wrong order

Use zero-padded naming like `001`, `002`, `010`.

### Android gallery shows manga images

Create an empty file named `.nomedia` in the local folder.

## Next steps

- [Getting started](/docs/guides/getting-started)
- [Troubleshooting](/docs/guides/troubleshooting)
