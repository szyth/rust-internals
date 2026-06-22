fn get_len<T>(v: &Vec<T>) -> usize {
    v.len()
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(get_len(&v)); // Read and Write in same line but still no error as NLL invalidates the
    // borrow of get_len(&v) the moment it returns the size aka 3 so WRITE v.push(3) works without error

    // fails - explicit &mut taken first, no two-phase relaxation:
    Vec::push(&mut v, v.len()); // E0502

    println!("{v:?}")
}
