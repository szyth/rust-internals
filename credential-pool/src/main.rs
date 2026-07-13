// 1.6 — Drop, RAII & Cleanup Ordering
// Exercise: Auto-Returning Credential Pool
// Spec: see §4 of "1.6 Drop, RAII & cleanup ordering.md" in the notes vault.

use std::{cell::RefCell, rc::Rc};

struct CredentialPool {
    creds: Rc<RefCell<Vec<u32>>>,
}

struct CredentialGuard {
    checked_out_cred: u32,
    pool: Rc<RefCell<Vec<u32>>>,
}

impl CredentialPool {
    fn new() -> Self {
        Self {
            creds: Rc::new(RefCell::new(Vec::new())),
        }
    }
    fn insert_cred(&self, cred: u32) {
        if let Ok(mut creds) = self.creds.try_borrow_mut() {
            creds.push(cred);
        }
    }

    fn available_count(&self) -> usize {
        self.creds.borrow().len()
    }
    fn checkout(&self) -> Option<CredentialGuard> {
        match self.creds.try_borrow_mut() {
            Ok(mut cred) => {
                if let Some(cred) = cred.pop() {
                    return Some(CredentialGuard {
                        checked_out_cred: cred,
                        pool: Rc::clone(&self.creds),
                    });
                }
            }
            Err(_e) => {
                return None;
            }
        }

        None
    }
}

impl Drop for CredentialGuard {
    fn drop(&mut self) {
        // append guarded credential back into pool
        if let Ok(mut cred) = self.pool.try_borrow_mut() {
            println!("returning credential: {}", self.checked_out_cred);
            cred.push(self.checked_out_cred);
        } else {
            eprintln!("Failed to return checked out credential back to pool.")
        }
    }
}

fn checkout_and_maybe_return_early(guard: CredentialGuard, early_return: bool) {
    let cred = guard.checked_out_cred;
    if early_return {
        println!("returing early with checked out cred: {}", cred);
        return;
    }
    println!("checked out: {}", cred);
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_normal_drop_with_scope() {
        let creds_pool = CredentialPool::new();
        creds_pool.insert_cred(1);
        creds_pool.insert_cred(2);
        creds_pool.insert_cred(3);
        creds_pool.insert_cred(4);
        println!("Pool count: {}", creds_pool.available_count());
        {
            let _guard = creds_pool.checkout().unwrap();
            println!("Pool count: {}", creds_pool.available_count());
        } // CredentialGuard dropped here
        println!("Pool count: {}", creds_pool.available_count());

        assert_eq!(
            creds_pool.available_count(),
            4,
            "Pool count should return back to 4 after a normal scope drop"
        );
    }
    #[test]
    fn test_early_drop() {
        let creds_pool = CredentialPool::new();
        creds_pool.insert_cred(1);
        creds_pool.insert_cred(2);
        creds_pool.insert_cred(3);
        creds_pool.insert_cred(4);

        println!("Pool count: {}", creds_pool.available_count());

        {
            let guard = creds_pool.checkout().unwrap();
            checkout_and_maybe_return_early(guard, true);
            println!("Pool count: {}", creds_pool.available_count());
        }
        println!("Pool count: {}", creds_pool.available_count());

        assert_eq!(
            creds_pool.available_count(),
            4,
            "Pool count should return back to 4 even after an early return"
        );
    }

    #[test]
    fn test_gracefull_none() {
        let creds_pool = CredentialPool::new();
        creds_pool.insert_cred(1);
        creds_pool.insert_cred(2);
        creds_pool.insert_cred(3);
        creds_pool.insert_cred(4);

        let mut guards = vec![];
        for _ in 0..4 {
            // consume all creds
            let guard = creds_pool.checkout().unwrap(); // safe to use unwrap
            guards.push(guard);
        }
        assert!(
            creds_pool.checkout().is_none(),
            "Checkout on an empty pool should return None"
        );
        println!("Gracefull None on an empty pool");
    }

    #[test]
    fn test_stack_vars_lifo_cleanup() {
        let creds_pool = CredentialPool::new();
        creds_pool.insert_cred(1);
        creds_pool.insert_cred(2);
        creds_pool.insert_cred(3);
        creds_pool.insert_cred(4);

        println!("Pool: {:?}", creds_pool.creds);
        let _a = creds_pool.checkout().unwrap();
        let _b = creds_pool.checkout().unwrap();
        let _c = creds_pool.checkout().unwrap();

        // checkout order : 4 -> 3 -> 2 (a,b,c)
        // drop order LIFO: 2 -> 3 -> 4 (c,b,a)
        println!("Pool: {:?}", creds_pool.creds);
    }
    #[test]
    fn test_wildcard_underscore_drops_immediately() {
        let creds_pool = CredentialPool::new();
        creds_pool.insert_cred(1);
        creds_pool.insert_cred(2);
        creds_pool.insert_cred(3);
        creds_pool.insert_cred(4);

        let _ = creds_pool.checkout(); // Guard assigned and dropped immediately
        println!("Available count: {}", creds_pool.available_count());
        assert_eq!(
            creds_pool.available_count(),
            4,
            "Pool count should return back to 4 immediately after a bare _ discard"
        );
    }
}
