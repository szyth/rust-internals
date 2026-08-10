# rate-limiter-window

A sliding-window rate limiter (`VecDeque<u64>`-backed) built to feel out `Vec` vs `VecDeque`'s front-operation cost difference firsthand, plus a `Vec`-backed twin for comparison.

## What's in here

`RateLimiter::record_attempt(now)` evicts every timestamp older than `now - window_ticks` from the front (always the oldest, since new ones only ever land on the back), then admits the attempt if there's room. `pop_front` makes eviction `O(1)`; `RateLimiterVecBacked` does the identical thing with `Vec::remove(0)` instead, which is `O(current length)` per call.

## The actual finding

Benchmarking both at `n=5,000` and `n=50,000` attempts (capacity 50, window 10 ticks — most attempts trigger an eviction) gives a roughly **flat** `VecDeque`-vs-`Vec` ratio (~2x at both scales), not a widening one. That's the opposite of the topic's headline `Vec::insert(0, _)` benchmark (~1300x, genuinely `O(n²)`), and the reason is structural: capping the collection at a fixed `capacity` via eviction also caps `Vec::remove(0)`'s per-call cost at `O(capacity)` — a constant — so total cost stays `O(n × capacity)`, linear in `n`, just with a worse constant factor than `VecDeque`'s `O(n)`. The unbounded `O(n²)` blowup only shows up when nothing evicts and the vector is allowed to grow all the way to `n` elements.
