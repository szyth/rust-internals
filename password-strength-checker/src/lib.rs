use anyhow::Context;

mod password;

pub fn register_user(password: &str) -> anyhow::Result<()> {
    let _result = password::check_strength(password)
        .context("password does not meet security requirements")?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_weak_password_too_short() {
        let err = register_user("weak").unwrap_err();
        // convert type-erased anyhow error into concrete error using downcast()
        let concrete_err: Option<&password::PasswordError> = err.downcast_ref();

        assert!(matches!(
            concrete_err,
            Some(password::PasswordError::TooShort { .. })
        ));

        // anyhow's context() prints Display {}
        assert_eq!(
            "password does not meet security requirements",
            format!("{err}")
        );

        // The Debug {:?} prints the full "Caused by" chain down to the specific
        // PasswordError
        assert!(format!("{err:?}").contains("Caused by"));
        assert!(format!("{err:?}").contains("Too short password. Actual: 4. Minimum: 8"));

        // DISPLAY ERROR:
        // password does not meet security requirements
        //
        // DEBUG ERROR:
        // password does not meet security requirements
        // Caused by:
        //    Too short password. Actual: 4. Minimum: 8
    }

    #[test]
    fn test_strong_password() {
        let result = register_user("StrongPass!123");

        assert!(result.is_ok());
    }
}
