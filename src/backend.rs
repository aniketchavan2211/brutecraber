pub trait CrackingBackend {
    fn run(&self, hashes: &[&str], wordlist: &str, hash_type: &str, rule: bool) -> usize;
}
