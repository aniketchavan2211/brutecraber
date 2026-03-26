pub fn apply(word: &str) -> Vec<String> {
    let mut variants = vec![word.to_string()];

    let mut capitalize = word.to_string();
    if let Some(first) = capitalize.get_mut(..1) {
        first.make_ascii_uppercase()
    }

    variants.push(capitalize);
}
