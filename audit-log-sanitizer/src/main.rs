// 1.9 — Cow & clone-on-write patterns
// Exercise: Audit-Log Sanitizer
// Spec: see §4 of "1.9 Cow & clone-on-write patterns.md" in the notes vault.

use std::borrow::Cow;

fn sanitize(line: &str) -> Cow<str> {
    let mut current = Cow::Borrowed(line);
    // Password
    if let Some(redacted_line) = redact_marker(&current, "password=") {
        current = Cow::Owned(redacted_line);
    }

    // API Key
    if let Some(redacted_line) = redact_marker(&current, "api_key=") {
        current = Cow::Owned(redacted_line);
    }

    // Private Key
    if let Some(marker_index) = current.find("-----BEGIN") {
        let value_start_index = marker_index + "-----BEGIN".len();
        let redacted_line = format!("{}[REDACTED]", &current[..value_start_index],); // redact
        // until the end of line
        current = Cow::Owned(redacted_line);
    }

    current
}
fn redact_marker(line: &Cow<str>, marker: &str) -> Option<String> {
    if let Some(marker_index) = line.find(marker) {
        let value_start_index = marker_index + marker.len();
        let mut value_end_index = line.len(); // assuming value ends at a line break

        // checking if otherwise value ends with a whitespace
        if let Some(relative_whitespace_index) = line[value_start_index..].find(char::is_whitespace)
        {
            value_end_index = value_start_index + relative_whitespace_index;
        }
        let redacted_line = format!(
            "{}[REDACTED]{}",
            &line[..value_start_index],
            &line[value_end_index..]
        );
        return Some(redacted_line);
    }
    None
}

fn sanitize_batch<'a>(lines: &'a [&'a str]) -> Vec<Cow<'a, str>> {
    let mut output = vec![];
    for line in lines {
        output.push(sanitize(line));
    }

    output
}

fn finalize(sanitized: Cow<str>) -> String {
    // using into_owned() we get:
    // Owned = free move, no cloning required,
    // Borrowed = pays its allocation right here
    sanitized.into_owned()
}

fn peek_original<'a>(line: &Cow<'a, str>) -> Option<&'a str> {
    match line {
        Cow::Borrowed(str) => Some(str),
        Cow::Owned(_) => None,
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE_LINES: [&str; 15] = [
        "user=alice action=login status=success",
        "user=bob action=logout status=success",
        "connecting to db=primary host=10.0.0.5 port=5432",
        "password=hunter2 user=alice", // password, marker at start
        "user=carol action=view_dashboard",
        "cache hit for key=user:123 ttl=300",
        "user=dave api_key=sk_live_abc123 endpoint=/v1/charges", // api_key, mid-line
        "user=eve action=export format=csv rows=42",
        "event=cert_upload -----BEGIN RSA PRIVATE KEY-----", // private key
        "user=frank action=settings_update field=timezone",
        "user=grace password=test123", // password, at line end
        "health check ok latency_ms=12",
        "user=heidi api_key=sk_test_xyz789", // api_key, at line end
        "user=ivan action=login status=success",
        "user=judy password=p@ss api_key=sk_live_zzz111", // BOTH markers for step 5 case
    ];
    #[test]
    fn test_redaction_with_no_sensitive_data() {
        let line = "nothing sensitive here";

        let sanitized_line = sanitize(line);

        assert_eq!(line, sanitized_line);
        assert!(matches!(sanitized_line, Cow::Borrowed(_)));
        assert!(
            !sanitized_line.contains("[REDACTED]"),
            "Redaction should not work on non-sensitive text"
        )
    }
    #[test]
    fn test_password_at_whitespace_redacted() {
        let line_with_password =
            "here is my password, separated by a whitespace: password=test123 dont share it";

        let sanitized_line = sanitize(line_with_password);

        // println!("{}", sanitized_line);
        assert!(
            sanitized_line.contains("password=[REDACTED]"),
            "Redaction should work on Password"
        )
    }
    #[test]
    fn test_password_at_linebreak_redacted() {
        let line_with_password =
            "here is my password at line break, dont share it: password=test123";

        let sanitized_line = sanitize(line_with_password);

        // println!("{}", sanitized_line);
        assert!(
            sanitized_line.contains("password=[REDACTED]"),
            "Redaction should work on Password"
        )
    }
    #[test]
    fn test_api_key_at_whitespace_redacted() {
        let line_with_api_key =
            "here is my api_key, separated by a whitespace: api_key=test123 dont share it";

        let sanitized_line = sanitize(line_with_api_key);

        // println!("{}", sanitized_line);
        assert!(
            sanitized_line.contains("api_key=[REDACTED]"),
            "Redaction should work on API Key"
        )
    }
    #[test]
    fn test_api_key_at_linebreak_redacted() {
        let line_with_api_key = "here is my api_key at line break, dont share it: api_key=test123";

        let sanitized_line = sanitize(line_with_api_key);

        // println!("{}", sanitized_line);
        assert!(
            sanitized_line.contains("api_key=[REDACTED]"),
            "Redaction should work on API Key"
        )
    }
    #[test]
    fn test_private_key_redaction() {
        let line_with_private_key_begin = "event=cert_upload -----BEGIN RSA PRIVATE KEY-----";

        let sanitized_line = sanitize(line_with_private_key_begin);

        println!("{}", sanitized_line);
        assert!(
            sanitized_line.contains("-----BEGIN[REDACTED]"),
            "Redaction should work on Private Key"
        )
    }

    #[test]
    fn test_sanitize_batch_for_multi_lines() {
        let result = sanitize_batch(&SAMPLE_LINES);

        let mut borrow_count = 0;
        let mut owned_count = 0;

        for line in result.iter() {
            match line {
                Cow::Borrowed(_) => borrow_count += 1,
                Cow::Owned(_) => owned_count += 1,
            }
        }

        println!("{:#?}", result);
        assert_eq!(borrow_count, 9);
        assert_eq!(owned_count, 6);
    }

    #[test]
    fn test_finalize_always_returns_owned_string() {
        assert_eq!(finalize(Cow::Borrowed("s")), "s");
        assert_eq!(finalize(Cow::Owned("s".to_string())), "s");
    }
    #[test]
    fn test_peek_original_recovers_borrowed_lifetime() {
        let owner = String::from("hello");
        let borrowed: Cow<str> = Cow::Borrowed(&owner);

        assert_eq!(peek_original(&borrowed), Some("hello"));

        // prove the returned &str's lifetime is tied to `owner`, not to `borrowed` itself
        let peeked = peek_original(&borrowed);

        drop(borrowed);

        assert_eq!(peeked, Some("hello")); // still valid after a Cow is gone
    }

    #[test]
    fn test_peek_original_returns_none_for_owned() {
        let owned: Cow<str> = Cow::Owned(String::from("hello"));

        assert_eq!(peek_original(&owned), None);
    }
}
