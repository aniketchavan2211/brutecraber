use crate::backend::CrackingBackend;
use crate::cracker;

pub struct CpuBackend;

impl CrackingBackend for CpuBackend {
    fn run(&self, hashes: &[&str], wordlist: &str, hash_type: &str, rule: bool) -> usize {
        cracker::run(hashes, wordlist, hash_type, rule)
    }
}
