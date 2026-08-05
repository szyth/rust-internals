// 3.1 — Iterator trait: lazy adapters vs terminal consumers
// Exercise: Suspicious Log Line Scanner
// Spec: see §4 of "3.1 Iterator trait - lazy adapters vs terminal consumers.md" in the vault.
// Steps 1-5 complete.

fn make_log_line(i: u64) -> String {
    if i == 42 {
        // injecting suspicious entry at a known index
        return format!("request id={} status=500 ip=10.0.0.1", i);
    }
    format!("request id={} status=200 ip=10.0.0.1", i)
}

fn main() {
    // Full-drain consumer via collect()
    let mut inspected = 0;

    // Note: infinite range (0..) would also compile here, but the collect() would never return.
    // an infinite source with a full-drain consumer just hangs forever.
    let iter: Vec<_> = (0..1000_000)
        .map(|i| make_log_line(i))
        .inspect(|_| inspected += 1)
        .filter(|line| line.contains("status=500"))
        .collect();

    println!("{inspected}"); // prints 1000_000
    assert_eq!(inspected, 1000_000);

    // VERSUS

    // Short-circuiting consumer via find()
    let mut inspected = 0;
    let found = (0..) // infinite range
        .map(|i| make_log_line(i))
        .inspect(|_| inspected += 1)
        .find(|line| line.contains("status=500"));
    println!("inspected:{inspected} found: {found:?} "); // prints 43
    assert_eq!(inspected, 43);

    // #[must_use] guardrail
    // warning: unused `Map` that must be used
    // note: iterators are lazy and do nothing unless consumed
    // (0..10).map(|i| println!("{i}"));
}
