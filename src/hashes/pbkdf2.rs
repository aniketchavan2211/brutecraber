use pbkdf2::{
    Pbkdf2,
    password_hash::{PasswordHash, PasswordVerifier},
};

pub fn verify(word: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Pbkdf2
            .verify_password(word.as_bytes(), &parsed_hash)
            .is_ok(),

        Err(_) => false,
    }
}
