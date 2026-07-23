// 2.2 — Trait bounds, where clauses, supertraits, default methods
// Exercise: Security Event Audit Trail
// Spec: see §4 of "2.2 Trait bounds, where clauses, supertraits, default methods.md" in the vault.
// Steps 1-5 complete. Step 6 (ambiguous-method stretch goal) skipped — unrelated to this domain.

trait Describable {
    fn describe(&self) -> String;
}

trait AuditableEvent: Describable {
    fn severity(&self) -> u8;
    // default method
    fn log_entry(&self) -> String {
        // using describe() of supertrait
        format!("[sev {}] {}", self.severity(), self.describe())
    }
}

// struct TestStruct;
// impl AuditableEvent for TestStruct {}
// ERROR [E0277]: Need to impl Supertrait Describable first
// only then can impl Subtrait AuditableEvent

struct NormalLogEvent {
    username: String,
    action: String,
    timestamp: u8,
    severity: u8,
}

impl Describable for NormalLogEvent {
    fn describe(&self) -> String {
        format!("{}:{}:{}", self.username, self.action, self.timestamp)
    }
}

impl AuditableEvent for NormalLogEvent {
    fn severity(&self) -> u8 {
        self.severity
    }
}

struct CriticalLogEvent {
    username: String,
    action: String,
    timestamp: u8,
    severity: u8,
}

impl Describable for CriticalLogEvent {
    fn describe(&self) -> String {
        format!("{}:{}:{}", self.username, self.action, self.timestamp)
    }
}

impl AuditableEvent for CriticalLogEvent {
    fn severity(&self) -> u8 {
        self.severity
    }

    // override default method
    fn log_entry(&self) -> String {
        format!(
            "***********[sev {}] {} ****************",
            self.severity(),
            self.describe()
        )
    }
}

// No AuditableEvent impl for TestStruct2
struct TestStruct2 {
    username: String,
    action: String,
    timestamp: u8,
    severity: u8,
}

impl Describable for TestStruct2 {
    fn describe(&self) -> String {
        format!("{}:{}:{}", self.username, self.action, self.timestamp)
    }
}

fn log_all<T>(events: &[&T])
where
    T: AuditableEvent,
{
    for event in events {
        println!("{}", event.log_entry());
    }
}

fn summarize<T>(events: &[&T])
where
    T: AuditableEvent,
{
    for event in events {
        // can use describe() method which was for Describable trait
        // because AuditableEvent: Describable
        println!("severity: {}....{}", event.severity(), event.describe())
    }
}

fn main() {
    let normal_event = NormalLogEvent {
        username: "alice".to_string(),
        action: "failed to login".to_string(),
        timestamp: 123,
        severity: 4,
    };
    let critical_event = CriticalLogEvent {
        username: "bob".to_string(),
        action: "escalated to admin".to_string(),
        timestamp: 128,
        severity: 1,
    };

    log_all(&vec![&normal_event]);
    log_all(&vec![&critical_event]);
    summarize(&vec![&critical_event]);

    let test_event = TestStruct2 {
        username: "alice".to_string(),
        action: "failed to login".to_string(),
        timestamp: 123,
        severity: 4,
    };

    // ERROR: E0599. No method severity() found. AuditableEvent not implemented
    // println!("{}", test_event.severity());
}
