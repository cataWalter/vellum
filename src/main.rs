//! Vellum - A modern, keyboard-centric, distraction-free TUI web browser
//! with native Sixel graphics support.

mod document;
mod graphics;
mod reader;
mod ui;

use crate::document::VellumDocument;
use crate::reader::Reader;
use crate::ui::{Theme, Viewport};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::time::Instant;

const DEFAULT_URL: &str = "https://example.com";
const HISTORY_LIMIT: usize = 50;

#[derive(Clone, Copy, PartialEq)]
enum AppState {
    Idle,
    Loading,
    Loaded,
    Error,
}

struct App {
    state: AppState,
    document: VellumDocument,
    url: String,
    error: Option<String>,
    reader: Reader,
    viewport: Viewport,
    history: Vec<HistoryEntry>,
    history_index: usize,
    scroll: u16,
    hints: Vec<(char, usize, usize)>,
    hint_index: Option<usize>,
    focus: Focus,
    loading: bool,
    anim_timer: Instant,
    theme: Theme,
}

struct HistoryEntry {
    url: String,
    document: VellumDocument,
}

#[derive(Clone, Copy, PartialEq)]
enum Focus {
    Url,
    Document,
}

impl App {
    pub fn new() -> Self {
        App {
            state: AppState::Idle,
            document: VellumDocument::new(),
            url: DEFAULT_URL.to_string(),
            error: None,
            reader: Reader::new(),
            viewport: Viewport::new(),
            history: Vec::new(),
            history_index: 0,
            scroll: 0,
            hints: Vec::new(),
            hint_index: None,
            focus: Focus::Document,
            loading: false,
            anim_timer: Instant::now(),
            theme: Theme::new(),
        }
    }

    pub fn load_url(&mut self, url: &str) -> Result<(), String> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Err("Empty URL".to_string());
        }

        let parsed = url::Url::parse(trimmed).map_err(|e| format!("Invalid URL: {}", e))?;
        let scheme = parsed.scheme();

        if scheme != "https" {
            return Err("Only HTTPS connections are supported. Use http:// for insecure URLs.".to_string());
        }

        if self.url == trimmed {
            return Ok(());
        }

        if self.history_index + 1 < self.history.len() {
            self.history.truncate(self.history_index + 1);
        }

        if self.history.last().map(|e| e.url != trimmed).unwrap_or(true) {
            self.history.push(HistoryEntry {
                url: trimmed.to_string(),
                document: VellumDocument::new(),
            });
            if self.history.len() > HISTORY_LIMIT {
                self.history.remove(0);
            }
            self.history_index = self.history.len() - 1;
        }

        self.url = trimmed.to_string();
        self.state = AppState::Loading;
        self.error = None;
        self.loading = true;
        self.scroll = 0;
        self.hints.clear();
        self.hint_index = None;
        self.focus = Focus::Document;

        let result = self.reader.fetch(trimmed);
        self.loading = false;

        match result {
            Ok(doc) => {
                self.state = AppState::Loaded;
                self.viewport.set_document(doc.blocks.clone());
                self.document = doc;
                self.scroll = 0;
                Ok(())
            }
            Err(e) => {
                self.state = AppState::Error;
                self.error = Some(e.clone());
                Err(e)
            }
        }
    }

    pub fn go_back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            let entry = &self.history[self.history_index];
            self.url = entry.url.clone();
            self.state = AppState::Loaded;
            self.viewport.set_document(self.document.blocks.clone());
            self.document = entry.document.clone();
            self.scroll = 0;
            self.hints.clear();
            self.hint_index = None;
            self.focus = Focus::Document;
        }
    }

    pub fn go_forward(&mut self) {
        if self.history_index + 1 < self.history.len() {
            self.history_index += 1;
            let entry = &self.history[self.history_index];
            self.url = entry.url.clone();
            self.state = AppState::Loaded;
            self.viewport.set_document(self.document.blocks.clone());
            self.document = entry.document.clone();
            self.scroll = 0;
            self.hints.clear();
            self.hint_index = None;
            self.focus = Focus::Document;
        }
    }

    pub fn set_document(&mut self, doc: VellumDocument) {
        self.viewport.set_document(doc.blocks.clone());
        self.document = doc;
    }

    pub fn scroll_down(&mut self, amount: u16) {
        if self.state == AppState::Loaded {
            let total = self.viewport.block_height;
            self.scroll = (self.scroll + amount).min(total.saturating_sub(1));
        }
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll = self.scroll.saturating_sub(amount);
    }

    pub fn enter_url_mode(&mut self) {
        self.focus = Focus::Url;
    }

    pub fn apply_url(&mut self, url: &str) -> Result<(), String> {
        self.load_url(url)
    }

    pub fn current_url(&self) -> &str {
        &self.url
    }

    pub fn document_title(&self) -> &str {
        &self.document.title
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn scroll_percentage(&self) -> u16 {
        if self.viewport.block_height == 0 {
            return 0;
        }
        ((self.scroll * 100) / self.viewport.block_height).min(100)
    }

    pub fn document(&self) -> &VellumDocument {
        &self.document
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    env_logger::init();

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).expect("Failed to create terminal");
    let mut app = App::new();

    if let Err(e) = app.load_url(DEFAULT_URL) {
        eprintln!("Failed to load default URL: {}", e);
    }

    loop {
        app.render(&mut terminal);

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                app.handle_key(key);
            }
        }
    }
}

impl App {
    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
        let size = terminal.size().unwrap_or(ratatui::layout::Size { width: 80, height: 24 });
        let cols = size.width;
        let rows = size.height;

