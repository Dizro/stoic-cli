pub mod types;

use crate::data::stoics;
use types::{Chapter, SearchResult, Verse};

/// Offline-only resolver for the stoic library.
/// All data is bundled in the binary — no network requests needed.
pub struct Resolver;

impl Resolver {
    pub fn new() -> Self {
        Self
    }

    pub fn get_verse_lang(
        &self,
        work_id: &str,
        division: u32,
        section: u32,
        lang: &str,
    ) -> Result<Verse, String> {
        stoics::get_verse_lang(work_id, division, section, lang)
            .ok_or_else(|| format!("Passage not found: {} {}:{}", work_id, division, section))
    }

    #[allow(dead_code)]
    pub fn get_chapter(
        &self,
        work_id: &str,
        division: u32,
    ) -> Result<Chapter, String> {
        self.get_chapter_lang(work_id, division, "en")
    }

    pub fn get_chapter_lang(
        &self,
        work_id: &str,
        division: u32,
        lang: &str,
    ) -> Result<Chapter, String> {
        stoics::get_chapter_lang(work_id, division, lang)
            .ok_or_else(|| format!("Section not found: {} {}", work_id, division))
    }

    pub fn get_verse_range_lang(
        &self,
        work_id: &str,
        division: u32,
        section_start: u32,
        section_end: u32,
        lang: &str,
    ) -> Result<Vec<Verse>, String> {
        let chapter = self.get_chapter_lang(work_id, division, lang)?;
        let verses: Vec<Verse> = chapter
            .verses
            .into_iter()
            .filter(|v| v.verse >= section_start && v.verse <= section_end)
            .collect();
        if verses.is_empty() {
            Err(format!("No passages found in range {}:{}-{}", work_id, section_start, section_end))
        } else {
            Ok(verses)
        }
    }

    pub fn search_lang(
        &self,
        query: &str,
        lang: &str,
    ) -> Result<Vec<SearchResult>, String> {
        Ok(stoics::search_lang(query, lang))
    }

    pub fn get_random_verse_lang(&self, lang: &str) -> Verse {
        stoics::random_verse_lang(lang)
    }

    pub fn get_daily_verse_lang(&self, lang: &str) -> Verse {
        stoics::daily_verse_lang(lang)
    }
}
