// 2.4 — Const generics
// Exercise: Fixed-Capacity Ring Buffer:
// Overwrite old data with new item rather than discarding item
// Spec: see §4 of "2.4 Const generics.md" in the vault.

struct RingBuffer<T, const N: usize> {
    items: [Option<T>; N],
    head: usize, // points to oldest item in `items`
    len: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    fn new() -> Self {
        // compile time check to reject zero-capacity buffer
        // LSP may not hightlight the error, do `cargo test` or `cargo build`
        const { assert!(N > 0) }

        Self {
            // buf: [None; N], // `Copy` not satisfied for `Option<T>`
            items: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
        }
    }

    fn push(&mut self, item: T) {
        if self.len == N {
            self.items[self.head] = Some(item);
            self.head = (self.head + 1) % N;
        } else {
            self.items[(self.head + self.len) % N] = Some(item);
            self.len += 1;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let popped = self.items[self.head].take();

        self.head = (self.head + 1) % N;
        self.len -= 1;
        popped
    }

    fn capacity(&self) -> usize {
        N
    }
    fn len(&self) -> usize {
        self.len
    }
    fn is_full(&self) -> bool {
        self.len == N
    }
}

// fn to prove two different N's are geniunely different types
fn takes_four(_buffer: RingBuffer<usize, 4>) {}

#[cfg(test)]
mod test {
    use crate::RingBuffer;

    #[test]
    fn test_push_items_upto_length() {
        let mut buffer: RingBuffer<usize, 3> = RingBuffer::new();

        buffer.push(1);
        assert_eq!([Some(1), None, None], buffer.items);
        buffer.push(2);
        buffer.push(3);
        assert_eq!([Some(1), Some(2), Some(3)], buffer.items);
    }

    #[test]
    fn test_push_overwrites_oldest_when_full() {
        let mut buffer: RingBuffer<usize, 3> = RingBuffer::new();

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);
        buffer.push(5);

        assert_eq!([Some(4), Some(5), Some(3)], buffer.items);
    }
    #[test]
    fn test_pop_one_element() {
        let mut buffer: RingBuffer<usize, 3> = RingBuffer::new();

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        assert_eq!([Some(4), Some(2), Some(3)], buffer.items);

        buffer.pop();
        assert_eq!([Some(4), None, Some(3)], buffer.items);
    }
    #[test]
    fn test_pop_drains_buffer_to_empty() {
        let mut buffer: RingBuffer<usize, 3> = RingBuffer::new();

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        assert_eq!([Some(4), Some(2), Some(3)], buffer.items);

        buffer.pop();
        buffer.pop();
        assert_eq!([Some(4), None, None], buffer.items);
        buffer.pop();
        assert_eq!([None, None, None], buffer.items);
    }
    #[test]
    fn test_push_after_pop_fills_freed_slot() {
        let mut buffer: RingBuffer<usize, 3> = RingBuffer::new();

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        assert_eq!([Some(4), Some(2), Some(3)], buffer.items);

        buffer.pop();
        assert_eq!([Some(4), None, Some(3)], buffer.items);
        buffer.push(5);
        assert_eq!([Some(4), Some(5), Some(3)], buffer.items);
    }

    #[test]
    fn test_two_n_are_different_types() {
        let _buffer_two: RingBuffer<usize, 8> = RingBuffer::new();

        // error[E0308], mismatched types, expected '4', found '8'
        // expected struct 'RingBuffer<_, 4>', found struct 'RingBuffer<_, 8>'
        // Genuinely two different types for two values of N
        // takes_four(_buffer_two);
    }

    #[test]
    fn test_zero_capacity_ringbuffer_fails_compile_time() {
        // error[E0080]: evaluation of `RingBuffer::<usize, 0>::new::{constant#0}` failed
        // let buffer: RingBuffer<usize, 0> = RingBuffer::new();
    }
}

fn main() {}
