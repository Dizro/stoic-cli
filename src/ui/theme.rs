use ratatui::style::Color;
use serde::{Deserialize, Serialize};

pub struct Theme {
    pub bg: Color,
    pub surface: Color,
    pub border: Color,
    pub border_active: Color,
    pub text: Color,
    pub text_dim: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub highlight_bg: Color,
    pub search_match: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ThemeName {
    #[default]
    Obsidian,
    Marble,
    Parchment,
    Bronze,
    Terminal,
}

impl ThemeName {
    pub fn next(self) -> Self {
        match self {
            ThemeName::Obsidian => ThemeName::Marble,
            ThemeName::Marble => ThemeName::Parchment,
            ThemeName::Parchment => ThemeName::Bronze,
            ThemeName::Bronze => ThemeName::Terminal,
            ThemeName::Terminal => ThemeName::Obsidian,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ThemeName::Obsidian => "Obsidian",
            ThemeName::Marble => "Marble",
            ThemeName::Parchment => "Parchment",
            ThemeName::Bronze => "Bronze",
            ThemeName::Terminal => "Terminal",
        }
    }
}


pub fn get_theme(name: ThemeName) -> Theme {
    match name {
        ThemeName::Obsidian => OBSIDIAN,
        ThemeName::Marble => MARBLE,
        ThemeName::Parchment => PARCHMENT,
        ThemeName::Bronze => BRONZE,
        ThemeName::Terminal => TERMINAL,
    }
}

/// Obsidian — deep, strict dark theme (default)
const OBSIDIAN: Theme = Theme {
    bg: Color::Rgb(15, 15, 20),            // near-black with blue undertone
    surface: Color::Rgb(25, 25, 32),       // slightly lighter
    border: Color::Rgb(55, 55, 70),        // muted border
    border_active: Color::Rgb(200, 200, 210), // bright border
    text: Color::Rgb(230, 230, 235),       // off-white text
    text_dim: Color::Rgb(140, 140, 155),   // dimmed text
    text_muted: Color::Rgb(90, 90, 105),   // muted text
    accent: Color::Rgb(255, 255, 255),     // white accent
    accent_soft: Color::Rgb(180, 180, 195),// soft accent
    highlight_bg: Color::Rgb(45, 45, 60),  // selection bg
    search_match: Color::Rgb(218, 165, 32),// golden highlight
};

/// Marble — cold, clean white theme
const MARBLE: Theme = Theme {
    bg: Color::Rgb(248, 248, 252),         // cool white
    surface: Color::Rgb(240, 240, 246),    // slightly gray
    border: Color::Rgb(195, 195, 210),     // cool gray border
    border_active: Color::Rgb(60, 60, 80), // dark border
    text: Color::Rgb(25, 25, 35),          // near-black text
    text_dim: Color::Rgb(110, 110, 130),   // dimmed text
    text_muted: Color::Rgb(155, 155, 175), // muted text
    accent: Color::Rgb(10, 10, 15),        // near-black accent
    accent_soft: Color::Rgb(70, 70, 90),   // soft accent
    highlight_bg: Color::Rgb(215, 215, 230), // selection bg
    search_match: Color::Rgb(180, 120, 20),// warm gold
};

/// Parchment — warm cream/sepia tones, comfortable long reading
const PARCHMENT: Theme = Theme {
    bg: Color::Rgb(245, 240, 225),         // warm cream
    surface: Color::Rgb(237, 230, 211),    // slightly darker cream
    border: Color::Rgb(196, 181, 153),     // warm tan
    border_active: Color::Rgb(120, 100, 70), // dark warm brown
    text: Color::Rgb(55, 47, 35),          // dark brown
    text_dim: Color::Rgb(140, 125, 100),   // muted brown
    text_muted: Color::Rgb(168, 155, 132), // light brown
    accent: Color::Rgb(40, 32, 20),        // near-black brown
    accent_soft: Color::Rgb(100, 85, 60),  // medium brown
    highlight_bg: Color::Rgb(210, 195, 160), // warm tan
    search_match: Color::Rgb(180, 100, 30),// warm orange
};

/// Bronze — dark background with warm copper/orange accents
const BRONZE: Theme = Theme {
    bg: Color::Rgb(18, 14, 10),            // very dark warm
    surface: Color::Rgb(30, 24, 18),       // dark brown surface
    border: Color::Rgb(85, 65, 45),        // bronze border
    border_active: Color::Rgb(205, 160, 100), // bright bronze
    text: Color::Rgb(235, 225, 210),       // warm off-white
    text_dim: Color::Rgb(168, 148, 120),   // warm dim
    text_muted: Color::Rgb(120, 100, 75),  // warm muted
    accent: Color::Rgb(220, 175, 110),     // bright bronze accent
    accent_soft: Color::Rgb(185, 145, 90), // soft bronze
    highlight_bg: Color::Rgb(55, 42, 28),  // warm dark selection
    search_match: Color::Rgb(240, 180, 50),// golden
};

/// Terminal — transparent, uses the terminal's own background
const TERMINAL: Theme = Theme {
    bg: Color::Reset,
    surface: Color::Reset,
    border: Color::Rgb(71, 85, 105),
    border_active: Color::Rgb(226, 232, 240),
    text: Color::Rgb(241, 245, 249),
    text_dim: Color::Rgb(148, 163, 184),
    text_muted: Color::Rgb(100, 116, 139),
    accent: Color::Rgb(255, 255, 255),
    accent_soft: Color::Rgb(203, 213, 225),
    highlight_bg: Color::Rgb(55, 70, 95),
    search_match: Color::Rgb(251, 191, 36),
};
