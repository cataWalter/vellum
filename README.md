# Vellum

> **The aesthetic, content-first TUI web browser with native Sixel graphics.**

Vellum is a keyboard-centric, distraction-free web browser that lives entirely in your terminal. Instead of rendering full HTML/CSS (or embedding a browser engine), Vellum fetches a page over HTTPS, strips away the noise — scripts, navigation, ads, footers — and presents just the content: styled text, headings, code blocks, quotes, tables, and links, rendered in a calm Catppuccin Mocha-themed interface.

The name *Vellum* refers to the ancient, refined parchment used for high-end manuscripts — chosen to represent clarity, distraction-free reading, and beautiful typography in a terminal environment.

Vellum is designed for high-performance reading on low-spec hardware (e.g. Intel Atom netbooks) inside modern terminal emulators such as [`mlterm`](https://sourceforge.net/projects/mlterm/).

## Features

- **Reader-mode extraction** — pages are parsed with [`scraper`](https://crates.io/crates/scraper); scripts, styles, nav bars, asides, footers, headers, and forms are stripped, and the primary content (`<main>`, `<article>`, `[role="main"]`, `#content`, `.post`, …) is extracted.
- **Semantic document model** — extracted content becomes a typed intermediate representation (`VellumDocument`) of headings, paragraphs, code blocks, blockquotes, images, tables, and links.
- **Vim-style navigation** — scroll line-by-line or half-a-page, jump to top/bottom, follow links with hint letters, and move through history — all from the home row.
- **Session history** — back/forward navigation across the last 50 visited pages.
- **Sixel graphics pipeline** — images are fetched, downscaled with Lanczos3 (fit to 80×300), and encoded to Sixel escape sequences for terminals that support them.
- **HTTPS-only** — Vellum refuses plain HTTP by default.
- **Catppuccin Mocha theme** — a TrueColor palette tuned for comfortable reading, with a Tokyo Night variant included.
- **Lightweight by design** — synchronous, minimal dependencies, single static binary, no JavaScript engine, no CSS layout engine.

## Terminal Requirements

- A terminal with **24-bit TrueColor** support (for the theme).
- A **Sixel-capable** terminal (e.g. `mlterm`, `xterm` with sixel, or a compatible emulator) if you want inline images. Terminals without Sixel support degrade gracefully.
- Unicode box-drawing glyph support for the rounded frames.

## Installation

Vellum is written in Rust (edition 2021). Build from source with Cargo:

```sh
git clone https://github.com/cataWalter/vellum.git
cd vellum
cargo build --release
```

The binary will be at `target/release/vellum`.

## Usage

```sh
cargo run --release
```

Vellum starts by loading `https://example.com`. Once the first page is parsed you can read, scroll, and follow links.

Enable diagnostic logging with `env_logger` via the `RUST_LOG` environment variable:

```sh
RUST_LOG=debug cargo run --release
```

## Keybindings

| Key | Action |
| --- | --- |
| `j` / `↓` | Scroll down one line |
| `k` / `↑` | Scroll up one line |
| `d` / `PgDn` | Scroll down half a page |
| `u` / `PgUp` | Scroll up half a page |
| `g` | Jump to top |
| `G` | Jump to bottom |
| `o` | Open / enter URL |
| `f` | Follow link (Vimium-style letter hints) |
| `H` / `h` | Back in history |
| `L` / `l` | Forward in history |
| `Esc` | Dismiss link hints |
| `q` | Quit |

The status bar shows the current URL, page title, scroll percentage, and a context-sensitive keybinding reminder.

## Architecture

```
src/
├── main.rs         # Application state machine, event loop, and TUI rendering
├── reader.rs       # Fetch pipeline: HTTPS GET (ureq) + HTML parsing & content extraction (scraper)
├── document.rs     # Core data model: VellumDocument, Block, Span, TextStyle, LinkInfo
├── graphics.rs     # Image fetching, Lanczos3 downscaling, and Sixel encoding
└── ui/
    ├── mod.rs
    ├── theme.rs    # Catppuccin Mocha palette + Tokyo Night variant
    └── viewport.rs # Scrollable text viewport, block-height calculation, paragraph layout
```

**Pipeline:** URL → `ureq` HTTPS fetch → `scraper` DOM parse → content extraction (strip chrome, find the article) → `VellumDocument` block list → ratatui rendering with the viewport/theme layer.

### Dependencies

| Crate | Purpose |
| --- | --- |
| `ratatui` | TUI widgets and layout |
| `crossterm` | Terminal event handling |
| `ureq` | Lightweight synchronous HTTPS client (rustls) |
| `url` | URL parsing and resolution |
| `scraper` | HTML parsing and CSS-selector-based extraction |
| `image` | Image decoding and resizing (PNG, JPEG, WebP, HDR) |
| `unicode-width` | Correct text-width measurement for layout |
| `log` / `env_logger` | Diagnostics |

## Roadmap

Vellum is under active development. Known next steps:

- Rewrite the Sixel encoder to emit raw `Vec<u8>` instead of `String`.
- Finish URL input mode (`Focus::Url`) with an interactive address bar.
- Wire up link-hint selection so pressing a hint letter navigates to that URL.
- Proper terminal setup/teardown (raw mode, alternate screen, Sixel artifact cleanup).
- Pass the scroll offset through to the `Paragraph` widget for true scrolling.
- Richer table rendering with Unicode borders.

## License

MIT

## Name Origin

> *Vellum* is ancient, refined parchment made for high-end manuscripts and calligraphy. The name represents clarity, distraction-free reading, and beautiful typography in a terminal environment.
>
> *(Alternative runner-up: **Litho** — short for lithography, printing text and images together).*
