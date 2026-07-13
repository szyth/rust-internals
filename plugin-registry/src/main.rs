// 1.5 — 'static: What It Actually Guarantees
// Exercise: Type-Erased Plugin Registry
// Spec: see §4 of "1.5 'static - what it actually guarantees.md" in the notes vault.

trait Check {
    fn run(&self) -> bool;
    fn name(&self) -> &str;
}

struct Registry {
    // `Box<dyn Check>` with no lifetime written defaults to `Box<dyn Check + 'static>`
    // that's Rust's default trait-object lifetime bound for owned types like Box. This
    // field's default is why register<C: Check + 'static> needs that bound at all.
    checks: Vec<Box<dyn Check>>,
}
impl Registry {
    fn new() -> Self {
        Self { checks: vec![] }
    }
    fn register<C: Check + 'static>(&mut self, check: C) {
        self.checks.push(Box::new(check));
    }

    fn run_all(&self) {
        for check in self.checks.iter() {
            println!("{}: {}", check.name(), check.run());
            // 'static is not required, else fails to compile
            println!("{}", is_known_check(check.name()));
        }
    }
}

// Satisfies 'static through OWNERSHIP. no &'static reference anywhere inside
struct OwnedCheck {
    name: String,
}

impl Check for OwnedCheck {
    fn run(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        &self.name
    }
}
// Satisfies 'static through a genuine long-live REFERENCE
struct StaticStrCheck {
    name: &'static str,
}
impl Check for StaticStrCheck {
    fn run(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// DOES NOT satisfy 'static. Short-lived borrowed reference
struct BorrowedCheck<'name> {
    name: &'name str,
}
impl<'name> Check for BorrowedCheck<'name> {
    fn run(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        &self.name
    }
}
const KNOWN_CHECKS: &[&str] = &["owned-check", "static-str-check"];

fn is_known_check(name: &str) -> bool {
    KNOWN_CHECKS.contains(&name)
}

fn main() {
    let mut registry = Registry::new();

    // Owned String, created just before registering. Still satisfies 'static
    let owned_label = OwnedCheck {
        name: "owned-check".to_string(),
    };
    registry.register(owned_label);

    // &'static str literal. satisfies 'static via an actual long-lived reference
    let static_str_label = StaticStrCheck {
        name: "static-str-check",
    };
    registry.register(static_str_label);

    // borrowed string. DOES NOT satisfy 'static
    let heap_str = String::from("borrowed-str-check");
    let borrowed_str_label = BorrowedCheck {
        name: heap_str.as_str(),
    }; // non-static str
    // registry.register(borrowed_str_label); // error: `heap_str` does not live long enough
    registry.run_all();
}
