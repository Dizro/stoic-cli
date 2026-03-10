use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verse {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
    pub translation: String, // Used for author name in stoic context
}

impl Verse {
    pub fn reference(&self) -> String {
        format!("{} {}:{}", self.book, self.chapter, self.verse)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub book: String,
    pub chapter: u32,
    pub verses: Vec<Verse>,
    pub translation: String,
}

impl Chapter {
    pub fn reference(&self) -> String {
        format!("{} {}", self.book, self.chapter)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
    pub translation: String,
}

impl SearchResult {
    pub fn reference(&self) -> String {
        format!("{} {}:{}", self.book, self.chapter, self.verse)
    }
}
