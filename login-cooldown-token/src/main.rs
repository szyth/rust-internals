// Topic 4.7 — async/await desugaring into state machines
// Exercise: authenticate_and_issue_token
// Spec: see §4 of "4.7 async-await desugaring into state machines.md" in the vault.

///////////////// DESUGARING THE FOLLOWING: //////////////////
// async fn authenticate_and_issue_token(username: String, password_hash: u64, cooldown: Cooldown) -> Result<String, AuthError> {
//     verify_password(&username, password_hash)?;
//     println!("password ok for {username}, waiting out cooldown");
//     cooldown.await;
//     Ok(format!("{username} may now retry"))
// }
///////////////////////////////////////////////////////////////
use std::{
    pin::Pin,
    sync::{Arc, atomic::AtomicBool},
    task::{Context, Poll, Wake},
    thread,
    time::Duration,
};

#[derive(Debug, PartialEq)]
enum AuthError {
    InvalidCredentials,
}

// NOTE: this is just a stand-in function and does not represent any real Password Verification
// logic
fn verify_password(username: &str, password_hash: u64) -> Result<(), AuthError> {
    // any random check just to return Err()
    if password_hash < 10 {
        return Err(AuthError::InvalidCredentials);
    }
    Ok(())
}

struct Cooldown {
    started: bool,
    is_complete: Arc<AtomicBool>,
    duration: Duration,
}

impl Future for Cooldown {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.is_complete.load(std::sync::atomic::Ordering::Acquire) {
            return Poll::Ready(());
        }

        if !self.started {
            let duration = self.duration;
            let waker = cx.waker().clone();
            let clone_is_complete = Arc::clone(&self.is_complete);
            thread::spawn(move || {
                thread::sleep(duration); // simulate an IO call for a cooldown period
                clone_is_complete.store(true, std::sync::atomic::Ordering::Release);
                waker.wake();
            });
            self.started = true;
        }

        Poll::Pending
    }
}

/////////// HAND-ROLLED STATE MACHINE //////////
enum AuthenticateAndIssueToken {
    Start {
        username: String,
        password_hash: u64,
        cooldown: Cooldown,
    },
    WaitingOnCooldown {
        username: String,
        cooldown: Cooldown,
    },
    Finished,
}

impl Future for AuthenticateAndIssueToken {
    type Output = Result<String, AuthError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match std::mem::replace(&mut *self, AuthenticateAndIssueToken::Finished) {
                AuthenticateAndIssueToken::Start {
                    username,
                    password_hash,
                    cooldown,
                } => {
                    // spec: step 1
                    if let Err(e) = verify_password(&username, password_hash) {
                        return Poll::Ready(Err(e));
                    }
                    println!("password ok for {username}, waiting out cooldown"); // spec: step 2
                    self.set(AuthenticateAndIssueToken::WaitingOnCooldown { username, cooldown });
                }
                AuthenticateAndIssueToken::WaitingOnCooldown {
                    username,
                    mut cooldown,
                } => {
                    // spec: step 3
                    // poll the inner-future
                    let cooldown_pin = Pin::new(&mut cooldown);
                    match cooldown_pin.poll(cx) {
                        Poll::Ready(_v) => {
                            return Poll::Ready(Ok(format!("{username} may now retry"))); // spec: step 4
                        }
                        Poll::Pending => {
                            self.set(AuthenticateAndIssueToken::WaitingOnCooldown {
                                username,
                                cooldown,
                            });
                            return Poll::Pending;
                        }
                    }
                }
                AuthenticateAndIssueToken::Finished => panic!("polled after completion"),
            }
        }
    }
}

/////////// HAND-ROLLED EXECUTOR //////////
struct ThreadWaker(thread::Thread);

impl Wake for ThreadWaker {
    fn wake(self: std::sync::Arc<Self>) {
        self.0.unpark();
    }
}

// drive the future to completion
fn block_on<T>(fut: Pin<&mut impl Future<Output = T>>) -> (T, i32) {
    let mut pin = std::pin::pin!(fut);

    let thread_waker = ThreadWaker(std::thread::current());
    let waker = Arc::new(thread_waker).into();
    let mut cx = Context::from_waker(&waker);

    let mut count_total_polls_for_assertion_later = 0;
    loop {
        count_total_polls_for_assertion_later += 1;
        match Pin::poll(pin.as_mut(), &mut cx) {
            Poll::Ready(output) => return (output, count_total_polls_for_assertion_later),
            Poll::Pending => std::thread::park(), // Blocks the thread until awoken, a real
                                                  // executor will run another future from the queue rather than sleep
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn valid_credentials_resolve_after_cooldown_in_two_polls() {
        let cooldown = Cooldown {
            started: false,
            is_complete: Arc::new(AtomicBool::new(false)),
            duration: Duration::from_secs(2),
        };
        let auth_and_issue_token = AuthenticateAndIssueToken::Start {
            username: "alice".to_string(),
            password_hash: 13,
            cooldown,
        };
        let fut = std::pin::pin!(auth_and_issue_token);
        let (result, poll_count) = block_on(fut);
        assert_eq!(poll_count, 2);
        assert_eq!(result, Ok(format!("alice may now retry")));
    }
    #[test]
    fn invalid_credentials_short_circuit_in_one_poll_before_cooldown() {
        let cooldown = Cooldown {
            started: false,
            is_complete: Arc::new(AtomicBool::new(false)),
            duration: Duration::from_secs(2),
        };
        let auth_and_issue_token = AuthenticateAndIssueToken::Start {
            username: "bob".to_string(),
            password_hash: 1,
            cooldown,
        };
        let fut = std::pin::pin!(auth_and_issue_token);
        let (result, poll_count) = block_on(fut);
        assert_eq!(poll_count, 1);
        assert_eq!(result, Err(AuthError::InvalidCredentials));
    }
}
