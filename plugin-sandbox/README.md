# plugin-sandbox

Three ways to run an untrusted plugin closure, escalating from "no protection" to "designed not to need protection" — the defensive-vs-preventive contrast this exercise is built around.

## What's in here

1. `run_plugin_unsafe` is the unprotected baseline — a panicking plugin crashes whatever calls it. 
2. `run_plugin_isolated` wraps the call in `catch_unwind`, converting a caught panic into `PluginError::Panicked(String)` via a message extractor that handles both panic-payload shapes (`&str` for a literal `panic!("...")`, `String` for a formatted one). 
3. `run_plugin_safe` is the preventive alternative: a plugin shape that returns `Result<bool, String>` directly instead of using `panic!` for its own failure signaling — no `catch_unwind`, no `UnwindSafe` bound, nothing to defend against, because there's no panic in the picture at all.

