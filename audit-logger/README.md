This is a tamper-evident Audit logger that explores Rust's Drop vs an explicit `close()` trade off.

How is it Production ready in a security context?
- The opened Log File only has Append-only file access with `append()`. Exluded giving it a `read()` handle to apply the principle of least privilege on the file handle
- BufWriter has an explicit `flush()` so no partial writes
- Used standard error propagation with `?`. Errors are not swallowed or hidden
- A `closed` flag helps identify if `close()` was explicitly called making failed flushes retryable with Drop
- The `Drop` implementation acts as a safety net and any forgotten close calls still flushes the writes using it
- A guard `std::thread::panicking()` helps identify any already panick unwinding from stack trace thereby preventing double-free then abort
- a loud failure to devs if they failed to call `close()` and not silently rerunning it in Drop.
