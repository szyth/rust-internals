#[derive(Debug, PartialEq)]
struct FirewallRule {
    priority: u8,
    pattern: String,
    ports: Vec<u16>,
}

impl Clone for FirewallRule {
    fn clone(&self) -> Self {
        Self {
            priority: self.priority,
            pattern: self.pattern.clone(),
            ports: self.ports.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_success() {
        let mut rule = FirewallRule {
            priority: 1,
            pattern: "Active".to_string(),
            ports: vec![22, 443],
        };

        let cloned_rule = rule.clone();
        assert_eq!(
            rule, cloned_rule,
            "The cloned data should match the original"
        );

        // modify original, check deep copy
        rule.pattern = "Inactive".to_string();

        assert_ne!(
            rule, cloned_rule,
            "Modifying the original should not alter the clone"
        );
    }
}
