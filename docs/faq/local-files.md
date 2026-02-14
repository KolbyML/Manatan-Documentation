---
title: Local files FAQ
description: FAQ for local manga and anime files in Manatan.
---

# Local files

Common questions about local folder structures and refresh behavior.

## What is the local manga folder structure?

Use one folder per series, with chapter folders or chapter archives inside.

```text
[Local Manga folder]/
  My Series Title/
    cover.jpg
    001/
      001.jpg
      002.jpg
    002.cbz
```

Full guide: [Local manga](/docs/guides/local-manga)

## What is the local anime folder structure?

Use one folder per series, with episodes inside.

```text
[Local anime folder]/
  My Anime Title/
    cover.jpg
    ep01.mp4
    ep02.mkv
```

Full guide: [Local anime](/docs/guides/local-anime)

## I added files but they do not show up

1. Confirm the local folder path in settings.
2. Refresh the list.
3. Confirm the structure matches the guides.

## My episodes/chapters are in the wrong order

Use consistent zero-padded naming such as `001`, `002`, `010` or `ep01`, `ep02`, `ep10`.

## How do I stop local media from showing in Android gallery apps?

Add an empty file named `.nomedia` in the local folder.

## Related pages

- [Local manga](/docs/guides/local-manga)
- [Local anime](/docs/guides/local-anime)
- [Troubleshooting](/docs/guides/troubleshooting)
