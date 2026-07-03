// 1.1 — Move Semantics, Copy vs Clone
// Exercise: One-Time Secret Vault
// Spec: see §4 of "1.1 Move semantics, Copy vs Clone.md" in the notes vault.

use std::collections::HashMap;

struct Secret {
    payload: String,
    ttl_secs: u32,
}

impl Secret {
    fn new(payload: impl Into<String>, ttl_secs: u32) -> Self {
        Self {
            payload: payload.into(),
            ttl_secs,
        }
    }
}

struct Vault {
    secrets: HashMap<u32, Option<Secret>>,
}

impl Vault {
    fn new() -> Self {
        Self {
            secrets: HashMap::new(),
        }
    }
    fn add_entry(&mut self, key_id: u32, secret: Secret) {
        self.secrets.insert(key_id, Some(secret));
    }
    fn checkout(&mut self, id: u32) -> Option<Secret> {
        self.secrets.get_mut(&id)?.take()
    }
    fn revoke(&mut self, id: u32) {
        if let Some(secret) = self.secrets.get_mut(&id) {
            *secret = None;
        }
    }
}

fn deliver(secret: Secret) -> String {
    // consume secret
    "secret delivered successfully".to_string() // receipt
}

fn main() {
    let secret = Secret::new("APIX-323", 30);
    let mut vault = Vault::new();
    vault.add_entry(1, secret);

    let secret_from_vault = vault.checkout(1);
    if let Some(secret) = secret_from_vault {
        println!(
            "LOGGING secret with TTL secs remaining: {}",
            secret.ttl_secs
        );
        let _ = deliver(secret);
        // let _ = deliver(secret); // Error: E0382
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_checkout_once() {
        let secret = Secret::new("APIX-323", 30);
        let mut vault = Vault::new();
        vault.add_entry(1, secret);

        let secret_from_vault = vault.checkout(1);
        assert!(secret_from_vault.is_some(), "First checkout should be some");
    }

    #[test]
    fn test_checkout_double() {
        let secret = Secret::new("APIX-323", 30);
        let mut vault = Vault::new();
        vault.add_entry(1, secret);

        let _checkout_first = vault.checkout(1);
        let second_checkout = vault.checkout(1);
        assert!(
            second_checkout.is_none(),
            "Second checkout should return None"
        );
    }

    #[test]
    fn test_checkout_invalid_id() {
        let mut vault = Vault::new();
        let checkout = vault.checkout(1);
        assert!(
            checkout.is_none(),
            "Invalid ID should return None on checkout"
        );
    }
    #[test]
    fn test_revoke_then_checkout() {
        let secret = Secret::new("APIX-323", 30);
        let mut vault = Vault::new();
        vault.add_entry(1, secret);
        vault.revoke(1);

        let secret_from_vault = vault.checkout(1);
        assert!(
            secret_from_vault.is_none(),
            "Revoking a secret should return None on checkout()"
        );
    }
}
