// Topic 4.6 — Future trait & the poll-based execution model
// Exercise: FlushSignal
// Spec: see §4 of "4.6 Future trait and the poll-based execution model.md" in the vault.

use std::{
    pin::Pin,
    sync::{Arc, atomic::AtomicBool},
    task::{Context, Poll, Wake},
    time::Duration,
};

struct FlushSignal {
    is_complete: Arc<AtomicBool>,
    started: bool,
    duration: Duration,
}

impl FlushSignal {
    fn new(duration: Duration) -> Self {
        Self {
            is_complete: Arc::new(AtomicBool::new(false)),
            started: false,
            duration,
        }
    }
}

impl Future for FlushSignal {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.is_complete.load(std::sync::atomic::Ordering::Acquire) {
            return Poll::Ready(());
        }

        if !self.started {
            let cloned_is_complete = self.is_complete.clone();
            let cloned_duration = self.duration;
            let waker = cx.waker().clone();
            std::thread::spawn(move || {
                // simulating the IO work by sleeping:
                std::thread::sleep(cloned_duration);
                // mark as finished:
                cloned_is_complete.store(true, std::sync::atomic::Ordering::Release);
                // wake up the sleeping parked thread:
                waker.wake();
            });
            self.started = true;
        }

        Poll::Pending
    }
}
struct BrokenFlushSignal {
    is_complete: Arc<AtomicBool>,
    started: bool,
    duration: Duration,
}

impl BrokenFlushSignal {
    fn new(duration: Duration) -> Self {
        Self {
            is_complete: Arc::new(AtomicBool::new(false)),
            started: false,
            duration,
        }
    }
}

impl Future for BrokenFlushSignal {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.is_complete.load(std::sync::atomic::Ordering::Acquire) {
            return Poll::Ready(());
        }

        if !self.started {
            let cloned_is_complete = self.is_complete.clone();
            let cloned_duration = self.duration;
            std::thread::spawn(move || {
                // simulating the IO work by sleeping:
                std::thread::sleep(cloned_duration);
                // mark as finished:
                cloned_is_complete.store(true, std::sync::atomic::Ordering::Release);

                // no waker here
                // returning Pending obliges us to clone cx.waker() and call .wake() on it once progress is possible
                // this closure never does either, which is exactly why it's missing here."
            });
            self.started = true;
        }

        Poll::Pending
    }
}

struct ThreadWaker(std::thread::Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark(); // Awake the parked thread
    }
}

// Hand-build executor
// drive the future to completion, blocks if Poll::Pending
fn block_on<T>(mut fut: Pin<&mut impl Future<Output = T>>) -> (T, i32) {
    let thread_waker = ThreadWaker(std::thread::current());
    let waker = Arc::new(thread_waker).into();
    let mut cx = Context::from_waker(&waker);

    let mut count_total_polls_for_assertion_later = 0;
    loop {
        count_total_polls_for_assertion_later += 1;
        match Future::poll(fut.as_mut(), &mut cx) {
            Poll::Ready(output) => return (output, count_total_polls_for_assertion_later),
            Poll::Pending => std::thread::park(), // Blocks the thread until awoken, a real
                                                  // executor will run another future from the queue rather than sleep
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use std::{sync::mpsc, time::Instant};

    use super::*;
    #[test]
    fn waits_the_full_duration_before_resolving() {
        let io_task_duration = Duration::from_secs(3);
        let flush = FlushSignal::new(io_task_duration);
        let pin = std::pin::pin!(flush);
        let now = Instant::now();
        let _ = block_on(pin);

        assert!(now.elapsed() >= io_task_duration);
    }

    #[test]
    fn polls_exactly_twice_not_spin_polled() {
        let io_task_duration = Duration::from_secs(3);
        let flush = FlushSignal::new(io_task_duration);
        let pin = std::pin::pin!(flush);
        let ((), count_polls) = block_on(pin);

        assert_eq!(count_polls, 2); // block_on only calls poll 2 times, not a busy spin-poll
    }

    #[test]
    fn broken_future_hangs_forever_without_a_waker() {
        let broken = BrokenFlushSignal::new(Duration::from_secs(3));

        let (tx, rx) = mpsc::channel::<bool>();
        std::thread::spawn(move || {
            let pin = std::pin::pin!(broken);
            let _res = block_on(pin); // this blocks and never returns
            let _res = tx.send(true);
        });

        match rx.recv_timeout(Duration::from_secs(4)) {
            Err(mpsc::RecvTimeoutError::Timeout) => assert!(true),
            _ => assert!(
                false,
                "block_on should always be parked due to no waker so the rx should timeout"
            ),
        }
    }
}
