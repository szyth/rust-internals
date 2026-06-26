// Variation 3: Handling Trailing Delimiters with Option

pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

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

// Problem: This still has the single lifetime issue - both haystack and delimiter must have the same lifetime.
// fn until_char(s: &str, c: char) -> &str {
//     let char_str = format!("{}", c);
//     StrSplit::new(s, &char_str)
//         .next()
//         .expect("StrSplit always gives at least on result")
// }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // fn until_char_test() {
    //     assert_eq!(until_char("hello world", 'l'), "he")
    // }
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
