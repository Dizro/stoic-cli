use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

/// Doric column — same 3D shadow technique as the original cross:
/// ████ = block face, ╗║ = right shadow, ╚═╝ = bottom shadow
const COLUMN_ART: &[&str] = &[
    "        █████████████        ",
    "        ╚═══════════╝        ",
    "          ██║███║██          ",
    "          ██║███║██          ",
    "          ██║███║██          ",
    "          ██║███║██          ",
    "          ██║███║██          ",
    "          ██║███║██          ",
    "        █████████████        ",
    "        ╚═══════════╝        ",
];

const TITLE_ART: &[&str] = &[
    "███████╗████████╗ ██████╗ ██╗ ██████╗",
    "██╔════╝╚══██╔══╝██╔═══██╗██║██╔════╝",
    "███████╗   ██║   ██║   ██║██║██║     ",
    "╚════██║   ██║   ██║   ██║██║██║     ",
    "███████║   ██║   ╚██████╔╝██║╚██████╗",
    "╚══════╝   ╚═╝    ╚═════╝ ╚═╝ ╚═════╝",
];

pub struct BannerState {
    pub phase: u8,
    pub tick: u32,
    pub done: bool,
}

impl BannerState {
    pub fn new() -> Self {
        Self {
            phase: 0,
            tick: 0,
            done: false,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        match self.tick {
            0..=50 => self.phase = 0,    // Pillar fades in (~800ms)
            51..=95 => self.phase = 1,   // Title appears (~700ms)
            96..=140 => self.phase = 2,  // Tagline types in (~700ms)
            141..=175 => self.phase = 3, // Settle
            _ => self.done = true,
        }
    }
}

pub fn render_banner(frame: &mut Frame, area: Rect, state: &BannerState, theme: &Theme) {
    let block = Block::default().style(Style::default().bg(theme.bg));
    frame.render_widget(block, area);

    // Center everything vertically
    let content_height = 10 + 1 + 6 + 1 + 1; // pillar + gap + title + gap + tagline
    let vertical = Layout::vertical([Constraint::Length(content_height as u16)])
        .flex(Flex::Center)
        .split(area);
    let center = vertical[0];

    let chunks = Layout::vertical([
        Constraint::Length(10), // Pillar
        Constraint::Length(1),  // Gap
        Constraint::Length(6),  // Title
        Constraint::Length(1),  // Gap
        Constraint::Length(1),  // Tagline
    ])
    .split(center);

    // Phase 0+: Pillar (fade in effect via opacity simulation)
    {
        let opacity = if state.phase == 0 {
            (state.tick as f32 / 50.0).min(1.0)
        } else {
            1.0
        };
        let pillar_color = interpolate_color(theme.bg, theme.accent_soft, opacity);

        let pillar_lines: Vec<Line> = COLUMN_ART
            .iter()
            .map(|line| Line::from(Span::styled(*line, Style::default().fg(pillar_color))))
            .collect();
        let pillar = Paragraph::new(pillar_lines).alignment(Alignment::Center);
        frame.render_widget(pillar, chunks[0]);
    }

    // Phase 1+: Title
    if state.phase >= 1 {
        let opacity = if state.phase == 1 {
            ((state.tick - 51) as f32 / 44.0).min(1.0)
        } else {
            1.0
        };
        let title_color = interpolate_color(theme.bg, theme.accent, opacity);

        let title_lines: Vec<Line> = TITLE_ART
            .iter()
            .map(|line| Line::from(Span::styled(*line, Style::default().fg(title_color).bold())))
            .collect();
        let title = Paragraph::new(title_lines).alignment(Alignment::Center);
        frame.render_widget(title, chunks[2]);
    }

    // Phase 2+: Tagline (typewriter)
    if state.phase >= 2 {
        let tagline = "Ancient wisdom at your fingertips";
        let chars_visible = if state.phase == 2 {
            let progress = (state.tick - 96) as usize;
            (progress * tagline.len() / 44).min(tagline.len())
        } else {
            tagline.len()
        };
        let visible: String = tagline.chars().take(chars_visible).collect();

        let tag = Paragraph::new(Line::from(Span::styled(
            visible,
            Style::default().fg(theme.text_dim),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(tag, chunks[4]);
    }
}

pub fn interpolate_color(from: ratatui::style::Color, to: ratatui::style::Color, t: f32) -> ratatui::style::Color {
    match (from, to) {
        (
            ratatui::style::Color::Rgb(r1, g1, b1),
            ratatui::style::Color::Rgb(r2, g2, b2),
        ) => {
            let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
            let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
            let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
            ratatui::style::Color::Rgb(r, g, b)
        }
        _ => to,
    }
}
