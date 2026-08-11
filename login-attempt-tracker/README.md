# login-attempt-tracker

A failed-login attempt tracker built two ways — `HashMap`-backed and `BTreeMap`-backed — to feel out `entry()` and the two structures' actual trade-off firsthand.

## What's in here

`AttemptTracker` (`HashMap<String, u32>`) does the classic get-or-insert-and-increment via `entry()` in one lookup, and a `top_offenders(n)` report that has to collect-and-sort since a `HashMap` has no ordering to lean on. `AttemptTrackerSorted` (`BTreeMap<String, u32>`) does the same increment, plus `offenders_in_range()` via `.range()` — a query the `HashMap` version has no reasonable way to support.

`attempts_for` deliberately does **not** use `entry()` — a read-only query has no business inserting a `0`-count entry for an IP that was only ever asked about, never attempted.

## The trade-off

`record_failure` is the hot path — it fires on every failed login, plausibly thousands of times a second under attack. `top_offenders`/`offenders_in_range` fire on a periodic dashboard poll, orders of magnitude less often. That asymmetry favors `HashMap`: keep the hot path at `O(1)` amortized, and pay an occasional sort (or range scan) only on the rare reporting path. Keeping the map sorted at all times (`BTreeMap`, `O(log n)` per `record_failure`) would trade a cost paid on *every single hot-path call* just to make the rare reporting path marginally cheaper — the wrong trade here, same reasoning as [[3.4 Vec, VecDeque internals and complexity tradeoffs]]'s audit-log buffer favoring `VecDeque` for its hot path over the rare report-generation path.
