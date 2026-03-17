use crate::api::types::{Chapter, SearchResult};
use crate::data::books::WORKS;
use crate::data::stoics;
use crate::ui::theme::{Theme, ThemeName};
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph,
        Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Works,
    Sections,
    Text,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchMode {
    Off,
    Active {
        query: String,
        results: Vec<SearchResult>,
        list_state: ListState,
    },
}

pub struct BrowserState {
    pub active_panel: Panel,
    pub work_list: ListState,
    pub section_list: ListState,
    pub text_scroll: u16,
    pub selected_work_idx: usize,
    pub selected_division: u32,
    pub current_chapter: Option<Chapter>,
    pub loading: bool,
    pub search: SearchMode,
    /// Current divisions list for the selected work
    pub divisions: Vec<(u32, String)>,
    /// Verse to highlight after jumping from search results.
    pub highlight_verse: Option<u32>,
    /// Error message to display in the text panel.
    pub error: Option<String>,
    /// Current language code ("en", "ru", "fr", "de", "la", "el")
    pub lang: String,
    /// Index in LANGUAGES array
    pub lang_idx: usize,
    /// True when selected work has no data in the current language
    pub lang_not_available: bool,
}

impl BrowserState {
    pub fn new() -> Self {
        let mut work_list = ListState::default();
        work_list.select(Some(0));
        let mut section_list = ListState::default();
        section_list.select(Some(0));

        let divisions = stoics::get_divisions_lang(WORKS[0].id, "en");

        Self {
            active_panel: Panel::Works,
            work_list,
            section_list,
            text_scroll: 0,
            selected_work_idx: 0,
            selected_division: if divisions.is_empty() { 1 } else { divisions[0].0 },
            current_chapter: None,
            loading: false,
            search: SearchMode::Off,
            divisions,
            highlight_verse: None,
            error: None,
            lang: "en".to_string(),
            lang_idx: 0,
            lang_not_available: false,
        }
    }

    /// Restore from a saved session state.
    pub fn restore(&mut self, saved: &crate::store::state::SessionState) {
        // Restore language first — needed for localised division names
        if let Some(idx) = stoics::LANGUAGES.iter().position(|l| l.code == saved.lang.as_str()) {
            self.lang_idx = idx;
            self.lang = saved.lang.clone();
        }

        let work_idx = saved.book_index.min(WORKS.len() - 1);
        self.selected_work_idx = work_idx;
        self.work_list.select(Some(work_idx));

        self.divisions = stoics::get_divisions_lang(WORKS[work_idx].id, &self.lang);
        self.lang_not_available = self.divisions.is_empty();

        if !self.divisions.is_empty() {
            let div_idx = self.divisions.iter()
                .position(|(d, _)| *d == saved.chapter)
                .unwrap_or(0);
            self.selected_division = self.divisions[div_idx].0;
            self.section_list.select(Some(div_idx));
        }

        self.text_scroll = saved.scroll_position;
        self.active_panel = match saved.active_panel {
            0 => Panel::Works,
            1 => Panel::Sections,
            _ => Panel::Text,
        };
    }

    /// Snapshot current state for persistence.
    pub fn snapshot(&self) -> crate::store::state::SessionState {
        crate::store::state::SessionState {
            book_index: self.selected_work_idx,
            chapter: self.selected_division,
            scroll_position: self.text_scroll,
            active_panel: match self.active_panel {
                Panel::Works => 0,
                Panel::Sections => 1,
                Panel::Text => 2,
            },
            lang: self.lang.clone(),
            ..Default::default()
        }
    }

