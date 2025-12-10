// Variation 5: Generic Delimiter with Trait
// Eliminating allocations by making the delimiter generic.
// Key improvement: No allocations needed. We can pass either `&str` or `char` directly. The generic `D` parameter can be any type that implements `Delimiter`.

pub trait Delimiter {
    /// Find the next occurrence of this delimiter in the given string
    /// Returns (start_index, end_index) if found
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// Implement Delimiter for &str
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

// Implement Delimiter for char
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let item = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            return Some(item);
        } else {
            // Take the remainder, leaving None in its place
            self.remainder.take()
        }
    }
}

/// Helper function that returns the string until the first occurrence of a character
fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
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
