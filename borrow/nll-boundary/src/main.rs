fn main() {
    let mut var = 10;

    let refer = &var;
    let mut_refer = &mut var; // Fix: move this AFTER the last usage of "refer" to let
    // NLL invalidate the "refer" borrow. Hence maintaining aliasing-xor-mutability

    println!("{refer}"); // shared borrow USED HERE
    *mut_refer += 1; // mutable borrow USED HERE
}
