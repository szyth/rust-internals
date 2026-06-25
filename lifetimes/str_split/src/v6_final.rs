// Variation 6: Complete implementation with all features
// Final version with better ergonimics and additional delimiter types

//! String splitting library for education purposes
//!
//! This library provides a generic string splitting iterator that works
//! with any type that can act as a delimiter.
//!
//! Trait for types that can act as delimiters in string splitting
pub trait Delimiter {
    /// Find the next occurrence of this delimiter in the given string
    ///
    /// Returns `Some((start_index, end_index))` if found, where:
    /// - `start_index` is where the delimiter begins
    /// - `end_index` is one past where the delimiter ends
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

/// A struct that splits a string by a delimiter
///
/// # Examples
///
/// ```
/// use lifetimes::StrSplit;
///
/// let haystack = "a b c d e";
/// let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
/// assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
/// ```

pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    /// Create a new StrSplit iterator
    ///
    /// # Examples
    ///
    /// ```
    /// use lifetimes::StrSplit;
    ///
    /// let split = StrSplit::new("hello world", " ");
    /// ```
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
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

/// Implement Delimiter for string slices
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

/// Implement Delimiter for characters
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}
/// Implement Delimiter for a slice of characters (matches any of them)
impl Delimiter for &[char] {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| self.contains(c))
            .map(|(start, c)| (start, start + c.len_utf8()))
    }
}

/// Helper function that returns the string until the first occurrence of a character
///
/// # Examples
///
/// ```
/// use lifetimes::until_char;
///
/// assert_eq!(until_char("hello world", 'o'), "hell");
/// ```
pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
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
