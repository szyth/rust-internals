// 2.5 — Marker traits: Send, Sync, Sized, Unpin, Copy
// Exercise: Thread-Bound vs. Shareable Credential Handle
// Spec: see §4 of "2.5 Marker traits - Send, Sync, Sized, Unpin, Copy.md" in the vault.
// Steps 1-5 complete.

use std::{marker::PhantomData, rc::Rc, sync::Arc};

struct ThreadBoundHandle {
    credential: Rc<String>,
}
struct SharedHandle {
    credential: Arc<String>,
}
struct PartiallyThreadBoundHandle {
    credential: Arc<String>,
    _marker: PhantomData<Rc<()>>,
}
struct ManualSendSyncHandle {
    credential: Arc<String>,
    _marker: PhantomData<Rc<()>>,
}
// SAFETY: the type does not contain anything that is thread-bound, credential Arc<> is Send-Sync while PhantomData holds no value at runtime
unsafe impl Send for ManualSendSyncHandle {}
// SAFETY: the type does not contain anything that is thread-bound, credential Arc<> is Send-Sync while PhantomData holds no value at runtime
unsafe impl Sync for ManualSendSyncHandle {}

impl SharedHandle {
    fn new() -> Self {
        Self {
            credential: Arc::new(String::from("secret")),
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use std::thread;

    use crate::{
        ManualSendSyncHandle, PartiallyThreadBoundHandle, SharedHandle, ThreadBoundHandle,
    };

    #[test]
    fn test_thread_bound_handle_is_not_send() {
        fn assert_send<T: Send>() {}

        // error[E0277] `Rc<String>` cannot be sent between threads safely, the trait Send isn't implemented for Rc<String>
        // assert_send::<ThreadBoundHandle>();
    }
    #[test]
    fn test_shared_handle_is_send_sync() {
        let shared = SharedHandle::new();
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<SharedHandle>();
        assert_sync::<SharedHandle>();

        let handle = thread::spawn(move || {
            // move whole value inside thread
            let shared = shared;

            shared.credential.len()
        });
        let len = handle.join().unwrap(); // blocks until the spawned thread finishes, propagates a panic if it panicked

        assert_eq!(len, 6)
    }
    #[test]
    fn test_partial_thread_bound_handle_is_not_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        // error[E0277] `Rc<()>` cannot be sent between threads safely, the trait Send isn't implemented for Rc<()>
        // assert_send::<PartiallyThreadBoundHandle>();
        // error[E0277] `Rc<()>` cannot be sent between threads safely, the trait Sync isn't implemented for Rc<()>
        // assert_sync::<PartiallyThreadBoundHandle>();
    }

    #[test]
    fn test_handle_made_send_sync_with_unsafe_impl() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<ManualSendSyncHandle>();
        assert_sync::<ManualSendSyncHandle>();
    }
}
