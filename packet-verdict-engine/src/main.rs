// 3.10 — Pattern matching: guards, bindings, exhaustiveness
// Exercise: Firewall Rule Engine (Library + Binary)
// Spec: see §4 of "3.10 Pattern matching - guards, bindings, exhaustiveness.md" in the vault.
// Steps 1-7 complete.

use packet_verdict_engine::RuleEngine;

fn main() {
    // with #[non_exhaustive], callers cannot construct the struct directly
    // and are forced to use available impl methods on that struct (similar to newtype)
    // error[E0639]: cannot create non-exhaustive struct using struct expression
    // let rule = RuleEngine { rules_loaded: 5 };

    // Works now,
    let _rule = RuleEngine::init(5);
}

#[cfg(test)]
mod test {
    use packet_verdict_engine::{
        Packet, PacketVerdict, RETRY_THRESHOLD, RuleEngine, classify, is_actionable,
    };

    #[test]
    fn test_privileged_port_is_dropped() {
        let pkt = Packet {
            src_port: 22,
            payload_len: 60,
        };
        assert!(matches!(
            classify(&pkt, 0),
            PacketVerdict::Drop {
                reason: "privileged src port",
            },
        ));
    }

    #[test]
    fn test_noisy_source_is_throttled() {
        let pkt = Packet {
            src_port: 8080,
            payload_len: 60,
        };
        assert!(matches!(
            classify(&pkt, 500),
            PacketVerdict::Throttle {
                retry_after_ms: RETRY_THRESHOLD
            }
        ));
    }
    #[test]
    fn test_normal_packet_is_allowed() {
        let pkt = Packet {
            src_port: 6436,
            payload_len: 60,
        };
        assert!(matches!(classify(&pkt, 0), PacketVerdict::Allow));
    }
    #[test]
    fn test_is_actionable_covers_all_verdicts() {
        assert_eq!(is_actionable(&PacketVerdict::Allow,), false);
        assert_eq!(
            is_actionable(&PacketVerdict::Throttle {
                retry_after_ms: 100
            },),
            true
        );
        assert_eq!(is_actionable(&PacketVerdict::Drop { reason: ".." },), true);
        assert_eq!(is_actionable(&PacketVerdict::Log { reason: ".." },), true);
    }
    #[test]
    fn test_rule_engine_init_succeeds() {
        let rule = RuleEngine::init(5);

        assert_eq!(rule.rules_loaded, 5)
    }
}
