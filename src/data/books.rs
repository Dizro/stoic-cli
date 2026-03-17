/// Stoic works and their structure.
/// Each work has a unique ID, display name, author, and section structure.
#[derive(Debug, Clone)]
pub struct WorkInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub author: &'static str,
    pub abbrevs: &'static [&'static str],
    /// Number of top-level divisions (books for Meditations/Discourses, letters for Seneca)
    #[allow(dead_code)]
    pub sections: u32,
    /// Label for top-level divisions
    pub section_label: &'static str,
}

pub static WORKS: &[WorkInfo] = &[
    WorkInfo {
        id: "meditations",
        name: "Meditations",
        author: "Marcus Aurelius",
        abbrevs: &["med", "aurelius", "marcus", "ma"],
        sections: 12,
        section_label: "Book",
    },
    WorkInfo {
        id: "discourses",
        name: "Discourses",
        author: "Epictetus",
        abbrevs: &["disc", "epictetus", "epic", "ep"],
        sections: 4, // 4 books + preface handled separately
        section_label: "Book",
    },
    WorkInfo {
        id: "letters",
        name: "Moral Letters",
        author: "Seneca",
        abbrevs: &["letters", "seneca", "sen", "ml", "epistles"],
        sections: 124,
        section_label: "Letter",
    },
];

/// Normalize a work name input to the canonical work.
/// Handles full names, abbreviations, case insensitivity.
pub fn normalize_work(input: &str) -> Option<&'static WorkInfo> {
    let input = input.trim().to_lowercase();
    let input = input.replace('.', "");

    // Try exact full name match first
    for work in WORKS {
        if work.name.to_lowercase() == input || work.id == input {
            return Some(work);
        }
    }

    // Try author name match
    for work in WORKS {
        if work.author.to_lowercase() == input {
            return Some(work);
        }
    }

    // Try abbreviation match
    for work in WORKS {
        for abbrev in work.abbrevs {
            if *abbrev == input {
                return Some(work);
            }
        }
    }

    // Try prefix match on full name or author
    WORKS.iter().find(|&work| (work.name.to_lowercase().starts_with(&input)
            || work.author.to_lowercase().starts_with(&input))
            && input.len() >= 3).map(|v| v as _)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_name() {
        assert_eq!(normalize_work("Meditations").unwrap().id, "meditations");
        assert_eq!(normalize_work("Discourses").unwrap().id, "discourses");
        assert_eq!(normalize_work("Moral Letters").unwrap().id, "letters");
    }

    #[test]
    fn test_author_name() {
        assert_eq!(normalize_work("Marcus Aurelius").unwrap().id, "meditations");
        assert_eq!(normalize_work("Epictetus").unwrap().id, "discourses");
        assert_eq!(normalize_work("Seneca").unwrap().id, "letters");
    }

    #[test]
    fn test_abbreviation() {
        assert_eq!(normalize_work("med").unwrap().id, "meditations");
        assert_eq!(normalize_work("aurelius").unwrap().id, "meditations");
        assert_eq!(normalize_work("disc").unwrap().id, "discourses");
        assert_eq!(normalize_work("sen").unwrap().id, "letters");
    }

    #[test]
    fn test_prefix_match() {
        assert_eq!(normalize_work("marc").unwrap().id, "meditations");
        assert_eq!(normalize_work("epic").unwrap().id, "discourses");
        assert_eq!(normalize_work("sen").unwrap().id, "letters");
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(normalize_work("MEDITATIONS").unwrap().id, "meditations");
        assert_eq!(normalize_work("seneca").unwrap().id, "letters");
    }

    #[test]
    fn test_invalid() {
        assert!(normalize_work("notawork").is_none());
        assert!(normalize_work("xyz").is_none());
    }
}