    pub fn selected_work(&self) -> &'static crate::data::books::WorkInfo {
        &WORKS[self.selected_work_idx]
    }

    pub fn update_divisions(&mut self) {
        self.divisions = stoics::get_divisions_lang(WORKS[self.selected_work_idx].id, &self.lang);
        self.lang_not_available = self.divisions.is_empty();
        self.section_list.select(Some(0));
        if !self.divisions.is_empty() {
            self.selected_division = self.divisions[0].0;
        }
    }

    /// Cycle to the next language.
    pub fn cycle_language(&mut self) {
        self.lang_idx = (self.lang_idx + 1) % stoics::LANGUAGES.len();
        self.lang = stoics::LANGUAGES[self.lang_idx].code.to_string();
        self.update_divisions();
    }

    pub fn lang_label(&self) -> &'static str {
        stoics::LANGUAGES[self.lang_idx].native
    }

    /// Move to the next panel (right arrow).
    pub fn next_panel_or_select(&mut self) -> bool {
        match self.active_panel {
            Panel::Works => {
                self.update_divisions();
                self.active_panel = Panel::Sections;
                false
            }
            Panel::Sections => {
                let idx = self.section_list.selected().unwrap_or(0);
                if idx < self.divisions.len() {
                    self.selected_division = self.divisions[idx].0;
                }
                self.text_scroll = 0;
                self.active_panel = Panel::Text;
                true // Signal to load chapter
            }
            Panel::Text => false,
        }
    }

    pub fn prev_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Works => Panel::Works,
            Panel::Sections => Panel::Works,
            Panel::Text => Panel::Sections,
        };
    }

    pub fn move_up(&mut self) {
        match self.active_panel {
            Panel::Works => {
                let i = self.work_list.selected().unwrap_or(0);
                if i > 0 {
                    self.work_list.select(Some(i - 1));
                    self.selected_work_idx = i - 1;
                }
            }
            Panel::Sections => {
                let i = self.section_list.selected().unwrap_or(0);
                if i > 0 {
                    self.section_list.select(Some(i - 1));
                }
            }
            Panel::Text => {
                self.highlight_verse = None;
                if self.text_scroll > 0 {
                    self.text_scroll -= 1;
                }
            }
        }
    }

    pub fn move_down(&mut self) {
        match self.active_panel {
            Panel::Works => {
                let i = self.work_list.selected().unwrap_or(0);
                if i < WORKS.len() - 1 {
                    self.work_list.select(Some(i + 1));
                    self.selected_work_idx = i + 1;
                }
            }
            Panel::Sections => {
                let i = self.section_list.selected().unwrap_or(0);
                if i < self.divisions.len().saturating_sub(1) {
                    self.section_list.select(Some(i + 1));
                }
            }
            Panel::Text => {
                self.highlight_verse = None;
                self.text_scroll += 1;
            }
        }
    }

    pub fn select_current(&mut self) -> bool {
        match self.active_panel {
            Panel::Works => {
                self.update_divisions();
                self.active_panel = Panel::Sections;
                false
            }
            Panel::Sections => {
                let idx = self.section_list.selected().unwrap_or(0);
                if idx < self.divisions.len() {
                    self.selected_division = self.divisions[idx].0;
                }
                self.text_scroll = 0;
                self.active_panel = Panel::Text;
                true
            }
            Panel::Text => false,
        }
    }

    /// Get the selected search result.
    pub fn selected_search_result(&self) -> Option<&SearchResult> {
        if let SearchMode::Active { results, list_state, .. } = &self.search {
            let idx = list_state.selected()?;
            results.get(idx)
        } else {
            None
        }
    }

    /// Navigate to a work and section from a search result.
    pub fn jump_to_result(&mut self, work_name: &str, division: u32, section: u32) {
        // Find the work index by matching the work name
        if let Some(idx) = WORKS.iter().position(|w| {
            work_name.contains(w.name) || work_name.to_lowercase().contains(&w.name.to_lowercase())
        }) {
            self.selected_work_idx = idx;
            self.work_list.select(Some(idx));
            self.update_divisions();

            // Find the division in the divisions list
            if let Some(div_idx) = self.divisions.iter().position(|(d, _)| *d == division) {
                self.section_list.select(Some(div_idx));
            }

            self.selected_division = division;
            self.text_scroll = 0;
            self.active_panel = Panel::Text;
            self.search = SearchMode::Off;
            self.highlight_verse = Some(section);
        }
    }
}

