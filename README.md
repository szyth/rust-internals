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
  | `descriptor-header` | `repr(C)`/`repr(packed)`/`repr(transparent)` layout, hand-derived and verified via `size_of`/`align_of`/`offset_of!`; manual field reordering vs. `repr(Rust)`'s auto-optimization; the `E0793` packed-reference hazard; const-time layout assertions |
  | `log-line-truncator` | UTF-8 byte-boundary-safe truncation via `is_char_boundary` walk-back (never panics on a multi-byte split, at any offset), marker-appending within a fixed byte budget, exhaustive no-panic proof across every possible cutoff, correctness verified against mixed 1/2/3/4-byte UTF-8 |
  | `log-filter-pipeline` | Static vs. dynamic dispatch on one trait; `Vec<Box<dyn Trait>>` heterogeneity; `E0038` dyn-incompatibility fixed via supertrait split or `where Self: Sized`; default-method `static` shared across monomorphized instantiations |
  | `security-event-audit-trail` | Supertrait dependency enforced at the `impl` site (`E0277`) but granting zero shared implementation; default method overridden per type; `where`-clause bound; supertrait access proven one-directional|
  | `security-event-source` | Associated type limited to one `impl` per type (`E0119`) vs. generic parameter allowing multiple coexisting `impl`s for the same type; generic functions needing one vs. two type parameters for the equivalent shape;hybrid trait combining a generic key parameter with an associated output type |
  | `ring-buffer` | Const generics (`RingBuffer<T, const N: usize>`) as genuinely distinct monomorphized types per `N` (`E0308`); fixed-capacity, stack-allocated overwrite-on-full semantics via `head`/`len` wraparound arithmetic; a `const { assert!(N > 0) }` block rejecting zero-capacity buffers at compile time (`E0080`), not runtime |
  | `credential-handle` | `Send`/`Sync` as auto traits proven via `assert_send`/`assert_sync` (`E0277`); `PhantomData<Rc<()>>` forcing `!Send`/`!Sync`; `unsafe impl Send`/`Sync` overriding it with a `SAFETY`-justified comment |
  | `scan-job-runner` | `Fn`/`FnMut`/`FnOnce` chosen by capture usage, not declaration; a retry loop (`FnMut`), an alert registry (`Fn + Send + Sync`, `Box<dyn Fn(&str) + Send + Sync>`), and a one-shot credential callback (`FnOnce`); the `E0596` gotcha calling an `FnMut` closure from a non-`mut` binding |
  | `firewall-rule-validator` | Newtype pattern (`Port`, `Hostname`) preventing type confusion (`E0308`); `TryFrom` validated construction; `Deref` implemented then rejected after it leaked raw arithmetic; composed fallible conversions via `?` with errors unified through `From` |
  | `log-line-scanner` | Lazy adapter chain (`map`/`inspect`/`filter`) proven to do nothing until a terminal call; short-circuiting `find()` over a genuinely infinite range vs. full-drain `collect()` over a bounded one; the `#[must_use]` guardrail on unconsumed iterators |
  | `backoff-audit-trail` | Custom `Iterator` (`Backoff`, exponential-capped delay generator) built from just `next()`; `IntoIterator` implemented separately for `AuditTrail` (owned, consuming) and `&AuditTrail` (borrowed, leaves it usable); private-field encapsulation via a submodule (`E0616`), proven with the real `E0382`/`E0277` guardrails |
| `rate-limiter-window` | `VecDeque`-backed sliding-window rate limiter (`O(1)` front-eviction) vs. a `Vec`-backed twin (`O(n)` `remove(0)`); benchmarked at two scales |
| `login-attempt-tracker` | `entry()`-based get-or-insert-and-increment on a `HashMap<String, u32>`; `top_offenders` collect-and-sort report vs. a `BTreeMap`-backed twin's `.range()` query; a read-only lookup deliberately avoiding `entry()` to prevent a phantom `0`-count insert |
| `bearer-token-authorizer` | Five-step `Result`/`Option` combinator chain (`ok_or`/`ok_or_else`/`and_then`) replacing a nested `match` pyramid; each failure mode tagged with its own `AuthError` variant instead of collapsing to a bare `None`; the `.map()`-vs-`.and_then()` trap (a plain transform can't branch into `Ok`/`Err`) hit and fixed live |
| `secure-config-loader` | Three fallible steps (`std::io::Error`, custom `ParseError`, custom `ValidationError`) unified into one `ConfigError` via `?` and three `From` impls; an inverted and later off-by-one port-range check caught live; `matches!` used over `assert_eq!` since `std::io::Error` has no `PartialEq` impl |
| `password-strength-checker` | Library (`lib.rs`) vs Binary (`main.rs`) split — a `thiserror`-derived `PasswordError` enum consumed by `anyhow`-based code with zero manual `From` glue; `.downcast_ref()` recovering the concrete variant from a type-erased `anyhow::Error`; `Display` vs `Debug` divergence (context message only vs. the full "Caused by" chain) asserted directly |


