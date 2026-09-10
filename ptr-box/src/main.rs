// Topic 4.5 — Send/Sync internals
// Exercise: PtrBox
// Spec: see §4 of "4.5 Send-Sync internals.md" in the vault.

use std::rc::Rc;

struct PtrBox<T> {
    ptr: *mut T,
}

impl<T> PtrBox<T> {
    fn new(value: T) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(value)),
        }
    }

    fn get(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> Drop for PtrBox<T> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.ptr) };
    }
}

// SAFETY:
// PtrBox has a single ownership (No Rc-style shared ownership) and Drop runs only once (no
// double-free)
// Moving a PtrBox<T> to another thread is exactly as sound as moving a Box<T> would be, which
// requires nothing more than T: Send.
unsafe impl<T: Send> Send for PtrBox<T> {}

// SAFETY:
// get() hands out &T from &PtrBox, so sharing a PtrBox<T> across threads is exactly as sound as
// sharing a &T would be, which requires T: Sync
unsafe impl<T: Sync> Sync for PtrBox<T> {}

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

struct UnsoundPtrBox<T> {
    ptr: *mut T,
}
// No T:Send bound on T. The unsound version of PtrBox
unsafe impl<T> Send for UnsoundPtrBox<T> {}

impl<T> UnsoundPtrBox<T> {
    fn new(value: T) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(value)),
        }
    }
    fn get(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
impl<T> Drop for UnsoundPtrBox<T> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.ptr) };
    }
}

fn main() {
    // Errors E0277 pre-Send, Sync implementation
    assert_send::<PtrBox<i32>>();
    assert_sync::<PtrBox<i32>>();

    // assert_send::<PtrBox<Rc<i32>>>(); // error[E0277]: Rc is not Send
    assert_send::<UnsoundPtrBox<std::rc::Rc<i32>>>();

    let ptr_box = PtrBox::new(4);
    let _res = std::thread::spawn(|| {
        let moved = ptr_box; // Can be moved; thread-safe
        drop(moved)
    })
    .join();

    // UNSOUND TERRITORY AHEAD
    let rc_value = Rc::new(5);
    let cloned_value = Rc::clone(&rc_value);
    let ptr1 = UnsoundPtrBox::new(rc_value);
    let ptr2 = UnsoundPtrBox::new(cloned_value);

    let mut handles = vec![];

    // 2 threads below are doing for Rc:
    // One refcount increment -> at clone()
    // Two refcount decrement -> two drop()
    // But we know that refcount of Rc is non-atomic and so there is data race but our
    // explicit impl Send for UnsoundPtrBox without T:Send bound bypassed that check
    let handle = std::thread::spawn(move || {
        let moved = ptr1;
        let clone = moved.get().clone();

        drop(moved);
        drop(clone);
    });
    handles.push(handle);
    let handle = std::thread::spawn(move || {
        let moved = ptr2;
        let clone = moved.get().clone();

        drop(moved);
        drop(clone);
    });
    handles.push(handle);

    for handle in handles {
        let res = handle.join();
    }
}
