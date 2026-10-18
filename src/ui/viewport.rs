//! Text viewport with scrolling and cursor management.

use ratatui::widgets::{Borders, Paragraph};

use crate::document::{Block, TextStyle};
use crate::document::StringExt;

/// The main text viewport that holds rendered document content.
pub struct Viewport {
    pub document: Vec<Block>,
    pub scroll: u16,
    pub block_height: u16,
}

impl Viewport {
    pub fn new() -> Self {
        Viewport {
            document: Vec::new(),
            scroll: 0,
            block_height: 0,
        }
    }

    pub fn set_document(&mut self, document: Vec<Block>) {
        self.document = document;
        self.recalculate();
    }

    pub fn recalculate(&mut self) {
        self.block_height = Viewport::calculate_block_height(&self.document, 80);
    }

    fn calculate_block_height(blocks: &[Block], width: usize) -> u16 {
        let mut total = 0u16;
        for block in blocks {
            total += Self::calculate_block_height_of(block, width);
        }
        total
    }

    fn calculate_block_height_of(block: &Block, width: usize) -> u16 {
        match block {
            Block::Heading { level: _, text, .. } => {
                let str_width = text.width().max(1) as u16;
                let lines = ((str_width as f32 / width as f32).ceil()) as u16 + 1;
                lines
            }
            Block::Paragraph { spans } => {
                let total_width: usize = spans.iter().map(|s| s.to_string().width()).sum();
                let lines = ((total_width as f32 / width as f32).ceil()) as u16 + 1;
                lines
            }
            Block::CodeBlock { code, .. } => {
                let code_lines = code.lines().count().max(1) as u16;
                code_lines.min(20)
            }
            Block::Blockquote { spans } => {
                let total_width: usize = spans.iter().map(|s| s.to_string().width()).sum();
                ((total_width as f32 / width as f32).ceil()) as u16 + 1
            }
            Block::Image { .. } => 5,
            Block::Table { headers: _, rows } => {
                let header_lines = 1u16;
                let row_lines = rows.len() as u16;
                header_lines + row_lines
            }
            _ => 1,
        }
    }

    /// Get the document content as a ratatui Paragraph.
    pub fn paragraph(&self) -> Paragraph<'static> {
        // Convert document blocks to lines using a simple conversion
        let mut lines = Vec::new();
        for block in &self.document {
            match block {
                Block::Heading { level: _, text, style } => {
                    let accent = match style {
                        TextStyle::Bold => ratatui::style::Style::default().fg(ratatui::style::Color::Blue),
                        _ => ratatui::style::Style::default().fg(ratatui::style::Color::White),
                    };
                    lines.push(ratatui::text::Line::from(ratatui::text::Span::styled(text.clone(), accent)));
                }
                Block::Paragraph { spans } => {
                    for span in spans {
                        let style = ratatui::style::Style::from(span.get_ratatui_style()).fg(ratatui::style::Color::White);
                    lines.push(ratatui::text::Line::from(ratatui::text::Span::styled(span.to_string(), style)));
                    }
                }
                Block::CodeBlock { code, .. } => {
                    for line in code.lines() {
                        lines.push(ratatui::text::Line::from(ratatui::text::Span::styled(line.to_string(), ratatui::style::Style::default().fg(ratatui::style::Color::Gray))));
                    }
                }
                _ => {}
            }
        }
        Paragraph::new(lines)
            .block(ratatui::widgets::Block::default().borders(Borders::ALL))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Rgb(10, 11, 16)))
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
