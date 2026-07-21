// 1.11 — `&str` vs `String`, slicing, UTF-8 boundaries
// Exercise: Safe Log-Line Truncator for a Security Console
// Spec: see §4 of "1.11 &str vs String, slicing, UTF-8 boundaries.md" in the vault (topics/).

fn truncate_display(s: &str, max_bytes: usize) -> &str {
    if max_bytes >= s.len() {
        return s;
    }

    let mut valid_char_index = max_bytes;
    loop {
        if s.is_char_boundary(valid_char_index) {
            break;
        }
        valid_char_index -= 1;
    }
    &s[..valid_char_index]
}
fn truncate_with_marker(s: &str, max_bytes: usize, marker: &str) -> String {
    if max_bytes >= s.len() {
        return s.to_string();
    }

    let mut valid_char_index = {
        if max_bytes < marker.len() {
            0
        } else {
            max_bytes - marker.len()
        }
    };
    loop {
        if s.is_char_boundary(valid_char_index) {
            break;
        }
        valid_char_index -= 1;
    }
    if valid_char_index == 0 && max_bytes < marker.len() {
        return String::new();
    }
    format!("{}{}", &s[..valid_char_index], marker)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{truncate_display, truncate_with_marker};

    #[test]
    fn test_truncate_display_never_panics() {
        let s = "héllo wörld";
        for i in 0..=s.len() + 5 {
            let _ = truncate_display(s, i);
        }
    }

    #[test]
    fn test_truncate_with_marker_appends_when_truncated() {
        let s = "héllo wörld";
        let result = truncate_with_marker(s, 5, "...");
        assert!(result.ends_with("..."), "expected marker at the end, got {:?}", result);
    }

    #[test]
    fn test_truncate_with_marker_omits_marker_when_untruncated() {
        let s = "héllo wörld";
        let result = truncate_with_marker(s, s.len(), "...");
        assert_eq!(result, s, "no truncation happened, marker should not appear");
    }

    #[test]
    fn test_truncate_with_marker_respects_total_budget() {
        let s = "héllo wörld";
        for max in 0..s.len() {
            let result = truncate_with_marker(s, max, "...");
            assert!(
                result.len() <= max,
                "max={}: result {:?} (len {}) exceeded budget",
                max, result, result.len()
            );
        }
    }

    #[test]
    fn test_truncate_with_marker_budget_smaller_than_marker() {
        let s = "héllo wörld";
        let result = truncate_with_marker(s, 2, "..."); // marker is 3 bytes, budget is 2
        assert_eq!(result, "", "budget smaller than marker should omit marker entirely");
    }

    #[test]
    fn test_truncate_mixed_byte_widths_never_splits_a_character() {
        let s = "A\u{e9}\u{20ac}\u{1f389}"; // 'A' (1B) + 'é' (2B) + '€' (3B) + '🎉' (4B)
        for max in 0..=s.len() + 5 {
            let result = truncate_display(s, max);
            assert!(s.starts_with(result), "result {:?} is not a genuine prefix of {:?}", result, s);
            assert!(result.len() <= max, "result exceeded requested budget");
        }
    }
}
