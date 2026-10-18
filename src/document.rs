//! Core data model for the Vellum document.

use ratatui::style::Modifier;
use ratatui::text::Text as RatatuiText;
use std::fmt;

pub trait StringExt {
    fn width(&self) -> usize;
}

impl StringExt for String {
    fn width(&self) -> usize {
        use unicode_width::UnicodeWidthStr;
        self.width()
    }
}

/// A styled text span within a paragraph.
#[derive(Debug, Clone, PartialEq)]
pub enum Span {
    /// Plain text with a style.
    Text {
        content: String,
        style: TextStyle,
    },
    /// A reference to a link (displayed as superscript number).
    LinkRef {
        index: usize,
    },
}

impl Span {
    pub fn text(content: &str, style: TextStyle) -> Self {
        Span::Text {
            content: content.to_string(),
            style,
        }
    }

    pub fn link_ref(index: usize) -> Self {
        Span::LinkRef { index }
    }

    /// Render this span as ratatui text.
    pub fn to_ratatui_text(&self, base_style: ratatui::style::Style) -> RatatuiText<'_> {
        match self {
            Span::Text { content, style } => {
                let mut s = base_style;
                s = s.add_modifier(Span::style_to_modifier(*style));
                RatatuiText::raw(content)
            }
            Span::LinkRef { index } => {
                RatatuiText::from(format!("[{}]", index + 1))
                    .style(base_style.add_modifier(Modifier::BOLD))
            }
        }
    }

    pub fn get_ratatui_style(&self) -> ratatui::style::Style {
        match self {
            Span::Text { style, .. } => ratatui::style::Style::from(*style),
            Span::LinkRef { .. } => ratatui::style::Style::default().add_modifier(Modifier::BOLD),
        }
    }
}

impl Span {
    pub fn style_to_modifier(style: TextStyle) -> Modifier {
        match style {
            TextStyle::Bold => Modifier::BOLD,
            TextStyle::Italic => Modifier::ITALIC,
            TextStyle::Code => Modifier::ITALIC,
            TextStyle::Link => Modifier::BOLD,
            TextStyle::Normal => Modifier::empty(),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Span::Text { content, .. } => write!(f, "{}", content),
            Span::LinkRef { index } => write!(f, "[{}]", index + 1),
        }
    }
}

/// Text styling options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextStyle {
    Normal,
    Bold,
    Italic,
    Code,
    Link,
}

impl From<TextStyle> for ratatui::style::Style {
    fn from(st: TextStyle) -> Self {
        match st {
            TextStyle::Normal => ratatui::style::Style::default(),
            TextStyle::Bold => ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD),
            TextStyle::Italic => ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::ITALIC),
            TextStyle::Code => ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
            TextStyle::Link => ratatui::style::Style::default().fg(ratatui::style::Color::LightBlue),
        }
    }
}

/// A block of content in the document.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// A heading (H1, H2, H3...).
    Heading {
        level: u8,
        text: String,
        style: TextStyle,
    },
    /// A paragraph with inline styled spans.
    Paragraph {
        spans: Vec<Span>,
    },
    /// A block of preformatted code.
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    /// A blockquote.
    Blockquote {
        spans: Vec<Span>,
    },
    /// An image with a source URL and alt text.
    Image {
        src: String,
        alt: String,
        width: u32,
        height: u32,
        lazy: bool,
    },
    /// A table with headers and rows.
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

impl Block {
    pub fn heading(level: u8, text: &str, style: TextStyle) -> Self {
        Block::Heading {
            level,
            text: text.to_string(),
            style,
        }
    }

    pub fn paragraph(spans: Vec<Span>) -> Self {
        Block::Paragraph { spans }
    }

    pub fn empty_paragraph() -> Self {
        Block::Paragraph {
            spans: vec![Span::text("", TextStyle::Normal)],
        }
    }

    pub fn code_block(language: Option<&str>, code: &str) -> Self {
        Block::CodeBlock {
            language: language.map(String::from),
            code: code.to_string(),
        }
    }

    pub fn blockquote(spans: Vec<Span>) -> Self {
        Block::Blockquote { spans }
    }

    pub fn image(src: &str, alt: &str, width: u32, height: u32) -> Self {
        Block::Image {
            src: src.to_string(),
            alt: alt.to_string(),
            width,
            height,
            lazy: true,
        }
    }

    pub fn table(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Block::Table { headers, rows }
    }
}

/// A link discovered in the document, with its position for Vimium-style hints.
#[derive(Debug, Clone)]
pub struct LinkInfo {
    pub index: usize,
    pub url: String,
    pub label: String,
    pub row: usize,
    pub col: usize,
}

/// The parsed, content-focused representation of a web page.
#[derive(Debug, Clone)]
pub struct VellumDocument {
    pub title: String,
    pub author: Option<String>,
    pub blocks: Vec<Block>,
    pub links: Vec<LinkInfo>,
}

impl VellumDocument {
    pub fn new() -> Self {
        VellumDocument {
            title: String::new(),
            author: None,
            blocks: Vec::new(),
            links: Vec::new(),
        }
    }

    /// Total number of blocks in the document.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Total number of links in the document.
    pub fn link_count(&self) -> usize {
        self.links.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_display() {
        let span = Span::text("Hello", TextStyle::Normal);
        assert_eq!(span.to_string(), "Hello");

        let span = Span::link_ref(0);
        assert_eq!(span.to_string(), "[1]");
    }

    #[test]
    fn test_block_heading() {
        let block = Block::heading(1, "Title", TextStyle::Normal);
        assert_eq!(block.heading_level(), Some(1));
    }
}

impl Block {
    pub fn heading_level(&self) -> Option<u8> {
        match self {
            Block::Heading { level, .. } => Some(*level),
            _ => None,
        }
    }
}
