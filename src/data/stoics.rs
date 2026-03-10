use crate::api::types::{Chapter, SearchResult, Verse};
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
struct StoicWork {
    metadata: StoicMetadata,
    sections: Vec<StoicSection>,
}

#[derive(Deserialize)]
struct StoicMetadata {
    work: String,
    author: String,
    #[allow(dead_code)]
    translator: String,
}

#[derive(Deserialize)]
struct StoicSection {
    #[allow(dead_code)]
    id: String,
    book: Option<u32>,
    #[allow(dead_code)]
    book_title: Option<String>,
    chapter: Option<u32>,
    chapter_title: Option<String>,
    section: Option<u32>,
    letter_number: Option<u32>,
    #[serde(default)]
    letter_title: Option<String>,
    text: String,
}

// ──────────────────────────────────────────────────────
// Bundled texts — all embedded at compile time
// ──────────────────────────────────────────────────────

// English (full)
const EN_MEDITATIONS: &str = include_str!("../../data/en/meditations.json");
const EN_DISCOURSES: &str = include_str!("../../data/en/discourses.json");
const EN_LETTERS: &str = include_str!("../../data/en/moral_letters.json");

// Русский (mock — заполнить переводом)
const RU_MEDITATIONS: &str = include_str!("../../data/ru/meditations.json");
const RU_DISCOURSES: &str = include_str!("../../data/ru/discourses.json");
const RU_LETTERS: &str = include_str!("../../data/ru/moral_letters.json");

// Français (mock — remplir avec la traduction)
const FR_MEDITATIONS: &str = include_str!("../../data/fr/meditations.json");
const FR_DISCOURSES: &str = include_str!("../../data/fr/discourses.json");
const FR_LETTERS: &str = include_str!("../../data/fr/moral_letters.json");

// Deutsch (mock — mit Übersetzung füllen)
const DE_MEDITATIONS: &str = include_str!("../../data/de/meditations.json");
const DE_DISCOURSES: &str = include_str!("../../data/de/discourses.json");
const DE_LETTERS: &str = include_str!("../../data/de/moral_letters.json");

// Latina (originale — Seneca scripsit Latine)
const LA_LETTERS: &str = include_str!("../../data/la/moral_letters.json");

// Ἑλληνικά (τὸ πρωτότυπον — Aurelius & Epictetus)
const EL_MEDITATIONS: &str = include_str!("../../data/el/meditations.json");
const EL_DISCOURSES: &str = include_str!("../../data/el/discourses.json");

// ──────────────────────────────────────────────────────
// Language definitions
// ──────────────────────────────────────────────────────

pub struct LangInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub native: &'static str,
}

pub const LANGUAGES: &[LangInfo] = &[
    LangInfo { code: "en", name: "English", native: "English" },
    LangInfo { code: "ru", name: "Russian", native: "Русский" },
    LangInfo { code: "fr", name: "French", native: "Français" },
    LangInfo { code: "de", name: "German", native: "Deutsch" },
    LangInfo { code: "la", name: "Latin", native: "Latina" },
    LangInfo { code: "el", name: "Greek", native: "Ἑλληνικά" },
];

// ──────────────────────────────────────────────────────
// Parsed library per language
// ──────────────────────────────────────────────────────

struct LangLibrary {
    meditations: StoicWork,
    discourses: StoicWork,
    letters: StoicWork,
}

struct MultiLangLibrary {
    en: LangLibrary,
    ru: LangLibrary,
    fr: LangLibrary,
    de: LangLibrary,
    la: LangLibrary,
    el: LangLibrary,
}

fn parse_work(data: &str, fallback_name: &str) -> StoicWork {
    serde_json::from_str(data).unwrap_or_else(|_| StoicWork {
        metadata: StoicMetadata {
            work: fallback_name.to_string(),
            author: String::new(),
            translator: String::new(),
        },
        sections: vec![],
    })
}

// Empty work placeholder (for languages missing a specific text)
fn empty_work(work: &str, author: &str) -> StoicWork {
    StoicWork {
        metadata: StoicMetadata {
            work: work.to_string(),
            author: author.to_string(),
            translator: String::new(),
        },
        sections: vec![],
    }
}

