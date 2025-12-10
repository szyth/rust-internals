// Variation 4. Multple lifetimes
// Separating the lifetime of the haystack from the delimiter.
// Key improvement: Now the delimiter can have a shorter lifetime than the haystack. The `until_char` function works because the temporary `String` from `format!` only needs to live during the `new` call, while the returned slice references the original haystack.

pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack> Iterator for StrSplit<'haystack, '_> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        // Using ref mut to get a mutable reference to the value inside Option

        if let Some(ref mut remainder) = self.remainder {
            if let Some(delimiter) = remainder.find(&self.delimiter) {
                let item = &remainder[..delimiter];
                *remainder = &remainder[(delimiter + self.delimiter.len())..];
                return Some(item);
            } else {
                // Take the remainder, leaving None in its place
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

/// Helper function that returns the string until the first occurrence of a character
fn until_char(s: &str, c: char) -> &str {
    let char_str = format!("{}", c);
    StrSplit::new(s, &char_str)
        .next()
        .expect("StrSplit always gives at least on result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello world", 'l'), "he")
    }
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
