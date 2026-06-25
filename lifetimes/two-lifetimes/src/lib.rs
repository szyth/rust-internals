// 'a = intersection (shorter) of the two input scopes.
// If the inputs have very different scopes,
// the shorter one limits how long the output can be used
// If it is too restrictive, then use String
fn select_log_entry<'a>(primary: &'a str, fallback: &'a str, use_primary: bool) -> &'a str {
    if use_primary {
        return primary;
    }
    return fallback;
}
