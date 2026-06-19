# clone-low-level

Implementing `Clone` without `#[derive]` or any existing clone methods. Just raw memory ops.

## Why

`#[derive(Clone)]` is a black box. I wanted to know what's actually inside it and what the compiler generates, what invariants it silently upholds, and where it would blow up if you got any of it wrong.

## What's in here

**`Integer32`**: wraps an `i32`. Trivial case but useful for understanding the shape of the operation:
- cast `&i32` to `*const i32` (safe, forming a raw pointer doesn't need `unsafe`)
- reserve an uninit stack slot with `MaybeUninit`
- `copy_nonoverlapping` into it
- `assume_init` to get back a real value

**`StringTwo`**: wraps a `String`. This is where it gets interesting. A `String` is three words: a heap pointer, a length, a capacity. Bitwise copying the struct gives you two owners of the same heap allocation; double-free on drop. So the actual steps are:

- read `ptr`, `len`, `cap` from the source
- compute a `Layout` (size = cap, align = 1 since it's a byte buffer)
- `alloc::alloc` for a fresh heap block
- guard against null return (`handle_alloc_error`) and zero capacity (`alloc(0)` is UB)
- `copy_nonoverlapping` the valid bytes (`len`, not `cap`)
- reconstruct an independent `String` via `from_raw_parts`