pub fn render_browser(
    frame: &mut Frame,
    area: Rect,
    state: &mut BrowserState,
    quit_pending: bool,
    theme: &Theme,
    theme_name: ThemeName,
) {
    // Outer border
    let outer_block = Block::default()
        .title(Line::from(vec![
            Span::styled(" stoic", Style::default().fg(theme.accent).bold()),
            Span::styled("-cli ", Style::default().fg(theme.text_dim)),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    // Layout: main content + optional search bar + status bar
    let has_search_input = matches!(state.search, SearchMode::Active { .. });
    let main_and_status = if has_search_input {
        Layout::vertical([
            Constraint::Min(1),    // Main content
            Constraint::Length(3), // Search input
            Constraint::Length(1), // Status bar
        ])
        .split(inner)
    } else {
        Layout::vertical([
            Constraint::Min(1),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(inner)
    };

    // Three panels
    let panels = Layout::horizontal([
        Constraint::Percentage(20), // Works
        Constraint::Percentage(20), // Sections/Books/Letters
        Constraint::Percentage(60), // Text
    ])
    .split(main_and_status[0]);

    render_works_panel(frame, panels[0], state, theme);
    render_sections_panel(frame, panels[1], state, theme);

    if has_search_input {
        render_search_results_panel(frame, panels[2], state, theme);
        render_search_input(frame, main_and_status[1], state, theme);
        render_status_bar(frame, main_and_status[2], theme, theme_name, state.lang_label());
    } else {
        render_text_panel(frame, panels[2], state, theme);
        render_status_bar(frame, main_and_status[1], theme, theme_name, state.lang_label());
    }

    // Quit confirmation popup
    if quit_pending {
        render_quit_popup(frame, area, theme);
    }
}

fn panel_border_style(active: bool, theme: &Theme) -> Style {
    if active {
        Style::default().fg(theme.border_active)
    } else {
        Style::default().fg(theme.border)
    }
}

fn render_works_panel(frame: &mut Frame, area: Rect, state: &mut BrowserState, theme: &Theme) {
    let is_active = state.active_panel == Panel::Works && matches!(state.search, SearchMode::Off);
    let block = Block::default()
        .title(Span::styled(
            " Works ",
            Style::default()
                .fg(if is_active { theme.accent } else { theme.text_dim })
                .bold(),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(panel_border_style(is_active, theme))
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(theme.surface));

    let items: Vec<ListItem> = WORKS
        .iter()
        .enumerate()
        .flat_map(|(i, work)| {
            let style = if Some(i) == state.work_list.selected() {
                Style::default()
                    .fg(theme.accent)
                    .bg(theme.highlight_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text)
            };
            let author_style = if Some(i) == state.work_list.selected() {
                Style::default()
                    .fg(theme.accent_soft)
                    .bg(theme.highlight_bg)
            } else {
                Style::default().fg(theme.text_dim)
            };

            vec![
                ListItem::new(Line::from(Span::styled(work.name, style))),
                ListItem::new(Line::from(Span::styled(
                    format!("  {}", work.author),
                    author_style,
                ))),
                ListItem::new(Line::default()), // spacer
            ]
        })
        .collect();

    let list = List::new(items).block(block).highlight_symbol(" ▸ ");

    // We need to map UI selection to our triple-line layout
    frame.render_widget(list, area);
}

fn render_sections_panel(frame: &mut Frame, area: Rect, state: &mut BrowserState, theme: &Theme) {
    let is_active = state.active_panel == Panel::Sections && matches!(state.search, SearchMode::Off);
    let work = state.selected_work();
    let section_label = work.section_label;

    let block = Block::default()
        .title(Span::styled(
            format!(" {} ", section_label),
            Style::default()
                .fg(if is_active { theme.accent } else { theme.text_dim })
                .bold(),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(panel_border_style(is_active, theme))
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(theme.surface));

    // Show message if this work is not available in the selected language
    if state.lang_not_available {
        let msg = Paragraph::new(vec![
            Line::default(),
            Line::from(Span::styled(
                "⚠  Not available",
                Style::default().fg(theme.text_muted),
            )),
            Line::from(Span::styled(
                "in this language",
                Style::default().fg(theme.text_muted),
            )),
            Line::default(),
            Line::from(Span::styled(
                "Press v",
                Style::default().fg(theme.accent_soft).bold(),
            )),
        ])
        .block(block)
        .alignment(Alignment::Center);
        frame.render_widget(msg, area);
        return;
    }

    let max_name_width = (area.width as usize).saturating_sub(7);

    let items: Vec<ListItem> = state.divisions
        .iter()
        .enumerate()
        .map(|(i, (_num, label))| {
            let is_selected = Some(i) == state.section_list.selected();
            let style = if is_selected {
                Style::default()
                    .fg(theme.accent)
                    .bg(theme.highlight_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text)
            };
            let name = truncate_display_name(label, max_name_width);
            ListItem::new(Span::styled(name, style))
        })
        .collect();

    let list = List::new(items).block(block).highlight_symbol(" ▸ ");

    frame.render_stateful_widget(list, area, &mut state.section_list);
}

fn render_text_panel(frame: &mut Frame, area: Rect, state: &mut BrowserState, theme: &Theme) {
    let is_active = state.active_panel == Panel::Text && matches!(state.search, SearchMode::Off);

    let work = state.selected_work();
    let title = if let Some(ref ch) = state.current_chapter {
        let label = match work.id {
            "letters" => stoics::localized_letter_label(&state.lang),
            _ => stoics::localized_book_label(&state.lang),
        };
        let count = ch.verses.len();
        format!(" {} — {} {}  ·  {} § ", work.author, label, state.selected_division, count)
    } else {
        " Text ".to_string()
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(if is_active { theme.accent } else { theme.text_dim })
                .bold(),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(panel_border_style(is_active, theme))
        .padding(Padding::new(2, 2, 1, 1))
        .style(Style::default().bg(theme.surface));

    if state.loading {
        let loading = Paragraph::new(Line::from(Span::styled(
            "Loading...",
            Style::default().fg(theme.text_dim),
        )))
        .block(block)
        .alignment(Alignment::Center);
        frame.render_widget(loading, area);
        return;
    }

    if let Some(ref err) = state.error {
        let error_msg = Paragraph::new(vec![
            Line::default(),
            Line::from(Span::styled(
                format!("Error: {}", err),
                Style::default().fg(theme.search_match),
            )),
            Line::default(),
            Line::from(Span::styled(
                "Press Enter to retry",
                Style::default().fg(theme.text_dim),
            )),
        ])
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
        frame.render_widget(error_msg, area);
        return;
    }

    if let Some(ref chapter) = state.current_chapter {
        let highlight = state.highlight_verse;
        let lines: Vec<Line> = chapter
            .verses
            .iter()
            .flat_map(|v| {
                let is_highlighted = highlight == Some(v.verse);

                // Build text lines — split by newlines to properly render multi-paragraph passages
                let mut verse_lines = Vec::new();

                // Section number header
                verse_lines.push(Line::from(Span::styled(
                    format!("§{}", v.verse),
                    if is_highlighted {
                        Style::default().fg(theme.search_match).bold()
                    } else {
                        Style::default().fg(theme.accent_soft).bold()
                    },
                )));
                verse_lines.push(Line::default());

                // Text content
                for text_line in v.text.split('\n') {
                    if text_line.is_empty() {
                        verse_lines.push(Line::default());
                    } else {
                        verse_lines.push(Line::from(Span::styled(
                            text_line,
                            if is_highlighted {
                                Style::default().fg(theme.search_match)
                            } else {
                                Style::default().fg(theme.text)
                            },
                        )));
                    }
                }

                // Separator between sections
                verse_lines.push(Line::default());
                verse_lines.push(Line::from(Span::styled(
                    "─".repeat(40),
                    Style::default().fg(theme.border),
                )));
                verse_lines.push(Line::default());

                verse_lines
            })
            .collect();

        let inner = block.inner(area);
        let visible_height = inner.height;
        let wrap_width = inner.width as usize;

        let line_heights: Vec<u16> = lines
            .iter()
            .map(|line| {
                if line.spans.is_empty() {
                    return 1;
                }
                let line_width: usize = line.spans.iter().map(|s| s.content.len()).sum();
                if wrap_width == 0 {
                    1
                } else {
                    ((line_width as f64 / wrap_width as f64).ceil() as u16).max(1)
                }
            })
            .collect();

        let content_height: u16 = line_heights.iter().sum();

        // Auto-scroll to highlighted verse
        if let Some(_target_verse) = highlight {
            // Find the line index for the target verse header
            let mut current_line = 0u16;
            for (i, line) in lines.iter().enumerate() {
                if let Some(span) = line.spans.first() {
                    if span.content.starts_with('§') {
                        if let Some(num_str) = span.content.strip_prefix('§') {
                            if let Ok(num) = num_str.parse::<u32>() {
                                if Some(num) == highlight {
                                    let center_offset = visible_height / 3;
                                    state.text_scroll = current_line.saturating_sub(center_offset);
                                    break;
                                }
                            }
                        }
                    }
                }
                current_line += line_heights.get(i).copied().unwrap_or(1);
            }
        }

        // Clamp scroll
        if content_height > visible_height {
            let max_scroll = content_height - visible_height;
            if state.text_scroll > max_scroll {
                state.text_scroll = max_scroll;
            }
        } else {
            state.text_scroll = 0;
        }

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((state.text_scroll, 0));

        frame.render_widget(paragraph, area);

        // Scrollbar
        if content_height > visible_height {
            let max_scroll = (content_height - visible_height) as usize;
            let mut scrollbar_state = ScrollbarState::new(max_scroll)
                .position(state.text_scroll as usize);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(theme.border));
            frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
        }
    } else {
        let hint_lines = if state.lang_not_available {
            vec![
                Line::default(),
                Line::default(),
                Line::from(Span::styled(
                    "⚠  Not available in this language",
                    Style::default().fg(theme.text_dim),
                )),
                Line::default(),
                Line::from(Span::styled(
                    "Press v to switch language",
                    Style::default().fg(theme.accent_soft),
                )),
            ]
        } else {
            vec![
                Line::default(),
                Line::default(),
                Line::from(Span::styled(
                    "Select a work and section to begin reading",
                    Style::default().fg(theme.text_dim),
                )),
                Line::default(),
                Line::from(Span::styled(
                    "Use arrow keys to navigate, Enter to select",
                    Style::default().fg(theme.text_muted),
                )),
                Line::default(),
                Line::from(Span::styled(
                    "\"The happiness of your life depends upon",
                    Style::default().fg(theme.text_muted).italic(),
                )),
                Line::from(Span::styled(
                    "  the quality of your thoughts.\"",
                    Style::default().fg(theme.text_muted).italic(),
                )),
                Line::from(Span::styled(
                    "                  — Marcus Aurelius",
                    Style::default().fg(theme.accent_soft),
                )),
            ]
        };
        let hint = Paragraph::new(hint_lines)
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(hint, area);
    }
}

fn render_search_results_panel(
    frame: &mut Frame,
    area: Rect,
    state: &mut BrowserState,
    theme: &Theme,
) {
    let (query, results, list_state) = match &mut state.search {
        SearchMode::Active { query, results, list_state } => (query.clone(), results, list_state),
        _ => return,
    };

    let title = format!(" Search: \"{}\" ({} results) ", query, results.len());

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(theme.accent).bold(),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_active))
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(theme.surface));

    if results.is_empty() {
        let msg = if query.len() < 3 {
            "Type at least 3 characters to search"
        } else {
            "No results found"
        };
        let empty = Paragraph::new(vec![
            Line::default(),
            Line::default(),
            Line::from(Span::styled(
                msg,
                Style::default().fg(theme.text_dim),
            )),
            Line::default(),
            Line::from(Span::styled(
                "Press Esc to go back",
                Style::default().fg(theme.text_muted),
            )),
        ])
        .block(block)
        .alignment(Alignment::Center);
        frame.render_widget(empty, area);
        return;
    }

    let query_lower = query.to_lowercase();
    let items: Vec<ListItem> = results
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let is_selected = Some(i) == list_state.selected();
            let ref_style = if is_selected {
                Style::default()
                    .fg(theme.accent)
                    .bg(theme.highlight_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.accent_soft).bold()
            };
            let text_style = if is_selected {
                Style::default().fg(theme.text).bg(theme.highlight_bg)
            } else {
                Style::default().fg(theme.text_dim)
            };

            let ref_str = format!("{} — {}", r.translation, r.reference());
            let text = truncate_result_text(&r.text, 60);

            let mut spans = vec![
                Span::styled(ref_str, ref_style),
                Span::styled("  ", text_style),
            ];

            // Simple highlight
            let text_chars: Vec<char> = text.chars().collect();
            let query_chars: Vec<char> = query_lower.chars().collect();
            let text_lower_chars: Vec<char> = text.to_lowercase().chars().collect();

            let match_pos = text_lower_chars
                .windows(query_chars.len())
                .position(|w| w == query_chars.as_slice());

            if let Some(pos) = match_pos {
                let before: String = text_chars[..pos].iter().collect();
                let matched: String = text_chars[pos..pos + query_chars.len()].iter().collect();
                let after: String = text_chars[pos + query_chars.len()..].iter().collect();
                spans.push(Span::styled(before, text_style));
                spans.push(Span::styled(
                    matched,
                    Style::default().fg(theme.search_match).add_modifier(Modifier::BOLD),
                ));
                spans.push(Span::styled(after, text_style));
            } else {
                spans.push(Span::styled(text, text_style));
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items).block(block).highlight_symbol("  ");
    frame.render_stateful_widget(list, area, list_state);
}

fn render_search_input(frame: &mut Frame, area: Rect, state: &BrowserState, theme: &Theme) {
    let query = match &state.search {
        SearchMode::Active { query, .. } => query.as_str(),
        _ => return,
    };

    let block = Block::default()
        .title(Span::styled(" / Search ", Style::default().fg(theme.accent).bold()))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_active))
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(theme.surface));

    let cursor = "\u{2588}";
    let spans = vec![
        Span::styled(query, Style::default().fg(theme.text)),
        Span::styled(cursor, Style::default().fg(theme.accent_soft)),
    ];

    let input = Paragraph::new(Line::from(spans)).block(block);
    frame.render_widget(input, area);
}

fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    theme_name: ThemeName,
    lang_label: &str,
) {
    let keybinds = [("\u{2190}\u{2192}/hl", "panels"),
        ("\u{2191}\u{2193}/jk", "navigate"),
        ("Enter", "select"),
        ("/", "search"),
        ("t", theme_name.label()),
        ("v", lang_label),
        ("qq", "quit")];

    let spans: Vec<Span> = keybinds
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(
                    format!(" {} ", key),
                    Style::default().fg(theme.accent_soft).bold(),
                ),
                Span::styled(
                    format!("{} ", desc),
                    Style::default().fg(theme.text_muted),
                ),
                Span::styled("  ", Style::default()),
            ]
        })
        .collect();

    let bar = Paragraph::new(Line::from(spans)).style(Style::default().bg(theme.bg));
    frame.render_widget(bar, area);
}

