#[derive(Debug, PartialEq)]
struct Integer32(i32);

impl Clone for Integer32 {
    fn clone(&self) -> Self {
        // STEPS: ALLOCATE EMPTY DEST SLOT -> COPY BYTES (memcpy)

        // raw pointer to the source field
        let src = &self.0 as *const i32;

        // prepare the destination
        use std::mem::MaybeUninit;
        let mut a: MaybeUninit<i32> = MaybeUninit::uninit();
        let dst = a.as_mut_ptr();

        // use rust's memcpy
        unsafe {
            // why not ptr::copy()?
            // src points to struct on stack;
            // dst points to fresh MaybeUninit slot;
            // hence both are guaranteed distinct locatons. so UB not possible

            std::ptr::copy_nonoverlapping(src, dst, 1);
        }

        let cloned = unsafe { a.assume_init() };

        Self(cloned)

        // return Self(self.0);
        // Since i32 is Copy, the following works as well without the above implementation
        // the complier will implicitly do the MaybeUninit and copy_nonoverlapping in background
    }
}

#[test]
fn clone_i32() {
    let integer_one = Integer32(10);
    let mut integer_two = integer_one.clone();

    assert_eq!(integer_one, integer_two, "Both integers should be same");

    // modify clone
    integer_two.0 = 20;

    assert_ne!(
        integer_one, integer_two,
        "Modifying Integer 2 should not alter Integer 1"
    );
}

#[derive(Debug, PartialEq)]
struct StringTwo(String);

impl Clone for StringTwo {
    fn clone(&self) -> Self {
        // STEPS: ALLOCATE NEW HEAP -> COPY BYTES (memcpy)-> CONSTRUCT NEW STRUCT

        // get src pointer
        let ptr = self.0.as_ptr();
        let cap = self.0.capacity();
        let len = self.0.len();

        // alloc() on cap = 0 is UB
        if cap == 0 {
            return StringTwo(String::new());
        }

        use std::alloc;
        // heap specs aka Layout
        let layout = alloc::Layout::from_size_align(cap, 1)
            .expect("cap comes from a valid String; align 1 is always valid");
        // Q: why align = 1?
        // A: String's heap buffer is a [u8] ie a Byte array. u8 has an alignment requirement of 1
        // byte

        // Allocate Heap
        let dst_heap_init = unsafe { alloc::alloc(layout) };

        // alloc() may return null address if allocation fails, causing UB
        if dst_heap_init.is_null() {
            alloc::handle_alloc_error(layout);
        }

        // Copy bytes into Dest Heap; memcpy
        unsafe { std::ptr::copy_nonoverlapping(ptr, dst_heap_init, len) }

        let cloned_string = unsafe { String::from_raw_parts(dst_heap_init, len, cap) };
        StringTwo(cloned_string)
    }
}

#[test]
fn clone_string() {
    let string_one = StringTwo("Im a string".to_string());
    let mut string_two = string_one.clone();

    assert_eq!(string_one, string_two, "Both strings should be same");

    // modify clone
    string_two.0 = "Im a string two".to_string();

    assert_ne!(
        string_one, string_two,
        "Modifying string 2 should not alter string 1"
    );
}
