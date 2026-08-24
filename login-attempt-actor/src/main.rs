// 4.2 — Channels: mpsc, crossbeam, tokio
// Exercise: tokio::sync Actor Pattern

use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

// SAME IMPLEMENTATION WITH ARC MUTEX, BUT...
// introduces LOCKING, not efficient as compared to below lock-free no-race channel impl

// #[derive(Clone)]
// struct TrackerHandle {
//     counts: Arc<Mutex<HashMap<String, u32>>>,
// }
//
// impl TrackerHandle {
//     async fn record_failure(&self, user: &str) -> u32 {
//         let mut counts = self.counts.lock().unwrap(); // <- blocks other callers here
//         let count = counts.entry(user.to_string()).or_insert(0);
//         *count += 1;
//         *count
//     }
//
//     async fn is_locked_out(&self, user: &str, threshold: u32) -> bool {
//         let counts = self.counts.lock().unwrap();
//         counts.get(user).copied().unwrap_or(0) >= threshold
//     }
// }

enum Command {
    RecordFailure {
        user: String,
        respond_to: oneshot::Sender<u32>,
    },
    IsLockedOut {
        user: String,
        threshold: u32,
        respond_to: oneshot::Sender<bool>,
    },
}

// Records the User's failed login attempts in RecordFailure, and also tells if the user is
// lockedOut due to multiple failed attemps
async fn run_actor(mut rx: mpsc::Receiver<Command>) {
    let mut counts: HashMap<String, u32> = HashMap::new();
    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::RecordFailure { user, respond_to } => {
                let count = counts.entry(user).or_insert(0);
                *count += 1;
                let _res = respond_to.send(*count);
            }
            Command::IsLockedOut {
                user,
                threshold,
                respond_to,
            } => {
                if *counts.get(&user).unwrap_or(&0) >= threshold {
                    let _res = respond_to.send(true);
                } else {
                    let _res = respond_to.send(false);
                }
            }
        }
    }
}

#[derive(Clone)]
struct TrackerHandle {
    tx: mpsc::Sender<Command>,
}

impl TrackerHandle {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Command>(1000);
        let _handle = tokio::spawn(async {
            run_actor(rx).await;
        });
        Self { tx: tx }
    }

    async fn record_failure(&self, user: &str) -> Result<u32, oneshot::error::RecvError> {
        let (respond_to, response) = oneshot::channel::<u32>();
        let _res = self
            .tx
            .send(Command::RecordFailure {
                user: user.to_string(),
                respond_to,
            })
            .await;

        response.await
    }

    async fn is_locked_out(
        &self,
        user: &str,
        threshold: u32,
    ) -> Result<bool, oneshot::error::RecvError> {
        let (respond_to, response) = oneshot::channel::<bool>();
        let _res = self
            .tx
            .send(Command::IsLockedOut {
                user: user.to_string(),
                threshold,
                respond_to,
            })
            .await;

        response.await
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_no_race_with_lockfree_channel() {
        let tracker = TrackerHandle::new();
        let mut handles = vec![];
        for _ in 0..5 {
            let cloned_tracker = tracker.clone();
            let handle =
                tokio::spawn(async move { cloned_tracker.record_failure("alice").await.unwrap() });
            handles.push(handle)
        }

        let mut output = vec![];
        for handle in handles {
            output.push(handle.await.unwrap());
        }

        assert_eq!([1, 2, 3, 4, 5].to_vec(), output)
    }

    #[tokio::test]
    async fn test_lockout_flow_for_different_users() {
        let tracker = TrackerHandle::new();

        let alice_tracker = tracker.clone();
        let bob_tracker = tracker.clone();

        assert_eq!(
            false,
            alice_tracker.is_locked_out("alice", 3).await.unwrap()
        );
        for _ in 0..3 {
            let _ = alice_tracker.record_failure("alice").await.unwrap();
        }
        assert_eq!(true, alice_tracker.is_locked_out("alice", 3).await.unwrap());
        assert_eq!(false, bob_tracker.is_locked_out("bob", 3).await.unwrap());
    }
}
