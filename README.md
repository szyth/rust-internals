# Rust Internals

  Recreating and exploring advanced Rust patterns and internals. Each folder is a small, self-contained exercise built around a realistic scenario.

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
  | `packet-inspector` | Borrowing, aliasing (`shared XOR mutable`), NLL, two-phase borrows |
  | `log-tokenizer` | Lifetime elision, multi-parameter lifetimes, independent field lifetimes |
  | `plugin-registry` | `'static` bound satisfaction (owned data vs. `&'static str` literals vs. a genuine `E0597` failure), `dyn Trait` type erasure vs. per-call borrow lifetimes |
  | `credential-pool` | `Drop`/RAII auto-return (`MutexGuard`-style pattern) via `Rc<RefCell<_>>` shared state, LIFO drop ordering, wildcard `_` vs named-binding drop timing |
  | `secrets-cache` | `Cell` vs `RefCell` choice by access shape, mutation through `&self` via interior mutability, `RefCell`'s runtime `BorrowMutError` panic vs. non-panicking `try_borrow_mut`, `Rc`-shared mutation across handles |
  | `process-ancestry-tree` | `Rc`+`Weak` parent/child tree that avoids a reference cycle (`RefCell<Vec<Rc<Node>>>` owns children, `RefCell<Weak<Node>>` observes parent), `Weak::upgrade()` reflecting live strong-owner state as owners drop one by one, cascading `Drop` triggered by releasing the last strong owner |
  | `audit-log-sanitizer` | `Cow`-based deferred allocation (zero-cost clean path, allocate only on redaction), `Cow::Borrowed`/`Owned` proven via variant matching (not just content), batch instrumentation of Borrowed vs Owned counts, `into_owned()` at the correct hand-off boundary, recovering the true `'a` lifetime via explicit `match` (not `Deref`/`AsRef`) |

