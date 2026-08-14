// 3.9 — panic: unwind vs abort, catch_unwind, when to panic vs Result
// Exercise: Plugin Sandbox
// Spec: see §4 of "3.9 panic - unwind vs abort, catch_unwind, when to panic vs Result.md" in the vault.

// Run Unsafe plugin: Version 1
// Running and handling third-party, not-trusted plugins that coult panic inside our sandbox
fn run_plugin_unsafe(plugin: impl FnOnce() -> bool) -> bool {
    plugin()
}

enum PluginError {
    Panicked(String),
}

// panic could return a &str, String or a non-string message: Handled all
fn extract_panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    // if let Some(panic_msg) = payload.downcast_ref::<&str>() {
    //     panic_msg.to_string()
    // } else if let Some(panic_msg) = payload.downcast_ref::<String>() {
    //     panic_msg.clone()
    // } else {
    //     "unknown panic payload".to_string()
    // }

    // equivalent combinator version of above; more idiomatic
    payload
        .downcast_ref::<&str>()
        .map(|panic_msg| panic_msg.to_string())
        .or_else(|| {
            payload
                .downcast_ref::<String>()
                .map(|panic_msg| panic_msg.clone())
        })
        .unwrap_or_else(|| format!("unknown panic payload"))
}

// Run Unsafe plugin: Version 2
fn run_plugin_isolated<F: FnOnce() -> bool + std::panic::UnwindSafe>(
    plugin: F,
) -> Result<bool, PluginError> {
    std::panic::catch_unwind(plugin)
        .map_err(|err| PluginError::Panicked(extract_panic_message(err)))
}

// a struct to count how many plugis ran (success or failure both)
// creating this struct to show the UnwindSafe error on `&mut T`
struct ScanStats {
    plugins_run: u32,
}

// Run Unsafe plugin: Version 3
// Idiomatic using Result rather than handling the panic
fn run_plugin_safe<F: FnOnce() -> Result<bool, String>>(plugin: F) -> Result<bool, String> {
    plugin()
}

fn main() {
    // Hitting the UnwindSafe wall on purpose
    // error[E0277] &mut T inside UnwindSafe is not safe
    // let mut stats = ScanStats { plugins_run: 0 };
    // run_plugin_isolated(|| {
    //     stats.plugins_run += 1;
    //     true
    // });
    // Idiomatic fix:
    // run_plugin_isolated(|| true);
    // stats.plugins_run += 1;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn isolated_plugin_catches_literal_panic() {
        // &str
        let result = run_plugin_isolated(|| panic!("bad plugin"));
        assert!(matches!(result, Err(PluginError::Panicked(msg)) if msg == "bad plugin"));
    }
    #[test]
    fn isolated_plugin_catches_formatted_panic() {
        // String
        let result = run_plugin_isolated(|| panic!("bad plugin: code {}", 42));
        assert!(matches!(result, Err(PluginError::Panicked(msg)) if msg == "bad plugin: code 42"));
    }

    #[test]
    fn isolated_plugin_success_returns_ok() {
        let result = run_plugin_isolated(|| true);
        assert!(matches!(result, Ok(true)));
    }

    #[test]
    fn stats_count_both_success_and_panic() {
        let mut stats = ScanStats { plugins_run: 0 };

        // a panicked plugin, counted as "ran" even though it failed
        let _ = run_plugin_isolated(|| panic!("bad plugin"));
        stats.plugins_run += 1;

        let _ = run_plugin_isolated(|| true);
        stats.plugins_run += 1;

        assert_eq!(stats.plugins_run, 2);
    }

    #[test]
    fn safe_plugin_propagates_failure_without_panicking() {
        let result = run_plugin_safe(|| Err("bad plugin".to_string()));
        assert_eq!(result, Err("bad plugin".to_string()));
    }

    #[test]
    fn safe_plugin_success_returns_ok() {
        let result = run_plugin_safe(|| Ok(true));
        assert_eq!(result, Ok(true));
    }
}