        terminal.draw(|frame| {
            self.render_header(frame, cols, rows, &self.theme);
            self.render_viewport(frame, cols, rows, &self.theme);
            self.render_status_bar(frame, cols, rows, &self.theme);
        });
    }

    fn render_header(&self, frame: &mut ratatui::Frame, cols: u16, _rows: u16, theme: &Theme) {
        let header_height = 2;
        let header_area = ratatui::layout::Rect::new(0, 0, cols, header_height);

        frame.render_widget(
            ratatui::widgets::Block::default()
                .style(theme.header_bg())
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
            header_area,
        );

        let title_text = if self.document.title.is_empty() {
            "Untitled".to_string()
        } else {
            self.document.title.clone()
        };

        frame.render_widget(
            ratatui::widgets::Paragraph::new(title_text),
            ratatui::layout::Rect::new(2, 1, cols.saturating_sub(4), 1),
        );

        if self.loading {
            let timer = self.anim_timer.elapsed();
            let ticks = (timer.as_millis() / 200) as usize;
            let dot = if ticks % 3 == 0 { "⠋" } else if ticks % 3 == 1 { "⠙" } else { "⠹" };
            frame.render_widget(
                ratatui::widgets::Paragraph::new(format!("Loading {}...", dot)),
                ratatui::layout::Rect::new(2, 0, cols.saturating_sub(4), 1),
            );
        }
    }

    fn render_viewport(&self, frame: &mut ratatui::Frame, cols: u16, rows: u16, _theme: &Theme) {
        let viewport_height = rows.saturating_sub(4);
        let viewport_width = cols.saturating_sub(4);

        match self.state {
            AppState::Loaded => {
                frame.render_widget(
                    self.viewport.paragraph(),
                    ratatui::layout::Rect::new(2, 2, viewport_width, viewport_height),
                );
            }
            AppState::Loading => {
                frame.render_widget(
                    ratatui::widgets::Paragraph::new("Loading... please wait."),
                    ratatui::layout::Rect::new(2, 2, viewport_width, viewport_height),
                );
            }
            AppState::Error => {
                if let Some(err) = &self.error {
                    frame.render_widget(
                        ratatui::widgets::Paragraph::new(format!("Error: {}", err)),
                        ratatui::layout::Rect::new(2, 2, viewport_width, viewport_height),
                    );
                }
            }
            AppState::Idle => {
                frame.render_widget(
                    ratatui::widgets::Paragraph::new("Press 'o' to enter URL"),
                    ratatui::layout::Rect::new(2, 2, viewport_width, viewport_height),
                );
            }
        }
    }

    fn render_status_bar(&self, frame: &mut ratatui::Frame, cols: u16, rows: u16, theme: &Theme) {
        let statusbar_height = 1;
        let status_area = ratatui::layout::Rect::new(0, rows.saturating_sub(statusbar_height), cols, statusbar_height);

        frame.render_widget(
            ratatui::widgets::Block::default()
                .style(theme.statusbar_bg())
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
            status_area,
        );

        let keys = match self.state {
            AppState::Loaded => "[j/k] scroll [d/u] half-page [f] follow [o] open [H/L] back/forward [q] quit",
            AppState::Loading => "[q] quit",
            AppState::Error => "[o] retry [q] quit",
            AppState::Idle => "[o] open [q] quit",
        };

        let status = format!(
            " {} | {} | {}% | {}",
            self.url,
            self.document.title,
            self.scroll_percentage(),
            keys,
        );

        frame.render_widget(
            ratatui::widgets::Paragraph::new(status),
            status_area,
        );
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Loading | AppState::Idle | AppState::Error => {
                match key.code {
                    KeyCode::Char('q') => { std::process::exit(0); }
                    KeyCode::Char('o') => { self.enter_url_mode(); }
                    KeyCode::Char('h') | KeyCode::Char('H') => { self.go_back(); }
                    KeyCode::Char('l') | KeyCode::Char('L') => { self.go_forward(); }
                    _ => {}
                }
            }
            AppState::Loaded => {
                match key.code {
                    KeyCode::Char('q') => { std::process::exit(0); }
                    KeyCode::Char('o') => { self.enter_url_mode(); }
                    KeyCode::Char('h') | KeyCode::Char('H') => { self.go_back(); }
                    KeyCode::Char('l') | KeyCode::Char('L') => { self.go_forward(); }
                    KeyCode::Char('f') => { self.enter_hint_mode(); }
                    KeyCode::Char('j') | KeyCode::Down => { self.scroll_down(1); }
                    KeyCode::Char('k') | KeyCode::Up => { self.scroll_up(1); }
                    KeyCode::Char('d') | KeyCode::PageDown => { self.scroll_down(self.scroll / 2 + 1); }
                    KeyCode::Char('u') | KeyCode::PageUp => { self.scroll_up(self.scroll / 2 + 1); }
                    KeyCode::Char('g') => { self.scroll = 0; }
                    KeyCode::Char('G') => { self.scroll = self.viewport.block_height.saturating_sub(1); }
                    KeyCode::Esc => { self.hints.clear(); self.hint_index = None; }
                    _ => {}
                }
            }
        }
    }

    fn enter_hint_mode(&mut self) {
        self.hints.clear();
        self.hint_index = None;

        let links = self.document().links.clone();
        if links.is_empty() {
            return;
        }

        for (i, _link) in links.iter().enumerate() {
            if i < 26 {
                let ch = (b'a' + i as u8) as char;
                self.hints.push((ch, i, i));
            }
        }

        if self.hints.is_empty() {
            if let Some(_link) = self.document().links.first() {
                self.hints.push(('a', 0, 0));
            }
        }
    }
}
