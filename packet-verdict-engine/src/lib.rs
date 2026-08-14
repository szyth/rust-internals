// 3.10 — Pattern matching: guards, bindings, exhaustiveness
// Exercise: Firewall Rule Engine (Library + Binary)
// Spec: see §4 of "3.10 Pattern matching - guards, bindings, exhaustiveness.md" in the vault.
// Steps 1-7 complete.

pub enum PacketVerdict {
    Allow,
    Throttle { retry_after_ms: u32 },
    Drop { reason: &'static str },
    Log { reason: &'static str },
}
pub const RETRY_THRESHOLD: u32 = 500;

pub struct Packet {
    pub src_port: u16,
    pub payload_len: usize,
}

pub fn classify(pkt: &Packet, recent_hits: u32) -> PacketVerdict {
    match pkt {
        Packet {
            src_port: port @ 0..=1023,
            ..
        } => {
            eprintln!("blocked privileged src port {port}");
            PacketVerdict::Drop {
                reason: "privileged src port",
            }
        }
        Packet { .. } if recent_hits > 100 => PacketVerdict::Throttle {
            retry_after_ms: RETRY_THRESHOLD,
        },
        _ => PacketVerdict::Allow,
    }
}

// return a bool after the packet is classified
// `Allow` means no action is required and the packet good to go.
// `Throttle` and `Drop` requires further action from the caller
pub fn is_actionable(verdict: &PacketVerdict) -> bool {
    match verdict {
        PacketVerdict::Allow => false,
        PacketVerdict::Throttle { .. } => true,
        PacketVerdict::Drop { .. } => true,
        PacketVerdict::Log { .. } => true,
    }
}

#[non_exhaustive]
pub struct RuleEngine {
    pub rules_loaded: u32,
}

impl RuleEngine {
    pub fn init(rules_loaded: u32) -> Self {
        Self { rules_loaded }
    }
}
