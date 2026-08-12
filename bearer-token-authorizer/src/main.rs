// 3.6 — Result/Option combinator chains
// Exercise: Bearer Token Authorization Chain
// Spec: see §4 of "3.6 Result, Option combinator chains.md" in the vault.
// Steps 1-4 complete.

use std::collections::HashMap;

enum AuthError {
    MissingHeader,
    MalformedHeader,
    UnknownToken,
    SessionExpired,
    InsufficientPermissions,
}

struct Session {
    token: String,
    expires_at: u64,
    permission_level: u8,
}

fn authorize<'a>(
    headers: &'a HashMap<String, String>,
    sessions: &'a HashMap<String, Session>,
    now: u64,
    min_permission: u8,
) -> Result<&'a Session, AuthError> {
    let auth_token = headers
        .get("Authorization")
        .ok_or_else(|| AuthError::MissingHeader)
        .and_then(|header| {
            header
                .strip_prefix("Bearer ")
                .ok_or_else(|| AuthError::MalformedHeader)
        })?;

    let session = sessions
        .get(auth_token)
        .ok_or_else(|| AuthError::UnknownToken)
        .and_then(|session| {
            if now >= session.expires_at {
                return Err(AuthError::SessionExpired);
            } else {
                return Ok(session);
            }
        })
        .and_then(|session| {
            if session.permission_level < min_permission {
                return Err(AuthError::InsufficientPermissions);
            } else {
                return Ok(session);
            }
        })?;

    Ok(session)
}
fn main() {}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{AuthError, Session, authorize};

    #[test]
    fn all_paths() {
        let mut sessions: HashMap<String, Session> = HashMap::new();

        sessions.insert(
            "tok-valid".to_string(),
            Session {
                token: "tok-valid".to_string(),
                expires_at: 1000,
                permission_level: 5,
            },
        );
        sessions.insert(
            "tok-expired".to_string(),
            Session {
                token: "tok-expired".to_string(),
                expires_at: 100,
                permission_level: 5,
            },
        );
        sessions.insert(
            "tok-low-perm".to_string(),
            Session {
                token: "tok-low-perm".to_string(),
                expires_at: 1000,
                permission_level: 1,
            },
        );
        let mut headers: HashMap<String, String> = HashMap::new();

        assert!(matches!(
            authorize(&headers, &sessions, 500, 3),
            Err(AuthError::MissingHeader)
        ));

        headers.insert("Authorization".to_string(), "tok-valid".to_string());

        assert!(matches!(
            authorize(&headers, &sessions, 500, 3),
            Err(AuthError::MalformedHeader)
        ));
        headers.insert(
            "Authorization".to_string(),
            "Bearer tok-nonexistent".to_string(),
        );
        assert!(matches!(
            authorize(&headers, &sessions, 500, 3),
            Err(AuthError::UnknownToken)
        ));
        headers.insert(
            "Authorization".to_string(),
            "Bearer tok-expired".to_string(),
        );
        assert!(matches!(
            authorize(&headers, &sessions, 500, 3),
            Err(AuthError::SessionExpired)
        ));
        headers.insert(
            "Authorization".to_string(),
            "Bearer tok-low-perm".to_string(),
        );
        assert!(matches!(
            authorize(&headers, &sessions, 500, 3),
            Err(AuthError::InsufficientPermissions)
        ));
        headers.insert("Authorization".to_string(), "Bearer tok-valid".to_string());
        let result = authorize(&headers, &sessions, 500, 3);
        assert!(result.is_ok());

        if let Ok(s) = &result {
            assert_eq!(s.token, "tok-valid");
        } else {
            panic!("expected Ok");
        }

        // exact expiry boundary: now == expires_at should count as expired (now >= expires_at)
        sessions.insert(
            "tok-boundary".to_string(),
            Session {
                token: "tok-boundary".to_string(),
                expires_at: 500,
                permission_level: 5,
            },
        );
        headers.insert(
            "Authorization".to_string(),
            "Bearer tok-boundary".to_string(),
        );
        assert!(matches!(
            authorize(&headers, &sessions, 500, 3),
            Err(AuthError::SessionExpired)
        ));
    }
}
