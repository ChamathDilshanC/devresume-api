pub fn estimate_token_count(text: &str) -> usize {
    text.split_whitespace().count() * 4 / 3
}
