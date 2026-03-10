#![allow(dead_code)]

mod api;
mod app;
mod cli;
mod data;
mod store;
mod ui;
mod update;

use crate::api::Resolver;
use crate::data::reference;

use crate::ui::theme;
use crate::ui::verse_card;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // Launch interactive TUI mode
            run_tui(!cli.no_banner).await?;
        }
        Some(Commands::Read {
            reference: ref_parts,
            lang,
        }) => {
            cmd_read(&ref_parts.join(" "), &lang)?;
        }
        Some(Commands::Search { query, lang }) => {
            cmd_search(&query.join(" "), &lang)?;
        }
        Some(Commands::Random { lang }) => {
            cmd_random(&lang)?;
        }
        Some(Commands::Daily { lang }) => {
            cmd_daily(&lang)?;
        }
        Some(Commands::Intro) => {
            run_intro().await?;
        }
        Some(Commands::Update { check }) => {
            if let Err(e) = update::run_update(check).await {
                eprintln!("Update failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

async fn run_tui(show_banner: bool) -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    let result = app::App::new(show_banner).run(terminal).await;
    ratatui::restore();
    result?;
    Ok(())
}

async fn run_intro() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    let result = app::App::new(true).run(terminal).await;
    ratatui::restore();
    result?;
    Ok(())
}

fn load_theme() -> theme::Theme {
    let saved = store::state::load();
    theme::get_theme(saved.theme)
}

fn cmd_read(ref_str: &str, lang: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed = reference::parse(ref_str).map_err(|e| format!("Parse error: {}", e))?;
    let resolver = Resolver::new();

    let is_tty = std::io::IsTerminal::is_terminal(&std::io::stdout());

    match (parsed.section_start, parsed.section_end) {
        (Some(start), Some(end)) if start == end => {
            // Single section
            let verse = resolver
                .get_verse_lang(parsed.work.id, parsed.division, start, lang)
                .map_err(|e| format!("Error: {}", e))?;

            if is_tty {
                render_verse_tui(&[verse])?;
            } else {
                println!("{}", format_verse_plain(&verse));
            }
        }
        (Some(start), Some(end)) => {
            // Section range
            let verses = resolver
                .get_verse_range_lang(parsed.work.id, parsed.division, start, end, lang)
                .map_err(|e| format!("Error: {}", e))?;

            if is_tty {
                render_verse_tui(&verses)?;
            } else {
                for v in &verses {
                    println!("{}", format_verse_plain(v));
                }
            }
        }
        _ => {
            // Whole division (book/letter)
            let chapter = resolver
                .get_chapter_lang(parsed.work.id, parsed.division, lang)
                .map_err(|e| format!("Error: {}", e))?;

            if is_tty {
                render_verse_tui(&chapter.verses)?;
            } else {
                for v in &chapter.verses {
                    println!("{}", format_verse_plain(v));
                }
            }
        }
    }

    Ok(())
}

fn cmd_search(query: &str, lang: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resolver = Resolver::new();
    let results = resolver
        .search_lang(query, lang)
        .map_err(|e| format!("Search error: {}", e))?;

    if results.is_empty() {
        println!("No results found for \"{}\"", query);
        return Ok(());
    }

    println!(
        "Found {} results for \"{}\":\n",
        results.len(),
        query
    );
    for r in &results {
        println!(
            "  {} — {} {}:{} — {}",
            r.translation, // author
            r.book,
            r.chapter,
            r.verse,
            truncate_text(&r.text, 80)
        );
    }

    Ok(())
}

fn cmd_random(lang: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resolver = Resolver::new();
    let verse = resolver.get_random_verse_lang(lang);

    let is_tty = std::io::IsTerminal::is_terminal(&std::io::stdout());
    if is_tty {
        render_verse_tui(&[verse])?;
    } else {
        println!("{}", format_verse_plain(&verse));
    }

    Ok(())
}

fn cmd_daily(lang: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resolver = Resolver::new();
    let verse = resolver.get_daily_verse_lang(lang);

    let is_tty = std::io::IsTerminal::is_terminal(&std::io::stdout());
    if is_tty {
        render_verse_tui(&[verse])?;
    } else {
        println!("{}", format_verse_plain(&verse));
    }

    Ok(())
}

fn render_verse_tui(
    verses: &[api::types::Verse],
) -> Result<(), Box<dyn std::error::Error>> {
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};

    let current_theme = load_theme();
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let block = ratatui::widgets::Block::default()
                .style(ratatui::style::Style::default().bg(current_theme.bg));
            frame.render_widget(block, area);

            // Center the verse card
            let card_height = (verses.len() as u16 * 3 + 6).min(area.height - 4);
            let card_width = (area.width - 8).min(80);
            let card_area = ratatui::layout::Rect {
                x: (area.width - card_width) / 2,
                y: (area.height - card_height) / 2,
                width: card_width,
                height: card_height,
            };
            verse_card::render_verse_card(frame, card_area, verses, &current_theme);

            // Hint at bottom
            let hint = ratatui::widgets::Paragraph::new(ratatui::text::Line::from(
                ratatui::text::Span::styled(
                    " Press q or Esc to exit ",
                    ratatui::style::Style::default().fg(current_theme.text_muted),
                ),
            ))
            .alignment(ratatui::layout::Alignment::Center);
            let hint_area = ratatui::layout::Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            };
            frame.render_widget(hint, hint_area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => break,
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn format_verse_plain(verse: &api::types::Verse) -> String {
    format!(
        "{} — {} {}:{}\n{}",
        verse.translation, // author
        verse.book,
        verse.chapter,
        verse.verse,
        verse.text
    )
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    let text = text.replace('\n', " ");
    let char_count = text.chars().count();
    if char_count <= max_chars {
        text
    } else {
        let truncated: String = text.chars().take(max_chars - 3).collect();
        format!("{}...", truncated)
    }
}
