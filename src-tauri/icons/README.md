# App icons

`source-placeholder.png` is a generated placeholder (not real branding) used to
produce the icon set checked into this directory — `cargo build`/`tauri build`
require `icon.ico` (and the PNGs referenced by `tauri.conf.json`) to exist, even
for debug builds, since `tauri-build` embeds them as a Windows resource.

To replace with real artwork, run from the repo root (`src-tauri` is a
subfolder so the Tauri CLI can find `tauri.conf.json`):

```powershell
./web/node_modules/.bin/tauri.cmd icon path\to\source-logo.png
```

using a source PNG of at least 1024x1024. This regenerates `32x32.png`,
`128x128.png`, `128x128@2x.png`, `64x64.png`, `icon.ico`, `icon.icns`, and
`icon.png`. The command also generates `android/`/`ios/` subfolders and Windows
Store `Square*Logo.png`/`StoreLogo.png` assets — delete those, as this app only
bundles for desktop (NSIS).
