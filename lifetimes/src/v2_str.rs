// Variation 2: Basic Implementation with Single Lifetime and &str
// Problem: This doesn't handle trailing delimiters correctly. If the string ends with a delimiter, it should yield an empty string as the last element.

pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(delimiter) = self.remainder.find(&self.delimiter) {
            let item = &self.remainder[..delimiter];
            self.remainder = &self.remainder[(delimiter + self.delimiter.len())..];
            return Some(item);
        }
        if !self.remainder.is_empty() {
            let item = self.remainder;
            self.remainder = "";
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
        let mut str_split = StrSplit::new("A ", " ");
        assert_eq!(str_split.next(), Some("A"))
    }
    #[test]
    fn test_single_char() {
        let mut str_split = StrSplit::new("A", " ");
        assert_eq!(str_split.next(), Some("A"))
    }

    #[test]
    fn test_series() {
        let haystack = "A B C D";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["A", "B", "C", "D"])
    }
}
