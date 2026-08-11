// 3.5 — HashMap/HashSet/BTreeMap internals & the entry() API
// Exercise: Failed-Login Attempt Tracker
// Spec: see §4 of "3.5 HashMap, HashSet, BTreeMap internals and the entry API.md" in the vault.
// Steps 1-5 complete.

use std::collections::{BTreeMap, HashMap};

struct AttemptTracker {
    counts: HashMap<String, u32>,
}

impl AttemptTracker {
    fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }
    fn record_failure(&mut self, ip: &str) {
        *self.counts.entry(ip.to_string()).or_insert(0) += 1;
    }

    fn attempts_for(&self, ip: &str) -> u32 {
        // Not using entry() as it requires an owned K (expensive), while the get() works with borrow (cheap)
        if let Some(count) = self.counts.get(ip) {
            return *count;
        } else {
            0
        }
    }

    fn top_offenders(&self, n: usize) -> Vec<(String, u32)> {
        let mut list: Vec<(String, u32)> =
            self.counts.iter().map(|(k, v)| (k.into(), *v)).collect();

        // descending sort for failure counts upto n
        // and when counts match for 2 ips, then ascending order for those IPs
        list.sort_by(|(k1, v1), (k2, v2)| v2.cmp(v1).then_with(|| k1.cmp(k2)));
        list.truncate(n);
        list
    }
}

struct AttemptTrackerSorted {
    counts: BTreeMap<String, u32>,
}

impl AttemptTrackerSorted {
    fn new() -> Self {
        Self {
            counts: BTreeMap::new(),
        }
    }

    fn record_failure(&mut self, ip: &str) {
        *self.counts.entry(ip.to_string()).or_insert(0) += 1;
    }

    fn attempts_for(&self, ip: &str) -> u32 {
        if let Some(count) = self.counts.get(ip) {
            return *count;
        } else {
            0
        }
    }

    fn offenders_in_range(&self, start: &str, end: &str) -> Vec<(String, u32)> {
        self.counts
            .range(start.to_string()..end.to_string())
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    }
}

fn main() {
    let mut t = AttemptTracker::new();
    t.record_failure("10.0.0.1");
    t.record_failure("10.0.0.1");
    t.record_failure("10.0.0.2");
    assert_eq!(t.attempts_for("10.0.0.1"), 2);
    assert_eq!(t.attempts_for("10.0.0.3"), 0);
    assert_eq!(t.counts.len(), 2);
    assert_eq!(t.top_offenders(1), vec![("10.0.0.1".to_string(), 2)]);

    let mut s = AttemptTrackerSorted::new();
    s.record_failure("10.0.0.5");
    s.record_failure("10.0.0.15");
    s.record_failure("10.0.0.25");
    let ranged = s.offenders_in_range("10.0.0.1", "10.0.0.2");
    assert_eq!(ranged, vec![("10.0.0.15".to_string(), 1)]);

    println!("all assertions passed");
}
