// 4.1 — std::thread, scoped threads
// Exercise: Parallel Password Strength Auditor
// Spec: see §4 of "4.1 std thread, scoped threads.md" in the vault.

fn is_weak(password: &str) -> bool {
    // adding a sentinel poison value to demonstrate a panic in thread
    if password == "\0poison" {
        panic!("sentinel value encountered")
    }
    password.len() < 8
}

fn audit_batch(passwords: &[String], results: &mut [bool]) {
    let num_threads = std::thread::available_parallelism().unwrap().get();
    let chunk_size = passwords.len().div_ceil(num_threads);
    let password_chunks = passwords.chunks(chunk_size);
    let result_chunks = results.chunks_mut(chunk_size);
    std::thread::scope(|s| {
        for (pw_chunk, result_chunk) in password_chunks.zip(result_chunks) {
            s.spawn(move || {
                for (password, result) in pw_chunk.iter().zip(result_chunk.iter_mut()) {
                    *result = is_weak(password);
                }
            });
        }
    })
}
fn audit_batch_safe(passwords: &[String], results: &mut [bool]) {
    let num_threads = std::thread::available_parallelism().unwrap().get();
    let chunk_size = passwords.len().div_ceil(num_threads);
    let password_chunks = passwords.chunks(chunk_size);
    let result_chunks = results.chunks_mut(chunk_size);

    std::thread::scope(|s| {
        let mut handles = vec![];
        for (pw_chunk, result_chunk) in password_chunks.zip(result_chunks) {
            let handle = s.spawn(move || {
                for (password, result) in pw_chunk.iter().zip(result_chunk.iter_mut()) {
                    *result = is_weak(password);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.join();
            if let Err(err) = result {
                eprintln!("{err:?}")
            }
        }
    });
}

// This Fails with error[0521]: non-'static borrow inside spawn
// fn audit_batch_spawn(passwords: &[String], results: &mut [bool]) {
//     let num_threads = std::thread::available_parallelism().unwrap().get();
//     let chunk_size = passwords.len().div_ceil(num_threads);
//     let password_chunks = passwords.chunks(chunk_size);
//     let result_chunks = results.chunks_mut(chunk_size);
//     for (pw_chunk, result_chunk) in password_chunks.zip(result_chunks) {
//         std::thread::spawn(move || {
//             for (password, result) in pw_chunk.iter().zip(result_chunk.iter_mut()) {
//                 *result = is_weak(password);
//             }
//         });
//     }
// }

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_passwords() -> Vec<String> {
        vec![
            "short".to_string(),
            "alsoshort".to_string(),
            "longenoughpassword".to_string(),
            "hi".to_string(),
            "another_long_one".to_string(),
            "x".to_string(),
            "\0poison".to_string(),
        ]
    }

    #[test]
    #[should_panic]
    fn test_thread_scope_implicit_join() {
        let passwords = sample_passwords();
        let mut results = vec![false; passwords.len()];

        audit_batch(&passwords, &mut results);
    }

    #[test]
    fn test_thread_scope_explicit_join() {
        let passwords = sample_passwords();
        let mut results = vec![false; passwords.len()];

        audit_batch_safe(&passwords, &mut results);
        assert_eq!(results, vec![true, false, false, true, false, true, false])
    }
}