static LIBRARY: Lazy<MultiLangLibrary> = Lazy::new(|| {
    MultiLangLibrary {
        en: LangLibrary {
            meditations: parse_work(EN_MEDITATIONS, "Meditations"),
            discourses: parse_work(EN_DISCOURSES, "Discourses"),
            letters: parse_work(EN_LETTERS, "Moral Letters"),
        },
        ru: LangLibrary {
            meditations: parse_work(RU_MEDITATIONS, "Размышления"),
            discourses: parse_work(RU_DISCOURSES, "Беседы"),
            letters: parse_work(RU_LETTERS, "Нравственные письма"),
        },
        fr: LangLibrary {
            meditations: parse_work(FR_MEDITATIONS, "Pensées"),
            discourses: parse_work(FR_DISCOURSES, "Entretiens"),
            letters: parse_work(FR_LETTERS, "Lettres à Lucilius"),
        },
        de: LangLibrary {
            meditations: parse_work(DE_MEDITATIONS, "Selbstbetrachtungen"),
            discourses: parse_work(DE_DISCOURSES, "Unterredungen"),
            letters: parse_work(DE_LETTERS, "Briefe an Lucilius"),
        },
        la: LangLibrary {
            meditations: empty_work("Meditationes", "Marcus Aurelius"), // wrote in Greek
            discourses: empty_work("Dissertationes", "Epictetus"),
            letters: parse_work(LA_LETTERS, "Epistulae Morales"),
        },
        el: LangLibrary {
            meditations: parse_work(EL_MEDITATIONS, "Τὰ εἰς ἑαυτόν"),
            discourses: parse_work(EL_DISCOURSES, "Διατριβαί"),
            letters: empty_work("—", "Seneca"), // Seneca wrote in Latin, not Greek
        },
    }
});

fn get_lang_library(lang: &str) -> &'static LangLibrary {
    match lang {
        "ru" => &LIBRARY.ru,
        "fr" => &LIBRARY.fr,
        "de" => &LIBRARY.de,
        "la" => &LIBRARY.la,
        "el" => &LIBRARY.el,
        _ => &LIBRARY.en,
    }
}

fn get_work(work_id: &str, lang: &str) -> Option<&'static StoicWork> {
    let lib = get_lang_library(lang);
    match work_id {
        "meditations" => Some(&lib.meditations),
        "discourses" => Some(&lib.discourses),
        "letters" => Some(&lib.letters),
        _ => None,
    }
}

// ──────────────────────────────────────────────────────
// Public API (all accept optional lang parameter)
// ──────────────────────────────────────────────────────

fn section_number(s: &StoicSection) -> u32 {
    s.section.or(s.chapter).unwrap_or(0)
}

fn top_level_division(s: &StoicSection, work_id: &str) -> u32 {
    match work_id {
        "letters" => s.letter_number.unwrap_or(0),
        _ => s.book.unwrap_or(0),
    }
}

fn section_title(s: &StoicSection) -> Option<&str> {
    s.letter_title.as_deref().or(s.chapter_title.as_deref())
}

pub fn get_chapter(work_id: &str, division: u32) -> Option<Chapter> {
    get_chapter_lang(work_id, division, "en")
}

pub fn get_chapter_lang(work_id: &str, division: u32, lang: &str) -> Option<Chapter> {
    let work = get_work(work_id, lang)?;

    let sections: Vec<&StoicSection> = work
        .sections
        .iter()
        .filter(|s| top_level_division(s, work_id) == division)
        .collect();

    if sections.is_empty() {
        return None;
    }

    let verses: Vec<Verse> = sections
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let verse_num = section_number(s);
            let verse_num = if verse_num == 0 { (i + 1) as u32 } else { verse_num };
            let title_prefix = section_title(s)
                .map(|t| format!("[{}]\n\n", t))
                .unwrap_or_default();
            Verse {
                book: work.metadata.work.clone(),
                chapter: division,
                verse: verse_num,
                text: format!("{}{}", title_prefix, s.text),
                translation: work.metadata.author.clone(),
            }
        })
        .collect();

    Some(Chapter {
        book: work.metadata.work.clone(),
        chapter: division,
        verses,
        translation: work.metadata.author.clone(),
    })
}

pub fn get_section_names(work_id: &str, division: u32) -> Vec<String> {
    get_section_names_lang(work_id, division, "en")
}

pub fn get_section_names_lang(work_id: &str, division: u32, lang: &str) -> Vec<String> {
    let Some(work) = get_work(work_id, lang) else {
        return vec![];
    };

    work.sections
        .iter()
        .filter(|s| top_level_division(s, work_id) == division)
        .map(|s| {
            if let Some(title) = section_title(s) {
                title.to_string()
            } else {
                let num = section_number(s);
                format!("§{}", num)
            }
        })
        .collect()
}

pub fn get_divisions(work_id: &str) -> Vec<(u32, String)> {
    get_divisions_lang(work_id, "en")
}

pub fn localized_book_label(lang: &str) -> &'static str {
    match lang {
        "ru" => "Книга",
        "fr" => "Livre",
        "de" => "Buch",
        "la" => "Liber",
        "el" => "Βιβλίον",
        _   => "Book",
    }
}

pub fn localized_letter_label(lang: &str) -> &'static str {
    match lang {
        "ru" => "Письмо",
        "fr" => "Lettre",
        "de" => "Brief",
        "la" => "Epistula",
        _   => "Letter",
    }
}

