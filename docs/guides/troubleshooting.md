---
title: Troubleshooting
description: Common fixes for Web UI, local files, OCR, and subtitle issues.
---

# Troubleshooting

Quick fixes for common problems.

## Web UI does not open

1. Wait around 30 seconds on first launch, then refresh.
2. Open `http://127.0.0.1:4568/` manually.
3. On Windows, allow firewall access if prompted.
4. Restart Manatan if the port seems busy.

## Local files not showing up

- Confirm your local folder path in settings.
- Confirm folder structure matches the guides.
- Refresh the list after adding files.

Guides:

- [Local manga](/docs/guides/local-manga)
- [Local anime](/docs/guides/local-anime)

## OCR or lookup feels wrong

- Check dictionary/parsing settings.
- If using Yomitan, apply the parsing recommendation from [Getting started](/docs/guides/getting-started).

## Subtitles are missing or out of sync

- Try another subtitle track.
- Adjust subtitle timing if player controls are available.
- For local files, verify subtitles exist in the file/container.

## Full reset (Windows)

If needed, remove these folders and relaunch:

- `%LOCALAPPDATA%\Tachidesk`
- `%APPDATA%\manatan`
- `%Temp%\Suwayomi*`
- `%Temp%\Tachidesk*`

Also clear site data/cookies for `127.0.0.1` in your browser.

## Still stuck?

Ask in Discord and include platform + what you expected + what happened.

- [Join the Discord](https://discord.gg/tDAtpPN8KK)
