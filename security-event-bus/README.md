# security-event-bus

Fan-in with `std::sync::mpsc` (many producer threads, one `Receiver`) contrasted against genuine multi-consumer work-stealing with `crossbeam_channel` — built around the one bug that actually bites people in production: forgetting that the *original* `Sender`, not just its clones, has to be dropped before the channel can close.

## What's in here

`collect_events_mpsc` spawns one thread per "sensor" source, each cloning a `Sender` and sending `events_per_source` `SecurityEvent`s; the original `tx` is explicitly `drop`ped after every clone is handed out, so `rx.iter().collect()` terminates cleanly once every sender is gone. `collect_events_mpsc_buggy_version` is structurally identical but never drops the original `tx` — instead of letting that hang the test suite forever, it uses `recv_timeout` and returns `(Vec<SecurityEvent>, RecvTimeoutError)`, so the test can assert on the *specific* error variant (`Timeout`, never `Disconnected`) rather than just "it eventually returned."

`process_events_crossbeam` swaps in `crossbeam_channel::unbounded`, whose `Receiver: Clone` lets multiple worker threads compete for messages off the *same* queue — something `std::sync::mpsc::Receiver` (`!Clone`) structurally cannot do. It returns `(Vec<SecurityEvent>, Vec<usize>)`, the second element being each worker's individual contribution count, so the test can assert that more than one worker actually did work — not just that the total came out right, which the earlier (buggy, sequential spawn-then-immediately-join) version of this function also satisfied while silently funneling 100% of the work onto a single thread.