fn truncate_display_name(name: &str, max_width: usize) -> String {
    use unicode_width::UnicodeWidthStr;
    let w = name.width();
    if w <= max_width {
        return name.to_string();
    }
    let target = max_width.saturating_sub(1);
    let mut truncated = String::new();
    let mut current_w = 0;
    for ch in name.chars() {
        let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if current_w + cw > target {
            break;
        }
        truncated.push(ch);
        current_w += cw;
    }
    truncated.push('\u{2026}');
    truncated
}

fn render_quit_popup(frame: &mut Frame, area: Rect, theme: &Theme) {
    let popup_width = 32u16;
    let popup_height = 3u16;

    let horizontal = Layout::horizontal([Constraint::Length(popup_width)])
        .flex(Flex::Center)
        .split(area);
    let vertical = Layout::vertical([Constraint::Length(popup_height)])
        .flex(Flex::Center)
        .split(horizontal[0]);
    let popup_area = vertical[0];

    frame.render_widget(Clear, popup_area);

    let popup = Paragraph::new(Line::from(vec![
        Span::styled("  Press ", Style::default().fg(theme.text_dim)),
        Span::styled("q", Style::default().fg(theme.accent).bold()),
        Span::styled(" again to quit  ", Style::default().fg(theme.text_dim)),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border_active))
            .style(Style::default().bg(theme.surface)),
    );

    frame.render_widget(popup, popup_area);
}

fn truncate_result_text(text: &str, max_chars: usize) -> String {
    // Remove newlines for display in search results
    let text = text.replace('\n', " ");
    let char_count = text.chars().count();
    if char_count <= max_chars {
        text
    } else {
        let truncated: String = text.chars().take(max_chars - 3).collect();
        format!("{}...", truncated)
    }
}
