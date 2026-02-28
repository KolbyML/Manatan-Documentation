---
title: Local anime
description: Supported local anime folder structures for Manatan.
---

# Local anime

Manatan can play anime stored locally on your device.

## 1) Set your local anime folder

1. Open **Settings** > **Browse**.
2. Set **Local anime source location**.
3. Open the Local anime source and pick a show.

If you add episodes later, refresh the episode list.

## 2) Supported folder structures

### Single folder format

You may use one folder per series, with video files directly inside.

```text
📁 [Local anime folder]
├─ 📁 My Anime Title
│   ├─ 🖼️ cover.jpg
│   ├─ 🎥 ep01.mp4
│   └─ 🎥 ep02.mkv
```
### Multiseason format

Manatan also supports libraries with seasonal folders.

```text
📁 [Local anime folder]
├─ 📁 My Anime Title
│   ├─ 📁 Season 1
│   │   ├─ 🖼️ cover.jpg
│   │   ├─ 🎥 ep01.mp4
│   │   └─ 🎥 ep02.mkv
│   └─ 📁 Season 2
│       ├─ 🖼️ cover.jpg
│       ├─ 🎥 ep01.mp4
│       └─ 🎥 ep02.mkv
```

Common formats are `.mp4` and `.mkv`.

## Tips

### Stable ordering

Use naming like `ep01`, `ep02`, `ep10`.

### Custom cover image

Add a `cover.jpg` file in the series folder.

### Android gallery cleanup

Create an empty `.nomedia` file to hide videos/covers from gallery apps.

## Next steps

- [Getting started](/docs/guides/getting-started)
- [Troubleshooting](/docs/guides/troubleshooting)
