# login-attempt-actor

A login-attempt tracker built as a `tokio::sync` actor — one task privately owns a `HashMap<String, u32>` of per-user failure counts, and every other task talks to it by sending `mpsc` messages and awaiting a per-request `oneshot` reply, with zero `Arc<Mutex<_>>` anywhere.

## What's in here

`Command` is the actor's entire public API expressed as data: `RecordFailure { user, respond_to: oneshot::Sender<u32> }` and `IsLockedOut { user, threshold, respond_to: oneshot::Sender<bool> }`, each carrying its own one-time reply channel since many callers share one `mpsc::Sender<Command>`. `run_actor` is a single `while let Some(cmd) = rx.recv().await` loop — the only code in the program that ever touches the `HashMap` — matching on each `Command` and answering through `respond_to`. `TrackerHandle` is a cloneable wrapper around the `mpsc::Sender`; its constructor spawns `run_actor` once and returns immediately, and `record_failure`/`is_locked_out` each build a fresh `oneshot::channel()`, send the matching `Command`, and `.await` the reply — propagating a `Result` instead of `.unwrap()`ing, so a dead actor becomes the caller's problem to decide about, not an automatic panic.

## The actual finding

Comparing "genuinely different implementations of the same interface" is the honest way to evaluate a design trade-off — the file keeps a commented-out `Arc<Mutex<HashMap<_, _>>>` version of the same two operations side by side with the actor version, both verified to produce identical output (`[1, 2, 3, 4, 5]` from 5 concurrent callers) for the concurrency test. The actor version isn't faster or "more correct" for this exact toy case — the real difference is where the safety guarantee lives: `Arc<Mutex<_>>` makes concurrent access *safe* by requiring every access path to remember to lock; the actor makes concurrent access to the map *structurally impossible*, since no code outside `run_actor`'s own stack frame can ever reach it at all.
