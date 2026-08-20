# parallel-password-auditor

Three variants of the same chunked-parallel password audit, built to feel out `std::thread::scope`'s `'scope` lifetime relaxation against plain `thread::spawn`'s `'static` bound — plus what "implicit" vs "explicit" join actually changes when a worker panics.

## What's in here

`audit_batch` splits `passwords: &[String]` / `results: &mut [bool]` into `available_parallelism()`-many disjoint chunks (`chunks`/`chunks_mut`, sized via `div_ceil` so the split never truncates) and processes each chunk on its own `s.spawn`-ed thread inside `thread::scope`. `audit_batch_spawn` is the same logic with `thread::scope` swapped for plain `thread::spawn` — it doesn't compile, and is kept commented out with the error attached.

## The actual finding

`audit_batch_spawn` fails with `E0521` twice, once for `passwords` and once for `results` — not because the chunks are borrowed differently, but because `Slice::chunks` returns items with the *same* lifetime as the original slice reference (`Chunks<'a, T>: Iterator<Item = &'a [T]>`), so capturing a chunk in a `'static`-bound closure is really capturing the parameter's own lifetime. `thread::scope` doesn't sidestep this by borrowing differently — it gives the compiler a `'scope` it can prove is bounded by the `scope()` call itself, so `'static` is never required.

Separately: `thread::scope` re-panics on scope exit if any spawned thread panicked and was never joined — `audit_batch` (fire-and-forget `s.spawn`, no handle collection) demonstrates this by propagating a panic straight out of the function when one chunk contains a poison-pill entry. `audit_batch_safe` collects each `ScopedJoinHandle` and joins them explicitly instead; a panicked chunk's `Err` is caught and logged locally, and the other chunks' results are unaffected — confirmed by asserting the full `results` vector against the poison entry's actual position.
