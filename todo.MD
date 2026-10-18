
### Project Name: **Vellum**
> **Tagline:** *The aesthetic, content-first TUI web browser with native Sixel graphics.*
> **Origin:** *Vellum* is ancient, refined parchment made for high-end manuscripts and calligraphy. The name represents clarity, distraction-free reading, and beautiful typography in a terminal environment.

*(Alternative runner-up: **Litho** — short for lithography, printing text and images together).*

---

### Master Copilot Prompt

Copy and paste the block below directly into **GitHub Copilot Chat** (or save it as `.github/copilot-instructions.md` in your repository root):

```markdown
# Prompt for GitHub Copilot: Scaffold "Vellum" TUI Browser

You are a Senior Systems & Terminal UI Engineer specializing in Rust, terminal protocols (ANSI, Sixel), and high-performance network clients.

We are building **Vellum**: a modern, keyboard-centric, distraction-free TUI web browser engineered for high-performance reading on low-spec hardware (e.g., Intel Atom netbooks) running inside modern terminal emulators like `mlterm`.

---

## 1. Core Philosophy & Architectural Constraints
- **Content-First ("Reader Mode"):** Do NOT build a full CSS flexbox/grid engine or embed a heavy V8/Chromium engine. Parse web pages by extracting semantic content (Reader View: Title, Author, Article Body, Headings, Code Blocks, Blockquotes, Images, Tables, Links).
- **Target Terminal:** `mlterm` (and terminals supporting 24-bit TrueColor, Unicode Box-Drawing glyphs, and Sixel graphics).
- **Resource Budget:** Idle RAM < 30 MB, 0% idle CPU usage, single compiled binary with zero runtime dependencies.
- **Language & Crates:** Rust.
  - TUI Framework: `ratatui` + `crossterm`
  - HTTP Client: `ureq` (minimal, synchronous, lightweight) or `reqwest` (minimal features, rustls)
  - HTML & Reader Parsing: `readability` or `scraper` (DOM traversal) + `html5ever`
  - Sixel & Image Conversion: `image` (for resizing) + a lightweight Sixel encoder (e.g. `sixel-rs` or direct ANSI escape sequence formatting)
  - Color Theme: Catppuccin Mocha / Tokyo Night (TrueColor hex palette).

---

## 2. Feature Requirements

### A. Network & Content Extraction Pipeline
1. Accept a URL from CLI arguments or an interactive address bar (`Ctrl+L` / `/`).
2. Fetch the HTML document over TLS (HTTPS).
3. Run an article/content extraction algorithm (strip `<script>`, `<nav>`, `<aside>`, cookie modals, and tracking footers; extract the primary `<article>` or `<main>` content).
4. Parse the extracted DOM into an intermediate representation (`VellumDocument`):
   - Text spans with styles (Bold, Italic, Code, Link with reference index [1], [2], etc.).
   - Headers (H1, H2, H3) with distinct TrueColor accents and horizontal separators.
   - Images (`<img>` tags with `src` and `alt`).
   - Tables (formatted into neat Unicode borders `╭─┬─╮`).

### B. Sixel Image Pipeline
1. For detected images, asynchronously or lazily fetch the raw image bytes.
2. Downscale the image preserving aspect ratio to fit within the text container column width (e.g., 72–80 characters wide).
3. Encode the bitmap into Sixel escape sequences (`\ePq...`).
4. Handle terminals without Sixel support gracefully by falling back to `[Image: <alt text>]`.

### C. Aesthetic TUI Interface (Ratatui)
1. **Layout:**
   - Top Header Bar: Clean URL bar, loading indicator, page title.
   - Central Viewport: Centered text column (max width 80 chars) with comfortable left/right margins and smooth vertical scrolling.
   - Bottom Status Bar: Scroll percentage (`64%`), current link target under cursor, keybindings hint (`[q] quit [o] open [j/k] scroll [f] follow link`).
2. **Typography & Styling:**
   - Rounded corners (`╭─╮`, `╰─╯`) for frames and dialogs.
   - Preformatted code blocks (`<pre><code>`) rendered inside a subtle shaded card with a left colored border (`│`).
   - Inline links displayed with a superscript/bracket number (e.g. `Rust[3]`) and mapped to a navigable link list.

### D. Navigation & Controls
- `j` / `k` or `Down` / `Up`: Scroll line by line.
- `d` / `u` or `PageDown` / `PageUp`: Half-page scroll.
- `f` (Follow Link): Overlays letter hints on each link on screen (Vimium-style) so pressing a letter jumps to/loads that URL.
- `o` or `/`: Focus the URL input prompt.
- `H` / `L`: Back / Forward in browsing history.
- `q`: Quit cleanly, restoring terminal raw mode and clearing any Sixel artifacts.

---

## 3. Immediate Implementation Tasks
1. Generate the initial `Cargo.toml` with the necessary dependencies configured with minimal features.
2. Create the project structure:
   - `src/main.rs`: Entry point and application state machine.
   - `src/ui/`: Ratatui layout, theme, and widget rendering.
   - `src/reader/`: HTML parsing, semantic extraction, and AST representation.
   - `src/graphics/sixel.rs`: Image fetching, scaling, and Sixel encoding routines.
3. Provide the full code for `src/main.rs` and the reader pipeline prototype to verify fetching an article and rendering styled text in the terminal.
```

---

### Starter `Cargo.toml` to bootstrap immediately

Save this in your project folder to start coding right away:

```toml
[package]
name = "vellum"
version = "0.1.0"
edition = "2021"

[dependencies]
# TUI & Terminal Handling
ratatui = "0.29"
crossterm = { version = "0.28", features = ["event-stream"] }

# HTTP & Network (lightweight, rustls-based)
ureq = { version = "2.10", features = ["tls"] }
url = "2.5"

# HTML Parsing & Reader Mode
scraper = "0.22"
readability = "0.3"

# Images & Sixel (Minimal codecs to save binary size)
image = { version = "0.25", default-features = false, features = ["png", "jpeg", "webp"] }

# Utilities
unicode-width = "0.2"
```
