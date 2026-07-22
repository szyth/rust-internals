// 2.1 — Static dispatch (monomorphization) vs dynamic dispatch (trait objects)
// Exercise: Configurable Log Filter Pipeline
// Spec: see §4 of "2.1 Static dispatch (monomorphization) vs dynamic dispatch (trait objects).md" in the vault (topics/).

use std::sync::atomic::AtomicUsize;

trait LogFilter {
    fn accept(&self, line: &str) -> bool;

    // Error: Generics are not dyn compatible
    // fn accept_scored<T: Into<f64>>(&self, line: &str, weight: T) -> f64;

    // Fix 1: add Self: Sized in a default method
    fn accept_scored<T: Into<f64>>(&self, line: &str, weight: T) -> f64
    where
        Self: Sized,
    {
        if self.accept(line) {
            weight.into()
        } else {
            0.0
        }
    }

    // a default method that increments static counter
    fn increment_static(&self) -> usize {
        static COUNT: AtomicUsize = AtomicUsize::new(0);
        COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
    }
}

// Fix 2: add a super trait
trait ScoredLogFilter: LogFilter {
    fn accept_scored<T: Into<f64>>(&self, line: &str, weight: T) -> f64;
}

struct MinLengthFilter {
    min: usize,
}
impl LogFilter for MinLengthFilter {
    fn accept(&self, line: &str) -> bool {
        line.len() >= self.min
    }
}
struct KeywordFilter {
    keyword: String,
}
impl LogFilter for KeywordFilter {
    fn accept(&self, line: &str) -> bool {
        line.contains(&self.keyword)
    }
}

// static dispatch
fn count_matching<F: LogFilter>(filter: &F, lines: &[&str]) -> usize {
    lines.iter().filter(|line| filter.accept(line)).count()
}

// dyn dispatch
fn count_matching_dyn(filter: &dyn LogFilter, lines: &[&str]) -> usize {
    lines.iter().filter(|line| filter.accept(line)).count()
}
fn main() {
    let lines = [
        "ok",                      // 2 chars  — short
        "short",                   // 5 chars
        "a very long line here",   // 22 chars — long
        "error: disk full",        // 17 chars, contains "error"
        "system rebooted cleanly", // 24 chars, no "error"
    ];

    let min_filter = MinLengthFilter { min: 6 };
    let keyword_filter = KeywordFilter {
        keyword: "error".to_string(),
    };

    assert_eq!(count_matching(&min_filter, &lines), 3);
    assert_eq!(count_matching(&keyword_filter, &lines), 1);

    assert_eq!(count_matching_dyn(&min_filter, &lines), 3);
    assert_eq!(count_matching_dyn(&keyword_filter, &lines), 1);

    println!("Count: {}", min_filter.increment_static());
    println!("Count: {}", keyword_filter.increment_static());

    // Error: fails because dyn LogFilter isn't Sized
    // let filters: Vec<dyn LogFilter> = vec![min_filter, keyword_filter];

    // Vec<Box<dyn LogFilter>> each element is a different concrete type, individually Sized.
    // Box makes it uniform Sized with: {data_ptr, vtable_ptr}
    let filters: Vec<Box<dyn LogFilter>> = vec![Box::new(min_filter), Box::new(keyword_filter)];

    for filter in filters {
        count_matching_dyn(&*filter, &lines);
        // count_matching_dyn(filter.as_ref(), &lines); // both works.
    }
}

#[cfg(test)]
mod test {}
