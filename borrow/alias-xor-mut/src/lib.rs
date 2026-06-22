fn firstmach<'a>(rules: &'a [String], prefix: &str) -> Option<&'a str> {
    for rule in rules {
        if rule.starts_with(prefix) {
            return Some(rule);
        }
    }
    None
}

#[test]
fn test_prefix_one() {
    let mut rules = vec!["AX".to_string(), "BX".to_string(), "CX".to_string()];

    let firstmatch = firstmach(&rules, "B");

    rules.push("DX".to_string());

    assert_eq!(firstmatch, Some("BX"), "DIDNT MATCH"); // Fix: move this line above WRITE to let
    // NLL flush the READ maintaining either MANY READS or ONE WRITE (aliasing-xor-mutability)
}
