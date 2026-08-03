// 2.6 — Closures: Fn, FnMut, FnOnce & capture semantics
// Exercise: Security Scan Job Runner
// Spec: see §4 of "2.6 Closures - Fn, FnMut, FnOnce & capture semantics.md" in the vault.
// Steps 1-5 complete.

use std::sync::{Arc, atomic::AtomicU32};

fn run_with_retry<F, T, E>(mut step: F, max_tries: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut last_result = step();
    for _ in 1..max_tries {
        if last_result.is_ok() {
            return last_result;
        }

        last_result = step();
    }

    last_result
}

struct ScanRunner {
    // 'static is implicit on Box<dyn Trait>
    handlers: Vec<Box<dyn Fn(&str) + Send + Sync + 'static>>,
}

impl ScanRunner {
    fn register_alert_handler<F>(&mut self, handler: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    fn raise_alert(&self, finding: &str) {
        for handler in self.handlers.iter() {
            handler(finding)
        }
    }
}

fn finish_scan(credential: String, on_finish: impl FnOnce(String) -> String) -> String {
    on_finish(credential)
    // on_finish(credential) // error[E0382]: use of moved value
}

fn main() {
    println!("=== scan starting ===");

    // Section 1: retry a flaky step ( FnMut - mutates its own capture counter across calls)
    let mut retry_attempts = 0;
    let flaky_step = move || {
        retry_attempts += 1;
        if retry_attempts < 3 {
            Err("Target unreachable, retrying")
        } else {
            Ok(retry_attempts)
        }
    };

    let retry_result = run_with_retry(flaky_step, 5);
    println!("retry result: {retry_result:?}");
    assert_eq!(retry_result, Ok(3));

    // Section 2: Register alert handlers (Fn + Send + Sync - callable many times from any thread)
    let mut runner = ScanRunner { handlers: vec![] };
    let alert_count = Arc::new(AtomicU32::new(0));
    let alert_count_clone = Arc::clone(&alert_count);

    let handler = move |finding: &str| {
        alert_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("[alert] {}", finding)
    };
    runner.register_alert_handler(handler);

    runner.raise_alert("open port 22 exposed to 0.0.0.0/0");
    runner.raise_alert("outdated TLS version detected");
    assert_eq!(alert_count.load(std::sync::atomic::Ordering::SeqCst), 2);

    // Section 3: Finish the scan - consumes the credential exactly once

    let credential = "Token-X-123".to_string();
    let on_finish = |credential: String| {
        println!("closing out credential: {}", credential);
        drop(credential);
        format!("receipt: credential consumed")
    };

    let receipt = finish_scan(credential, on_finish);
    println!("Scan complete: {receipt}");

    // Deliberate capture-related error, kept isolated
    let mut tries = 0;
    let step = || {
        tries += 1;
        if tries < 3 {
            Err("Not ready")
        } else {
            Ok(tries)
        }
    };
    // let first_attempt = step(); // error[E0596] cannot borrow `step` as mutable
}
