#[derive(thiserror::Error, Debug)]
pub enum PasswordError {
    #[error("Too short password. Actual: {actual}. Minimum: {min}")]
    TooShort { min: usize, actual: usize },
    #[error("password must contain an uppercase letter")]
    MissingUppercase,
    #[error("password must contain a digit")]
    MissingDigit,
    #[error("password must contain a special character")]
    MissingSpecialChar,
}

pub fn check_strength(password: &str) -> Result<(), PasswordError> {
    if password.len() < 8 {
        return Err(PasswordError::TooShort {
            min: 8,
            actual: password.len(),
        });
    }
    if !password.chars().any(|ch| ch.is_uppercase()) {
        return Err(PasswordError::MissingUppercase);
    }
    if !password.chars().any(|ch| ch.is_ascii_digit()) {
        return Err(PasswordError::MissingDigit);
    }
    if password.chars().all(|ch| ch.is_alphanumeric()) {
        return Err(PasswordError::MissingSpecialChar);
    }

    Ok(())
}
