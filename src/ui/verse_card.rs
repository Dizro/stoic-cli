use crate::api::types::Verse;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

/// Render a single passage or range of passages in a beautiful framed card.
pub fn render_verse_card(frame: &mut Frame, area: Rect, verses: &[Verse], theme: &Theme) {
    if verses.is_empty() {
        return;
    }

    let first = &verses[0];
    let reference = if verses.len() == 1 {
        first.reference()
    } else {
        let last = &verses[verses.len() - 1];
        format!(
            "{} {}:{}-{}",
            first.book, first.chapter, first.verse, last.verse
        )
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_active))
        .padding(Padding::new(2, 2, 1, 1))
        .style(Style::default().bg(theme.surface));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Length(2), // Reference header
        Constraint::Min(1),   // Text
        Constraint::Length(1), // Author badge
    ])
    .split(inner);

    // Reference header
    let header = Paragraph::new(Line::from(vec![
        Span::styled(&reference, Style::default().fg(theme.accent).bold()),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Passage text
    let text_lines: Vec<Line> = verses
        .iter()
        .flat_map(|v| {
            v.text.split('\n').map(|line| {
                Line::from(Span::styled(line, Style::default().fg(theme.text)))
            }).collect::<Vec<_>>()
        })
        .collect();

    let text = Paragraph::new(text_lines).wrap(Wrap { trim: true });
    frame.render_widget(text, chunks[1]);

    // Author badge
    let badge = Paragraph::new(Line::from(vec![Span::styled(
        format!(" — {} ", first.translation),
        Style::default().fg(theme.text_muted),
    )]))
    .alignment(Alignment::Right);
    frame.render_widget(badge, chunks[2]);
}
