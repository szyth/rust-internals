# password-strength-checker

A genuine two-crate-target split (`src/lib.rs` + `src/main.rs` in one package) — a `thiserror`-derived `PasswordError` enum in the library half, consumed by `anyhow`-based code in the binary half, connected by `?` with zero manual glue.

## What's in here

`password::check_strength()` returns `Result<(), PasswordError>` — a typed, matchable enum (`TooShort`, `MissingUppercase`, `MissingDigit`, `MissingSpecialChar`), each with its own `#[error("...")]` message. `register_user()` calls it via `?` composed with `.context("password does not meet security requirements")`, converting straight into `anyhow::Result<()>` with no `From` impl written anywhere — the blanket `impl<E: Error + Send + Sync + 'static> From<E> for anyhow::Error` handles it, since `#[derive(thiserror::Error)]` makes `PasswordError` satisfy that bound automatically.

Since `main.rs` and `lib.rs` are separate crate roots even within one package, the binary reaches into the library explicitly (`password_strength_checker::register_user(...)`) rather than sharing scope directly — a small but real consequence of choosing the genuine library/binary split over a single-file `mod` block.

