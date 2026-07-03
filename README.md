# Rust Internals

  Recreating and exploring advanced Rust patterns and internals. Each folder is a small, self-contained exercise built around a realistic scenario, not a toy demo.

  ## Index

  | Exercise | Concepts demonstrated |
  |---|---|
  | `audit-logger` | File-backed logging, `BufWriter`, explicit `closed`-flag tracking vs. forgetting to close on an early-return path |
  | `clone-low-level` | Manual `Clone` implementation via raw pointers and `memcpy`/`MaybeUninit` |
  | `event-subscription-registry` | `Rc`/`Weak` — shared ownership between config and subscribers without reference cycles |
  | `lazy-cell` | Hand-rolled lazy initialization built from `UnsafeCell` + `Cell` |
  | `packet-counter` | `Cell`-based zero-cost counters on a `&self` API |
  | `str_split` | Generic string-splitting iterator, progressively refined (v1 → v6) |
  | `task-registry` | Type-erased callback registry — `Box<dyn Fn() + Send + 'static>` |
  | `secret-vault` | Move semantics, `Copy`/`Clone` exclusion, ownership-enforced single-use guarantees |
