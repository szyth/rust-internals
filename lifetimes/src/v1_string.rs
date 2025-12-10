// Variation 1: Basic Implementation with Single Lifetime and String
// Problem: This doesn't handle trailing delimiters correctly. If the string ends with a delimiter, it should yield an empty string as the last element. Plus String takes heap allocation that is expensive

/// A struct that splits a string by a delimiter
pub struct StrSplit {
    remainder: String,
    delimiter: String,
}

impl StrSplit {
    pub fn new(haystack: String, delimiter: String) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl Iterator for StrSplit {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(delimiter) = self.remainder.find(&self.delimiter) {
            let hsclone = self.remainder.clone();
            let item = &hsclone[..delimiter];
            self.remainder = self.remainder[(delimiter + self.delimiter.len())..].to_string();
            return Some(item.to_string());
        }
        if !self.remainder.is_empty() {
            let item = self.remainder.clone();
            self.remainder = "".to_string();
            return Some(item);
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_char_ending_with_delimiter() {
        let mut str_split = StrSplit::new("A ".to_string(), " ".to_string());
        assert_eq!(str_split.next(), Some("A".to_string()))
    }
    #[test]
    fn test_single_char() {
        let mut str_split = StrSplit::new("A".to_string(), " ".to_string());
        assert_eq!(str_split.next(), Some("A".to_string()))
    }

    #[test]
    fn test_series() {
        let haystack = "A B C D".to_string();
        let letters: Vec<_> = StrSplit::new(haystack, " ".to_string()).collect();
        assert_eq!(letters, vec!["A", "B", "C", "D"])
    }
}
