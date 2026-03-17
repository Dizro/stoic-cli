use crate::data::books;

/// A parsed stoic text reference.
#[derive(Debug, Clone)]
pub struct StoicReference {
    pub work: &'static books::WorkInfo,
    pub division: u32,
    pub section_start: Option<u32>,
    pub section_end: Option<u32>,
}

impl StoicReference {
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        match (self.section_start, self.section_end) {
            (Some(start), Some(end)) if start != end => {
                format!("{} {}:{}-{}", self.work.name, self.division, start, end)
            }
            (Some(start), _) => {
                format!("{} {}:{}", self.work.name, self.division, start)
            }
            _ => {
                format!("{} {}", self.work.name, self.division)
            }
        }
    }
}

/// Parse a stoic reference string into a structured reference.
///
/// Supports formats:
/// - "meditations 4:3"      -> work=Meditations, book=4, section=3
/// - "aurelius 4"           -> work=Meditations, book=4 (all sections)
/// - "seneca 13"            -> work=Moral Letters, letter=13
/// - "discourses 1:2"       -> work=Discourses, book=1, chapter=2
/// - "med 4:3"              -> abbreviation
pub fn parse(input: &str) -> Result<StoicReference, String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("Empty reference".to_string());
    }

    let (work_str, rest) = split_work_and_location(input)?;

    let work = books::normalize_work(&work_str)
        .ok_or_else(|| format!("Unknown work: '{}'. Try: meditations, discourses, seneca", work_str))?;

    if rest.is_empty() {
        return Ok(StoicReference {
            work,
            division: 1,
            section_start: None,
            section_end: None,
        });
    }

    let (division, section_start, section_end) = parse_division_section(&rest)?;

    Ok(StoicReference {
        work,
        division,
        section_start,
        section_end,
    })
}

/// Split input into (work_name, division_section_rest).
fn split_work_and_location(input: &str) -> Result<(String, String), String> {
    // Try to find where the numeric part starts
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    // Skip alphabetic characters and spaces (the work name)
    while i < len && (chars[i].is_alphabetic() || chars[i] == ' ' || chars[i] == '.') {
        i += 1;
        // Stop at space if next char is a digit
        if i < len && chars[i - 1] == ' ' && chars[i].is_ascii_digit() {
            i -= 1;
            break;
        }
    }

    let work_str = input[..i].trim().to_string();
    let rest = input[i..].trim().to_string();

    if work_str.is_empty() {
        return Err("No work name found. Try: meditations, discourses, seneca".to_string());
    }

    Ok((work_str, rest))
}

/// Parse "4:3", "4:3-5", "4" into (division, section_start, section_end).
fn parse_division_section(input: &str) -> Result<(u32, Option<u32>, Option<u32>), String> {
    let input = input.trim();

    if let Some((div_str, section_part)) = input.split_once(':') {
        let division: u32 = div_str
            .trim()
            .parse()
            .map_err(|_| format!("Invalid number: '{}'", div_str))?;

        if let Some((start_str, end_str)) = section_part.split_once('-') {
            let start: u32 = start_str
                .trim()
                .parse()
                .map_err(|_| format!("Invalid number: '{}'", start_str))?;
            let end: u32 = end_str
                .trim()
                .parse()
                .map_err(|_| format!("Invalid number: '{}'", end_str))?;
            Ok((division, Some(start), Some(end)))
        } else {
            let section: u32 = section_part
                .trim()
                .parse()
                .map_err(|_| format!("Invalid number: '{}'", section_part))?;
            Ok((division, Some(section), Some(section)))
        }
    } else {
        let division: u32 = input
            .parse()
            .map_err(|_| format!("Invalid number: '{}'", input))?;
        Ok((division, None, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meditations_reference() {
        let r = parse("meditations 4:3").unwrap();
        assert_eq!(r.work.id, "meditations");
        assert_eq!(r.division, 4);
        assert_eq!(r.section_start, Some(3));
    }

    #[test]
    fn test_aurelius_shorthand() {
        let r = parse("aurelius 4").unwrap();
        assert_eq!(r.work.id, "meditations");
        assert_eq!(r.division, 4);
        assert_eq!(r.section_start, None);
    }

    #[test]
    fn test_seneca_letter() {
        let r = parse("seneca 13").unwrap();
        assert_eq!(r.work.id, "letters");
        assert_eq!(r.division, 13);
    }

    #[test]
    fn test_discourses_chapter() {
        let r = parse("discourses 1:2").unwrap();
        assert_eq!(r.work.id, "discourses");
        assert_eq!(r.division, 1);
        assert_eq!(r.section_start, Some(2));
    }

    #[test]
    fn test_abbreviation() {
        let r = parse("med 4:3").unwrap();
        assert_eq!(r.work.id, "meditations");
        assert_eq!(r.division, 4);
        assert_eq!(r.section_start, Some(3));
    }

    #[test]
    fn test_display() {
        let r = parse("meditations 4:3").unwrap();
        assert_eq!(r.display(), "Meditations 4:3");

        let r = parse("aurelius 4").unwrap();
        assert_eq!(r.display(), "Meditations 4");
    }

    #[test]
    fn test_invalid_work() {
        assert!(parse("notawork 1:1").is_err());
    }
}
