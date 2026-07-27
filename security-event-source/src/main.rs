// 2.3 — Associated types vs generic parameters
// Exercise: Security Event Source
// Spec: see §4 of "2.3 Associated types vs generic parameters.md" in the vault .

use std::collections::VecDeque;

// SECTION 1: ASSOCIATED TYPE
trait EventSource {
    type Event;

    fn next_event(&mut self) -> Option<Self::Event>;
}

#[derive(Debug, PartialEq)]
struct SecurityEvent {
    source: String, // eg. ssh, api
    kind: String,   // eg. login_failure, privelege_escalation
    timestamp: u64,
}
#[derive(Debug)]
struct EventQueue {
    data: VecDeque<SecurityEvent>, // using VecDeque to pop events from front
}

impl EventSource for EventQueue {
    type Event = SecurityEvent;

    fn next_event(&mut self) -> Option<Self::Event> {
        self.data.pop_front()
    }
}
// Error: E0119, two conflicting Impls
// impl EventSource for EventQueue {
//     type Event = String;
//
//     fn next_event(&mut self) -> Option<String> {
//         unimplemented!()
//     }
// }

fn drain_all<T: EventSource>(source: &mut T) -> Vec<T::Event> {
    let mut v: Vec<T::Event> = vec![];
    while let Some(element) = source.next_event() {
        v.push(element);
    }
    v
}

#[test]
fn test_event_source_next_event_drains_fifo() {
    let mut queue = EventQueue {
        data: vec![
            SecurityEvent {
                source: "sshd".to_string(),
                kind: "login_failure".to_string(),
                timestamp: 1,
            },
            SecurityEvent {
                source: "auth-api".to_string(),
                kind: "privilege_escalation".to_string(),
                timestamp: 2,
            },
        ]
        .into(),
    };

    assert_eq!(
        queue.next_event(),
        Some(SecurityEvent {
            source: "sshd".to_string(),
            kind: "login_failure".to_string(),
            timestamp: 1,
        })
    );
    assert_eq!(
        queue.next_event(),
        Some(SecurityEvent {
            source: "auth-api".to_string(),
            kind: "privilege_escalation".to_string(),
            timestamp: 2,
        })
    );
    assert_eq!(queue.next_event(), None);
}

#[test]
fn test_drain_all_over_associated_type() {
    let mut queue = EventQueue {
        data: vec![
            SecurityEvent {
                source: "sshd".to_string(),
                kind: "login_failure".to_string(),
                timestamp: 1,
            },
            SecurityEvent {
                source: "auth-api".to_string(),
                kind: "privilege_escalation".to_string(),
                timestamp: 2,
            },
        ]
        .into(),
    };

    let v = drain_all(&mut queue);
    assert_eq!(v.len(), 2)
}

// SECTION 2: GENERIC
trait Decoder<T> {
    fn decode(&self) -> Option<T>;
}

fn decode_or_none<D, T: Decoder<D>>(d: &T) -> Option<D> {
    d.decode()
}

// input
struct RawStream {
    bytes: Vec<u8>,
}

// output
#[derive(Debug, PartialEq)]
struct RawFrame {
    bytes: Vec<u8>,
}
#[derive(Debug, PartialEq)]
struct ParsedCommand {
    name: String,
}

// Same generic Decoder trait on two distinct types
impl Decoder<RawFrame> for RawStream {
    fn decode(&self) -> Option<RawFrame> {
        if self.bytes.is_empty() {
            return None;
        }
        Some(RawFrame {
            bytes: self.bytes.clone(),
        })
    }
}
// Same generic Decoder trait on two distinct types
impl Decoder<ParsedCommand> for RawStream {
    fn decode(&self) -> Option<ParsedCommand> {
        if let Some(cmd) = String::from_utf8(self.bytes.clone()).ok() {
            return Some(ParsedCommand { name: cmd });
        }
        None
    }
}

#[test]
fn test_decoder_two_impls_direct_call() {
    let stream = RawStream {
        bytes: b"restart-service".to_vec(),
    };
    let frame: Option<RawFrame> = stream.decode(); // Same Generic decode() on two distinct types
    let cmd: Option<ParsedCommand> = stream.decode(); // Same Generic decode() on two distinct types

    assert_eq!(
        frame,
        Some(RawFrame {
            bytes: b"restart-service".to_vec()
        })
    );
    assert_eq!(
        cmd,
        Some(ParsedCommand {
            name: "restart-service".to_string()
        })
    );
}
#[test]
fn test_decode_or_none_generic_over_output_type() {
    let stream = RawStream {
        bytes: b"restart-service".to_vec(),
    };
    let frame: Option<RawFrame> = decode_or_none(&stream); // Same Generic decode() on two distinct types
    let cmd: Option<ParsedCommand> = decode_or_none(&stream); // Same Generic decode() on two distinct types

    assert_eq!(
        frame,
        Some(RawFrame {
            bytes: b"restart-service".to_vec()
        })
    );
    assert_eq!(
        cmd,
        Some(ParsedCommand {
            name: "restart-service".to_string()
        })
    );
}

// SECTION 3: ASSOCIATED TYPE + GENERICS

trait Lookup<Key> {
    type Value;
    fn lookup(&self, key: Key) -> Option<&Self::Value>;
}

impl Lookup<usize> for EventQueue {
    type Value = SecurityEvent;

    fn lookup(&self, key: usize) -> Option<&Self::Value> {
        self.data.get(key)
    }
}
impl Lookup<&str> for EventQueue {
    type Value = SecurityEvent;

    fn lookup(&self, key: &str) -> Option<&Self::Value> {
        self.data.iter().find(|e| e.source.contains(key))
    }
}
#[test]
fn test_lookup_index_and_str_key_coexist() {
    let queue = EventQueue {
        data: vec![
            SecurityEvent {
                source: "sshd".to_string(),
                kind: "login_failure".to_string(),
                timestamp: 1,
            },
            SecurityEvent {
                source: "auth-api".to_string(),
                kind: "privilege_escalation".to_string(),
                timestamp: 2,
            },
        ]
        .into(),
    };

    assert_eq!(
        queue.lookup("hd"),
        Some(&SecurityEvent {
            source: "sshd".to_string(),
            kind: "login_failure".to_string(),
            timestamp: 1,
        })
    );
    assert_eq!(
        queue.lookup(1),
        Some(&SecurityEvent {
            source: "auth-api".to_string(),
            kind: "privilege_escalation".to_string(),
            timestamp: 2,
        })
    );
}
fn main() {}
