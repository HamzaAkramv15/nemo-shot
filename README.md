# yami-shot

English · [Русский](README.ru.md)

A screenshot tool for Wayland: the screen freezes, you select, mark up, copy or save, read the text
or a QR code off it. Works on any compositor with layer-shell and screencopy (niri, Hyprland, Sway,
river, KDE). Part of the [yami](https://github.com/Trendorin/yami) desktop, and works on its own:
no yami shell needed.

| neutral look (`plain`) | pixel look (`yami`) |
|---|---|
| ![yami-shot in the neutral look](.github/assets/plain.png) | ![yami-shot in the pixel look](.github/assets/pixel.png) |

## What it does

- An area, a window (on niri) or a whole screen, on every monitor at once, HiDPI and fractional
  scales included (1.25 cuts exactly what you selected).
- Marks: pen, frame, arrow, marker, mosaic to hide things, numbered dots, in six colours and three
  widths; undo and redo.
- A colour picker with a loupe: the colour under the cursor as `#rrggbb` into the clipboard.
- Text from the selection (tesseract, English and Russian on the same picture) and QR codes (zbar)
  into the clipboard.
- Copy, save into a folder, or «save as» through yami-files' dialog (or zenity); a notification with
  a thumbnail and «open» and «folder» buttons.

## Install

**One file** (x86_64, glibc 2.34 or newer) and `grim` and `wl-clipboard` from your distribution:

```sh
curl -LO https://github.com/Trendorin/yami/releases/latest/download/yami-shot
chmod +x yami-shot
./yami-shot install
```

No Rust and no root. `install` asks the language and the look, lists what it will change and writes
into `~/.local` only after your yes (`--yes` takes the defaults).

**From source** (Rust 1.89 or newer): `git clone https://github.com/Trendorin/yami-shot`, then
`cargo build --release --locked` and `target/release/yami-shot install`. The drawing library,
yami-px, comes with the build from the [yami](https://github.com/Trendorin/yami) repository.

`yami-shot deps` lists what it uses: grim and wl-clipboard; optional tesseract with the `eng` and
`rus` data (text), zbar (QR), yami-files or zenity («save as»). `yami-shot uninstall` takes
everything back.

## Use

Bind it to a key in your compositor, for example in niri:

```kdl
binds {
    Print { spawn "yami-shot"; }
    Shift+Print { spawn "yami-shot" "screen"; }
}
```

```sh
yami-shot [area|window|screen] [--delay SECONDS]   # a second start closes the open overlay
yami-shot ocr FILE                                 # the text of a picture to stdout
yami-shot shot -o DIR                              # the overlay as PNG without Wayland
```

| Keys | |
|---|---|
| Space or Enter, Esc | done (copy and save as set below), cancel |
| Tab, W, F | area, window, screen in turn; the window; the screen under the cursor |
| 1 … 8 | select, pen, frame, arrow, marker, mosaic, number, colour picker |
| arrows, Shift+arrows | move the selection by 1 or 10 pixels |
| Ctrl+C | copy the selection; with nothing selected, the colour under the cursor |
| Ctrl+S | save as… |
| Ctrl+Z, Ctrl+Shift+Z or Ctrl+Y | undo, redo |

## Settings

`~/.config/yami/shot.json`:

| Key | Default | What |
|---|---|---|
| `dir` | empty | where to save; empty: `Pictures/Screenshots` |
| `copy`, `save` | `true`, `true` | what «done» does: into the clipboard, into the folder |
| `confirm` | `space` | `space`: done by Space or Enter; `instant`: as soon as you let go of the mouse |
| `loupe` | `true` | the magnifier next to the cursor |
| `notify` | `true` | a notification with the picture |
| `delay` | `3` | the seconds of the ◷ timer button |
| `prefix` | `yami` | the start of the file name |

## Look and themes

Without any configuration it takes a neutral look: the system font, rounded corners, soft shadows,
light or dark as your system is set. Every part of it can be changed in two files:

| File | What |
|---|---|
| `~/.config/yami/style.json` | the look of every yami program: write a theme once, all of them take it |
| `~/.config/yami/style.shot.json` | what yami-shot does differently (only those keys) |

| Key | Values |
|---|---|
| `preset` | `plain` (neutral) or `yami` (pixel art: whole pixels, notched corners, the Tiny5 font inside the binary) |
| `palette`, `accent` | `blood`, `ash`, `sakura`, `amber`, `venom`, `ice`, `dusk`; or any `#rrggbb`: the rest of the palette is made from it |
| `light` | `true` or `false`; absent: as the system |
| `corners`, `radius` | `notch`, `round` or `square`; the radius in pixels |
| `shadow`, `bevel` | `hard`, `soft` or `none`; a raised edge on buttons |
| `font` | a font family |
| `animations`, `animSpeed` | on or off; their length (0.5 fast, 2 slow) |
| `colors` | any of `bg panel surface surfaceHi codeBg border dim accent deep glow bright selection text textDim textFaint onAccent textHi ok warn` |
| `lang` | `ru`, `en`, `pt-BR`, `es`, `de`, `fr`, `uk`, `pl`; empty: as the system |

The overlay reads them at every start. Inside the yami desktop the shell writes `style.json` itself
and has a «screenshots» settings tab for `shot.json`.

## License

MIT
