use argon2::{Argon2, PasswordHash, PasswordVerifier};

pub fn verify(word: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(word.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}
