// 3.2 — Custom iterators & IntoIterator
// Exercise: Retry Backoff Sequence & Audit Trail Batch
// Spec: see §4 of "3.2 Custom iterators and IntoIterator.md" in the vault.
// Steps 1-6 complete.

struct Backoff {
    current_ms: u64, // current delay
    max_ms: u64,     // maximum cap
}

impl Iterator for Backoff {
    type Item = u64;

    // Backoff is not a sequence of iterable items that gets exhausted or "gets over".
    // We always need a time to retry the request.
    // Hence it should never return None.

    fn next(&mut self) -> Option<Self::Item> {
        let delay = self.current_ms;
        self.current_ms = (self.current_ms * 2).min(self.max_ms);

        Some(delay)
    }
}

// added mod {} to add the encapsulation making this scope's field private in main()
// to demonstrate the idiomatic fix
mod audit {

    pub struct AuditTrail {
        entries: Vec<String>,
        is_a_bool_field: bool,
    }

    impl AuditTrail {
        pub fn new() -> Self {
            Self {
                entries: vec![],
                is_a_bool_field: true,
            }
        }

        pub fn record(&mut self, entry: impl Into<String>) {
            self.entries.push(entry.into());
        }
    }

    impl IntoIterator for AuditTrail {
        type Item = String;

        type IntoIter = std::vec::IntoIter<String>;

        fn into_iter(self) -> Self::IntoIter {
            self.entries.into_iter()
        }
    }

    impl<'a> IntoIterator for &'a AuditTrail {
        type Item = &'a String;

        type IntoIter = std::slice::Iter<'a, String>;

        fn into_iter(self) -> Self::IntoIter {
            self.entries.iter()
        }
    }
}

fn main() {
    let mut audit_trail = audit::AuditTrail::new();
    audit_trail.record("login failed for user=alice");
    audit_trail.record("login failed for user=alice");
    audit_trail.record("account locked for user=alice");

    // for entry in audit_trail.entries {} // error[E0616]: cant access private field

    // Solution: escape hatch: make `entries` public, or create a getter get_inner()
    // Solution: idiomatic fix: impl IntoIterator

    // Borrow
    for entry in (&audit_trail).into_iter() {
        println!("{entry}");
    }
    // Own
    for entry in audit_trail.into_iter() {
        println!("{entry}");
    }

    // let value_moved = audit_trail; // error[E0382]: value moved

    // without the IntoIterator impl for AuditTrail, the above for loop gives error[E0277]: audit_trail
    // is not an iterator
}

#[cfg(test)]
mod backoff_tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_backoff_doubles_then_caps() {
        let first_ten = Backoff {
            current_ms: 100,
            max_ms: 5000,
        }
        .take(10)
        .collect::<Vec<_>>();

        assert_eq!(
            [100, 200, 400, 800, 1600, 3200, 5000, 5000, 5000, 5000],
            first_ten.as_ref()
        );

        let sum: u64 = first_ten.iter().sum();
        assert_eq!(26300, sum);
    }

    #[test]
    fn test_backoff_map_to_duration() {
        let durations = Backoff {
            current_ms: 100,
            max_ms: 5000,
        }
        .take(3)
        .map(|ms| Duration::from_millis(ms))
        .collect::<Vec<_>>();

        assert_eq!(
            [
                Duration::from_millis(100),
                Duration::from_millis(200),
                Duration::from_millis(400),
            ],
            durations.as_ref()
        );
    }
}
