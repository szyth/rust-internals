// 3.4 — Vec/VecDeque internals & complexity trade-offs
// Exercise: Rate-Limiter Sliding Window
// Spec: see §4 of "3.4 Vec, VecDeque internals and complexity tradeoffs.md" in the vault.
// Steps 1-4 complete.

use std::collections::VecDeque;
use std::time::Instant;

struct RateLimiter {
    timestamps: VecDeque<u64>,
    capacity: usize,
    window_ticks: u64,
}

impl RateLimiter {
    fn new(capacity: usize, window_ticks: u64) -> Self {
        Self {
            timestamps: VecDeque::new(),
            capacity,
            window_ticks,
        }
    }
    fn record_attempt(&mut self, now: u64) -> bool {
        while let Some(timestamp) = self.timestamps.front()
            && *timestamp < now.saturating_sub(self.window_ticks)
        {
            self.timestamps.pop_front();
        }
        if self.capacity == self.timestamps.len() {
            return false;
        }
        self.timestamps.push_back(now);
        true
    }
}
struct RateLimiterVecBacked {
    timestamps: Vec<u64>,
    capacity: usize,
    window_ticks: u64,
}
impl RateLimiterVecBacked {
    fn new(capacity: usize, window_ticks: u64) -> Self {
        Self {
            timestamps: Vec::new(),
            capacity,
            window_ticks,
        }
    }
    fn record_attempt(&mut self, now: u64) -> bool {
        while let Some(timestamp) = self.timestamps.first()
            && *timestamp < now.saturating_sub(self.window_ticks)
        {
            self.timestamps.remove(0);
        }
        if self.capacity == self.timestamps.len() {
            return false;
        }
        self.timestamps.push(now);
        true
    }
}

fn main() {
    // capacity 3, window 100 ticks
    let mut limiter = RateLimiter::new(3, 100);
    assert!(limiter.record_attempt(0));
    assert!(limiter.record_attempt(10));
    assert!(limiter.record_attempt(20));
    assert!(!limiter.record_attempt(30)); // capacity reached, nothing expired yet

    // boundary: a timestamp exactly `window_ticks` old is NOT expired (strict `<`)
    assert!(!limiter.record_attempt(100)); // threshold = 100-100=0; front(0) is NOT < 0, stays

    // now push past the window -- 0, 10, 20 all age out (threshold = 150-100=50)
    assert!(limiter.record_attempt(150));

    let mut vec_limiter = RateLimiterVecBacked::new(3, 100);
    assert!(vec_limiter.record_attempt(0));
    assert!(vec_limiter.record_attempt(10));
    assert!(vec_limiter.record_attempt(20));
    assert!(!vec_limiter.record_attempt(30));
    assert!(!vec_limiter.record_attempt(100));
    assert!(vec_limiter.record_attempt(150));

    println!("all assertions passed");

    bench(5_000);
    bench(50_000);
}

fn bench(n: u64) {
    // capacity 50, window 10 ticks -- window << n, so most attempts trigger an eviction
    let mut deque_limiter = RateLimiter::new(50, 10);
    let start = Instant::now();
    for now in 0..n {
        deque_limiter.record_attempt(now);
    }
    let deque_time = start.elapsed();

    let mut vec_limiter = RateLimiterVecBacked::new(50, 10);
    let start = Instant::now();
    for now in 0..n {
        vec_limiter.record_attempt(now);
    }
    let vec_time = start.elapsed();

    println!(
        "n={n}: VecDeque-backed {deque_time:?} vs Vec-backed {vec_time:?} ({:.2}x)",
        vec_time.as_secs_f64() / deque_time.as_secs_f64()
    );
}
