// 1.7 — Interior Mutability: Cell vs RefCell vs UnsafeCell
// Exercise: Shared Secrets Cache with Access Auditing
// Spec: see §4 of "1.7 Interior mutability - Cell vs RefCell vs UnsafeCell.md" in the notes vault.

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

type Key = String;
type Secret = String;
struct SecretsCache {
    entries: RefCell<HashMap<Key, Secret>>, // Shared secrets needs mutation. And RefCell
    // because Cell::get() wont work for non-Copy HashMap
    access_count: Cell<u32>, // Mutation without &mut requires Interior Mutability. Cell fits
                             // this usecase
}

impl SecretsCache {
    fn new() -> Self {
        Self {
            entries: RefCell::new(HashMap::new()),
            access_count: Cell::new(0),
        }
    }

    fn get(&self, key: &str) -> Option<Secret> {
        self.access_count.set(self.access_count.get() + 1);

        self.entries.borrow().get(key).cloned() // return a cloned Some(secret) and not a borrow
    }

    fn insert(&self, key: String, value: String) {
        let _ = self.entries.borrow_mut().insert(key, value);
    }
    fn try_insert(&self, key: String, value: String) -> bool {
        if let Ok(mut entries) = self.entries.try_borrow_mut() {
            if entries.get(&key).is_some() {
                return false;
            }
            entries.insert(key, value);
            return true;
        }

        false
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_get_and_access_count() {
        let cache = SecretsCache::new();
        cache.insert("AWS".into(), "EC@123".into());

        assert_eq!(
            cache.get("AWS"),
            Some("EC@123".to_string()),
            "Cache hit should return a Some() wrapped secret"
        );
        assert_eq!(
            cache.get("UNKNOWN_KEY"),
            None,
            "Cache miss should return a None."
        );

        assert_eq!(
            cache.access_count.get(),
            2,
            "Access count should increment on every cache.get()"
        )
    }

    #[test]
    fn test_rc_wrapped_cache() {
        let cache: Rc<SecretsCache> = Rc::new(SecretsCache::new());
        let cloned_cache = cache.clone();

        cache.insert("AWS".into(), "EC@123".into());
        cloned_cache.insert("GCP".into(), "VM0989".into());

        assert_eq!(
            cloned_cache.get("AWS"),
            Some("EC@123".to_string()),
            "Cloned cache handle should also point to the original cache."
        );
        assert_eq!(
            cache.get("GCP"),
            Some("VM0989".to_string()),
            "Original cache should be updated when cloned handle inserts an entry"
        );
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_insert_with_panic() {
        let cache = SecretsCache::new();
        cache.insert("AWS".into(), "EC@123".into());
        cache.insert("GCP".into(), "VM0989".into());
        cache.insert("AZU".into(), "CS4399".into());

        for _ in cache.entries.borrow().iter() {
            cache.insert("FAILEDKEY".into(), "NOTHING".into());
        }
    }
    #[test]
    fn test_insert_gracefully() {
        let cache = SecretsCache::new();
        cache.insert("AWS".into(), "EC@123".into());
        cache.insert("GCP".into(), "VM0989".into());
        cache.insert("AZU".into(), "CS4399".into());

        let result = cache.try_insert("REDIS".into(), "PASS@123".into());
        assert_eq!(result, true, "First insert should be allowed");
        let result = cache.try_insert("REDIS".into(), "PASS@123".into());
        assert_eq!(result, false, "Subsequent inserts should be disallowed");
    }
}
