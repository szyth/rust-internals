use std::cell::{Cell, UnsafeCell};

pub struct LazyCell<T, F = fn() -> T> {
    value: UnsafeCell<Option<T>>,
    init: Cell<Option<F>>,
}

impl<T, F: FnOnce() -> T> LazyCell<T, F> {
    pub fn new(init: F) -> Self {
        Self {
            value: UnsafeCell::new(None),
            init: Cell::new(Some(init)),
        }
    }

    pub fn get(&self) -> &T {
        // SAFETY:
        // no &mut alias to `value` exists; we hold no other reference
        // into the UnsafeCell at this point. Read-only check.
        if unsafe { &*self.value.get() }.is_none() {
            // narrow unsafe to the raw pointer deref only;
            // is_none() is safe and belongs outside the unsafe block
            // not prefered: unsafe { (*self.value.get()).is_none() }
            let f = self
                .init
                .take()
                .expect("LazyCell: reentrant initialisation");
            let val = f();
            // SAFETY:
            // no &T into `value` is live (the is_none() reference was a temporary,
            // dropped before this line). Reaching this line proves self.init.take()
            // succeeded; had init already been None, expect() would have panicked
            // and we would not be here. Therefore this write executes at most once.
            // Single-threaded: no concurrent access.
            unsafe { *self.value.get() = Some(val) };
        }

        // SAFETY:
        // value is Some; either it was already Some (if-branch skipped) or we just wrote Some(val)
        // above. The returned &T has a lifetime tied to &self; the UnsafeCell owns the T for the
        // lifetime of LazyCell. No further writes to `value` are possible: init is now None.
        let value = unsafe { &*self.value.get() }.as_ref().unwrap(); // cleaner and lesser unsafe
        // block. as_ref and unwrap dont need it
        // let value = unsafe { (*self.value.get()).as_ref().unwrap() }; // not recommended to wrap
        // everything in unsafe when they dont need it
        value
    }
}
