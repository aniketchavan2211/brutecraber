use bcrypt::verify;

// verify returns bool!
pub fn crack(word: &str, hash: &str) -> bool {
    verify(word, hash).unwrap_or(false)
}
