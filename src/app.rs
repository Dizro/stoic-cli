use crate::api::Resolver;
use crate::data::stoics;
use crate::store::state as session;
use crate::ui::banner::{self, BannerState};
use crate::ui::browser::{self, BrowserState, SearchMode};
use crate::ui::theme::{self, ThemeName};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;
use ratatui::{DefaultTerminal, Frame};
use std::time::Duration;

enum AppMode {
    Banner(BannerState),
    Browser(Box<BrowserState>),
}

pub struct App {
    mode: AppMode,
    resolver: Resolver,
    should_quit: bool,
    quit_pending: bool,
    theme_name: ThemeName,
}

impl App {
    pub fn new(show_banner: bool) -> Self {
        let mode = if show_banner {
            AppMode::Banner(BannerState::new())
        } else {
            AppMode::Browser(Box::new(BrowserState::new()))
        };

        Self {
            mode,
            resolver: Resolver::new(),
            should_quit: false,
            quit_pending: false,
            theme_name: ThemeName::default(),
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        // Load saved session state
        let saved = session::load();
        let has_saved_session = saved.book_index > 0 || saved.chapter > 1;
        self.theme_name = saved.theme;

        // Load the chapter from the saved session
        let work_id = if has_saved_session {
            crate::data::books::WORKS
                .get(saved.book_index)
                .map(|w| w.id)
                .unwrap_or("meditations")
        } else {
            "meditations"
        };
        let division = if has_saved_session {
            saved.chapter.max(1)
        } else {
            1
        };

        let initial_chapter = self.resolver.get_chapter(work_id, division).ok();

        if let AppMode::Browser(ref mut state) = &mut self.mode {
            if has_saved_session {
                state.restore(&saved);
            }
            state.current_chapter = initial_chapter.clone();
        }

        let mut pending_initial = Some((initial_chapter, saved));

        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;

            let tick_rate = match &self.mode {
                AppMode::Banner(_) => Duration::from_millis(16),
                AppMode::Browser(_) => Duration::from_millis(50),
            };

            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            } else {
                if let AppMode::Banner(ref mut state) = self.mode {
                    state.tick();
                    if state.done {
                        let mut browser = BrowserState::new();
                        if let Some((ch, ref saved)) = pending_initial {
                            let has_saved = saved.book_index > 0 || saved.chapter > 1;
                            if has_saved {
                                browser.restore(saved);
                            }
                            browser.current_chapter = ch;
                        }
                        pending_initial = None;
                        self.mode = AppMode::Browser(Box::new(browser));
                    }
                }
            }
        }

        // Save session state on quit
        if let AppMode::Browser(ref state) = self.mode {
            let mut snapshot = state.snapshot();
            snapshot.theme = self.theme_name;
            session::save(&snapshot);
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let theme = theme::get_theme(self.theme_name);

        match &mut self.mode {
            AppMode::Banner(state) => {
                banner::render_banner(frame, area, state, &theme);
            }
            AppMode::Browser(state) => {
                browser::render_browser(
                    frame,
                    area,
                    state,
                    self.quit_pending,
                    &theme,
                    self.theme_name,
                );
            }
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        match &mut self.mode {
            AppMode::Banner(state) => {
                state.done = true;
            }
            AppMode::Browser(state) => {
                // Search mode
                if matches!(state.search, SearchMode::Active { .. }) {
                    match key {
                        KeyCode::Esc => {
                            state.search = SearchMode::Off;
                        }
                        KeyCode::Backspace => {
                            if let SearchMode::Active { query, .. } = &mut state.search {
                                query.pop();
                            }
                            self.live_search();
                        }
                        KeyCode::Char(c) => {
                            if let SearchMode::Active { query, .. } = &mut state.search {
                                query.push(c);
                            }
                            self.live_search();
                        }
                        KeyCode::Up => {
                            if let SearchMode::Active { list_state, .. } = &mut state.search {
                                let i = list_state.selected().unwrap_or(0);
                                if i > 0 {
                                    list_state.select(Some(i - 1));
                                }
                            }
                        }
                        KeyCode::Down => {
                            if let SearchMode::Active {
                                results,
                                list_state,
                                ..
                            } = &mut state.search
                            {
                                let i = list_state.selected().unwrap_or(0);
                                if i < results.len().saturating_sub(1) {
                                    list_state.select(Some(i + 1));
                                }
                            }
                        }
                        KeyCode::Enter => {
                            let target = state
                                .selected_search_result()
                                .map(|r| (r.book.clone(), r.chapter, r.verse));
                            if let Some((book, chapter, verse)) = target {
                                state.jump_to_result(&book, chapter, verse);
                                self.load_chapter();
                            }
                        }
                        _ => {}
                    }
                    return;
                }

                // Normal browser mode
                if key == KeyCode::Char('q') {
                    if self.quit_pending {
                        self.should_quit = true;
                    } else {
                        self.quit_pending = true;
                    }
                    return;
                }
                self.quit_pending = false;

                match key {
                    KeyCode::Char('/') => {
                        state.search = SearchMode::Active {
                            query: String::new(),
                            results: vec![],
                            list_state: ListState::default(),
                        };
                    }
                    KeyCode::Char('t') => {
                        self.theme_name = self.theme_name.next();
                    }
                    KeyCode::Char('v') => {
                        state.cycle_language();
                        self.load_chapter();
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        state.prev_panel();
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        let should_load = state.next_panel_or_select();
                        if should_load {
                            self.load_chapter();
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        state.move_up();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        state.move_down();
                    }
                    KeyCode::Enter => {
                        let should_load = state.select_current();
                        if should_load {
                            self.load_chapter();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Live search — all data is bundled, so always instant.
    fn live_search(&mut self) {
        let query = match &self.mode {
            AppMode::Browser(state) => match &state.search {
                SearchMode::Active { query, .. } if query.len() >= 3 => query.clone(),
                SearchMode::Active { .. } => {
                    if let AppMode::Browser(ref mut state) = self.mode {
                        if let SearchMode::Active { results, list_state, .. } = &mut state.search {
                            results.clear();
                            list_state.select(None);
                        }
                    }
                    return;
                }
                _ => return,
            },
            _ => return,
        };

        let lang = match &self.mode {
            AppMode::Browser(state) => state.lang.clone(),
            _ => "en".to_string(),
        };

        let search_results = stoics::search_lang(&query, &lang);

        if let AppMode::Browser(ref mut state) = self.mode {
            if let SearchMode::Active { query: current_query, results, list_state } = &mut state.search {
                if *current_query == query {
                    *results = search_results;
                    if !results.is_empty() {
                        list_state.select(Some(0));
                    } else {
                        list_state.select(None);
                    }
                }
            }
        }
    }

    fn load_chapter(&mut self) {
        if let AppMode::Browser(ref mut state) = self.mode {
            let work = state.selected_work();
            let division = state.selected_division;
            let lang = state.lang.clone();

            match self.resolver.get_chapter_lang(work.id, division, &lang) {
                Ok(ch) => {
                    state.current_chapter = Some(ch);
                    state.text_scroll = 0;
                    state.error = None;
                }
                Err(e) => {
                    state.error = Some(e);
                }
            }
        }
    }
}