pub fn get_divisions_lang(work_id: &str, lang: &str) -> Vec<(u32, String)> {
    let Some(work) = get_work(work_id, lang) else {
        return vec![];
    };

    let mut seen = std::collections::BTreeSet::new();
    for s in &work.sections {
        let div = top_level_division(s, work_id);
        if div > 0 {
            seen.insert(div);
        }
    }

    match work_id {
        "letters" => {
            let label = localized_letter_label(lang);
            seen
                .into_iter()
                .map(|n| {
                    let title = work
                        .sections
                        .iter()
                        .find(|s| s.letter_number == Some(n))
                        .and_then(|s| s.letter_title.as_deref())
                        .unwrap_or("");
                    if title.is_empty() {
                        (n, format!("{} {}", label, n))
                    } else {
                        (n, format!("{}. {}", n, title))
                    }
                })
                .collect()
        }
        _ => {
            let label = localized_book_label(lang);
            seen
                .into_iter()
                .map(|n| (n, format!("{} {}", label, n)))
                .collect()
        }
    }
}

pub fn get_verse(work_id: &str, division: u32, section: u32) -> Option<Verse> {
    get_verse_lang(work_id, division, section, "en")
}

pub fn get_verse_lang(work_id: &str, division: u32, section: u32, lang: &str) -> Option<Verse> {
    let work = get_work(work_id, lang)?;

    let s = work.sections.iter().find(|s| {
        top_level_division(s, work_id) == division && section_number(s) == section
    })?;

    let title_prefix = section_title(s)
        .map(|t| format!("[{}]\n\n", t))
        .unwrap_or_default();

    Some(Verse {
        book: work.metadata.work.clone(),
        chapter: division,
        verse: section,
        text: format!("{}{}", title_prefix, s.text),
        translation: work.metadata.author.clone(),
    })
}

pub fn search(query: &str) -> Vec<SearchResult> {
    search_lang(query, "en")
}

pub fn search_lang(query: &str, lang: &str) -> Vec<SearchResult> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let lib = get_lang_library(lang);

    for (work_id, work) in [
        ("meditations", &lib.meditations),
        ("discourses", &lib.discourses),
        ("letters", &lib.letters),
    ] {
        for s in &work.sections {
            if s.text.to_lowercase().contains(&query_lower) {
                let div = top_level_division(s, work_id);
                let num = section_number(s);
                results.push(SearchResult {
                    book: work.metadata.work.clone(),
                    chapter: div,
                    verse: num,
                    text: s.text.clone(),
                    translation: work.metadata.author.clone(),
                });
            }
        }
    }

    results.truncate(50);
    results
}

pub fn random_verse() -> Verse {
    random_verse_lang("en")
}

pub fn random_verse_lang(lang: &str) -> Verse {
    use std::time::{SystemTime, UNIX_EPOCH};

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;

    let lib = get_lang_library(lang);
    let all_works = [&lib.meditations, &lib.discourses, &lib.letters];
    let work_ids = ["meditations", "discourses", "letters"];

    let work_idx = seed % all_works.len();
    let work = all_works[work_idx];
    let work_id = work_ids[work_idx];

    if work.sections.is_empty() {
        let fallback_lib = get_lang_library(lang);
        let fallback_work = &fallback_lib.meditations;
        if let Some(s) = fallback_work.sections.first() {
            let div = top_level_division(s, "meditations");
            let num = section_number(s);
            return Verse {
                book: fallback_work.metadata.work.clone(),
                chapter: div,
                verse: num,
                text: s.text.clone(),
                translation: fallback_work.metadata.author.clone(),
            };
        }
        return Verse {
            book: "Meditations".to_string(),
            chapter: 1,
            verse: 1,
            text: "The happiness of your life depends upon the quality of your thoughts.".to_string(),
            translation: "Marcus Aurelius".to_string(),
        };
    }

    let section_idx = (seed / 7) % work.sections.len();
    let s = &work.sections[section_idx];
    let div = top_level_division(s, work_id);
    let num = section_number(s);

    Verse {
        book: work.metadata.work.clone(),
        chapter: div,
        verse: num,
        text: s.text.clone(),
        translation: work.metadata.author.clone(),
    }
}

pub fn daily_verse() -> Verse {
    daily_verse_lang("en")
}

pub fn daily_verse_lang(lang: &str) -> Verse {
    use std::time::{SystemTime, UNIX_EPOCH};

    let day_seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 86400;

    let lib = get_lang_library(lang);
    let all_works = [&lib.meditations, &lib.discourses, &lib.letters];
    let work_ids = ["meditations", "discourses", "letters"];

    let mut all_sections: Vec<(&StoicWork, &str, &StoicSection)> = Vec::new();
    for (i, work) in all_works.iter().enumerate() {
        for s in &work.sections {
            all_sections.push((work, work_ids[i], s));
        }
    }

    if all_sections.is_empty() {
        return random_verse_lang(lang);
    }

    let idx = (day_seed as usize) % all_sections.len();
    let (work, work_id, s) = all_sections[idx];
    let div = top_level_division(s, work_id);
    let num = section_number(s);

    Verse {
        book: work.metadata.work.clone(),
        chapter: div,
        verse: num,
        text: s.text.clone(),
        translation: work.metadata.author.clone(),
    }
}
