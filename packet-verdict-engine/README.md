# packet-verdict-engine

A real `src/lib.rs` + `src/main.rs` split that ties together every mechanism from 3.10 in one build: an `@` binding, a match guard, exhaustiveness forced and then broken by evolving the enum, and `#[non_exhaustive]` blocking construction across the crate boundary.

## What's in here

`classify()` turns a `Packet` into a `PacketVerdict` — an `@` binding (`port @ 0..=1023`) catches privileged source ports as `Drop` while also capturing the port for logging, and a guard (`recent_hits > 100`) catches noisy sources as `Throttle`, falling through to `Allow`. `is_actionable()` matches every `PacketVerdict` variant explicitly, no wildcard — which is what let it catch a real gap when `PacketVerdict::Log` was added to the enum later (`E0004`, until a real arm was added). `RuleEngine` is `#[non_exhaustive]`, so `main.rs` — a genuinely separate crate root from `lib.rs`, even within one package — can't construct it via struct-literal syntax (`E0639`) and has to go through `RuleEngine::init()` instead.

