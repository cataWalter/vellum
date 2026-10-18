//! Theme based on Catppuccin Mocha / Tokyo Night colour palettes.

use ratatui::style::{Color, Modifier, Style};

/// Catppuccin Mocha colour palette (TrueColor).
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub base: Color,
    pub surface0: Color,
    pub surface1: Color,
    pub surface2: Color,
    pub surface3: Color,
    pub text: Color,
    pub subtext1: Color,
    pub subtext0: Color,
    pub overlay2: Color,
    pub overlay1: Color,
    pub overlay0: Color,
    pub blue: Color,
    pub blue2: Color,
    pub mauve: Color,
    pub mauve2: Color,
    pub green: Color,
    pub green2: Color,
    pub yellow: Color,
    pub yellow2: Color,
    pub red: Color,
    pub red2: Color,
    pub pink: Color,
    pub pink2: Color,
}

impl Theme {
    /// Creates the default Catppuccin Mocha-inspired palette.
    pub fn new() -> Self {
        Theme {
            base: Color::Rgb(10, 11, 16),
            surface0: Color::Rgb(38, 39, 51),
            surface1: Color::Rgb(46, 47, 60),
            surface2: Color::Rgb(54, 55, 69),
            surface3: Color::Rgb(62, 63, 78),
            text: Color::Rgb(228, 228, 235),
            subtext1: Color::Rgb(152, 150, 170),
            subtext0: Color::Rgb(124, 121, 140),
            overlay2: Color::Rgb(255, 255, 255),
            overlay1: Color::Rgb(212, 212, 229),
            overlay0: Color::Rgb(184, 184, 208),
            blue: Color::Rgb(135, 189, 240),
            blue2: Color::Rgb(113, 181, 255),
            mauve: Color::Rgb(208, 135, 215),
            mauve2: Color::Rgb(202, 138, 215),
            green: Color::Rgb(140, 238, 146),
            green2: Color::Rgb(111, 230, 180),
            yellow: Color::Rgb(255, 237, 167),
            yellow2: Color::Rgb(250, 228, 150),
            red: Color::Rgb(255, 115, 115),
            red2: Color::Rgb(253, 105, 105),
            pink: Color::Rgb(243, 139, 168),
            pink2: Color::Rgb(235, 104, 156),
        }
    }

    /// Main text style.
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text)
    }

    /// Subtle text style.
    pub fn subtext_style(&self) -> Style {
        Style::default().fg(self.subtext1)
    }

    /// Background style.
    pub fn background_style(&self) -> Style {
        Style::default().bg(self.base)
    }

    /// Header/title style.
    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.overlay2)
            .add_modifier(Modifier::BOLD)
    }

    /// Accent style (for links, key highlights).
    pub fn accent_style(&self) -> Style {
        Style::default().fg(self.blue)
    }

    /// Error/danger style.
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.red)
    }

    /// Success/green style.
    pub fn success_style(&self) -> Style {
        Style::default().fg(self.green)
    }

    /// Warning/yellow style.
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.yellow)
    }

    /// Muted/comment style.
    pub fn muted_style(&self) -> Style {
        Style::default().fg(self.subtext1)
    }

    /// Code block background.
    pub fn code_bg(&self) -> Style {
        Style::default().bg(self.surface1)
    }

    /// Blockquote text style (green tint).
    pub fn blockquote_style(&self) -> Style {
        Style::default()
            .fg(self.green2)
            .add_modifier(Modifier::ITALIC)
    }

    /// Code text style.
    pub fn code_style(&self) -> Style {
        Style::default().fg(self.overlay1)
    }

    /// Heading styles by level (H1 is most prominent).
    pub fn heading_style(&self, level: u8) -> Style {
        let base_style = if level == 1 {
            Style::default().fg(self.overlay2).add_modifier(Modifier::BOLD)
        } else if level == 2 {
            Style::default().fg(self.blue).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.overlay1)
        };
        
        if level == 1 {
            base_style
        } else {
            base_style
        }
    }

    /// Link text style.
    pub fn link_style(&self) -> Style {
        Style::default().fg(self.blue)
    }

    /// Header bar background (slightly lighter than base).
    pub fn header_bg(&self) -> Style {
        Style::default().bg(self.surface0)
    }

    /// Status bar background.
    pub fn statusbar_bg(&self) -> Style {
        Style::default().bg(self.surface2)
    }

    /// Frame style for content area.
    pub fn frame_style(&self) -> Style {
        Style::default().bg(self.surface0)
    }

    /// Cell background (for link hint letters).
    pub fn cell_bg(&self) -> Style {
        Style::default().bg(self.surface1)
    }
}

/// Tokyo Night variant of the theme.
#[derive(Debug, Clone, Copy)]
pub struct TokyoNightTheme {
    pub base: Color,
    pub surface0: Color,
    pub surface1: Color,
    pub surface2: Color,
    pub text: Color,
    pub subtext1: Color,
    pub blue: Color,
    pub blue2: Color,
    pub green: Color,
    pub yellow: Color,
    pub red: Color,
    pub magenta: Color,
}

impl TokyoNightTheme {
    pub fn new() -> Self {
        TokyoNightTheme {
            base: Color::Rgb(16, 18, 25),
            surface0: Color::Rgb(29, 31, 44),
            surface1: Color::Rgb(40, 43, 60),
            surface2: Color::Rgb(53, 56, 74),
            text: Color::Rgb(214, 215, 225),
            subtext1: Color::Rgb(160, 164, 189),
            blue: Color::Rgb(149, 165, 191),
            blue2: Color::Rgb(189, 196, 255),
            green: Color::Rgb(146, 189, 137),
            yellow: Color::Rgb(197, 179, 102),
            red: Color::Rgb(255, 121, 119),
            magenta: Color::Rgb(216, 142, 182),
        }
    }

    pub fn apply_to(&self, style: &mut Style) {
        *style = style.clone().fg(self.text);
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}
