// 4.2 — Channels: mpsc, crossbeam, tokio
// Exercise: Security Event Bus

use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug)]
struct SecurityEvent {
    source: String,
    message: String,
}

fn collect_events_mpsc(sources: &[&str], events_per_source: usize) -> Vec<SecurityEvent> {
    let (tx, rx) = mpsc::channel::<SecurityEvent>();

    for source in sources {
        let source = source.to_string();
        let tx1 = tx.clone();
        thread::spawn(move || {
            for i in 0..events_per_source {
                let event = SecurityEvent {
                    source: source.clone(),
                    message: format!("Souce: {}, Event index: {}", source, i),
                };
                if let Err(e) = tx1.send(event) {
                    eprintln!("Failed to send: {:?}", e);
                }
            }
        });
    }
    std::mem::drop(tx);

    // let mut events = vec![];
    // while let Ok(event) = rx.recv() {
    //     events.push(event);
    // }
    // events

    // same as above but shorter
    rx.iter().collect::<Vec<SecurityEvent>>()
}
fn collect_events_mpsc_buggy_version(
    sources: &[&str],
    events_per_source: usize,
) -> (Vec<SecurityEvent>, mpsc::RecvTimeoutError) {
    // returning RecvTimeoutError to export the channel Timeout error due to never dropping the tx.
    // to be used later in the test assertion
    let (tx, rx) = mpsc::channel::<SecurityEvent>();

    for source in sources {
        let source = source.to_string();
        let tx1 = tx.clone();
        thread::spawn(move || {
            for i in 0..events_per_source {
                let event = SecurityEvent {
                    source: source.clone(),
                    message: format!("Souce: {}, Event index: {}", source, i),
                };
                if let Err(e) = tx1.send(event) {
                    eprintln!("Failed to send: {:?}", e);
                }
            }
        });
    }
    // tx never dropped
    // rx.clone(); // error[E0599]: no method named `clone` found for Receiver

    let mut events = vec![];
    // this loop will always return Err: Timeout or Disconnected
    let err = loop {
        match rx.recv_timeout(Duration::from_millis(400)) {
            Ok(event) => events.push(event),
            Err(err) => break err,
        }
    };

    (events, err)
}
// work-stealing multi-worker distribution
fn process_events_crossbeam(
    sources: &[&str],
    events_per_source: usize,
    num_workers: usize,
) -> (Vec<SecurityEvent>, Vec<usize>) {
    // returning Vec<usize> to export the events processed
    // per-thread for the assertion later in test
    let (tx, rx) = crossbeam_channel::unbounded::<SecurityEvent>();

    for source in sources {
        let source = source.to_string();
        let tx1 = tx.clone();
        thread::spawn(move || {
            for i in 0..events_per_source {
                let event = SecurityEvent {
                    source: source.clone(),
                    message: format!("Souce: {}, Event index: {}", source, i),
                };
                if let Err(e) = tx1.send(event) {
                    eprintln!("Failed to send: {:?}", e);
                }
            }
        });
    }
    std::mem::drop(tx);

    let mut events = vec![];
    let mut handles = vec![];
    let mut per_thread_events_count = vec![];
    for _i in 0..num_workers {
        let rx1 = rx.clone();
        let handle = thread::spawn(move || {
            let mut events = vec![];
            while let Ok(event) = rx1.recv() {
                events.push(event);
            }
            return events;
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Ok(ev) = handle.join() {
            per_thread_events_count.push(ev.len());
            events.extend(ev);
        }
    }
    println!("{:?}", per_thread_events_count);
    (events, per_thread_events_count)
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_collect_events_mpsc() {
        let sources = ["auth-service", "firewall", "ids"];
        let events_per_source = 5;

        let events = collect_events_mpsc(&sources, events_per_source);

        assert_eq!(events.len(), sources.len() * events_per_source);
    }
    #[test]
    fn test_collect_events_mpsc_buggy_version() {
        let sources = ["auth-service", "firewall", "ids"];
        let events_per_source = 5;

        let (events, err) = collect_events_mpsc_buggy_version(&sources, events_per_source);
        assert_eq!(events.len(), sources.len() * events_per_source);
        assert_eq!(err, mpsc::RecvTimeoutError::Timeout)
    }
    #[test]
    fn test_process_events_crossbeam() {
        let sources = ["auth-service", "firewall", "ids"];
        let events_per_source = 5;

        let (events, per_thread_events_count) =
            process_events_crossbeam(&sources, events_per_source, 3);
        assert_eq!(events.len(), sources.len() * events_per_source);

        // work-stealing should assert that more than one threads got work to do;
        // checking with the events-per-thread count > 1
        assert!(
            per_thread_events_count
                .iter()
                .filter(|&&event| event != 0)
                .count()
                > 1
        );
    }
}
