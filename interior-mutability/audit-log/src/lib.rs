// REFCELL usage

use std::cell::{Ref, RefCell};

struct AuditLog {
    entries: RefCell<Vec<String>>,
}

impl AuditLog {
    fn new() -> Self {
        Self {
            entries: RefCell::new(vec![]),
        }
    }

    fn record(&self, entry: impl Into<String>) {
        self.entries.borrow_mut().push(entry.into());
    }

    fn latest(&self) -> Option<Ref<String>> {
        let latest = Ref::filter_map(self.entries.borrow(), |v| v.last()).ok();

        latest
    }

    fn latest_with_map(&self) -> Option<Ref<String>> {
        if self.entries.borrow().is_empty() {
            return None;
        }
        let latest = Ref::map(self.entries.borrow(), |v| v.last().unwrap());

        Some(latest)
    }
}

#[cfg(test)]
mod test {
    use crate::AuditLog;

    #[test]
    fn empty_log() {
        let log = AuditLog::new();
        assert!(log.latest().is_none())
    }

    #[test]
    fn latest_entry() {
        let log = AuditLog::new();

        log.record("login: root");
        log.record("exec: /bin/bash");
        log.record("exec: ls /root");

        let last = log.latest();
        assert_eq!("exec: ls /root", *last.unwrap())
    }

    #[test]
    fn borrow_released_after_ref_drop() {
        let log = AuditLog::new();

        log.record("ls a");
        {
            let _last = log.latest().unwrap();
            // shared borrow is active in above line, below line will panic
            // log.record("ls b");
        }
        log.record("ls b");
    }
}
