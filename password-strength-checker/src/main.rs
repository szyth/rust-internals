// 3.8 — anyhow vs thiserror, custom error type design
// Exercise: Password Strength Checker (Library + Binary)
// Spec: see §4 of "3.8 anyhow vs thiserror, custom error type design.md" in the vault.
fn main() {
    match password_strength_checker::register_user("weak") {
        Ok(()) => println!("registration succeeded"),
        Err(err) => {
            println!("registration failed: {err}"); // Display - just the context
            // message

            eprintln!("{err:?}"); // Debug - full cause chain, for logs
        }
    }
    match password_strength_checker::register_user("Str0ng!Pass") {
        Ok(()) => println!("registration succeeded"),
        Err(err) => println!("registration failed: {err}"),
    }
}
