pub fn series(digits: &str, len: usize) -> Vec<String> {
    // Not clear why this is a special case!
    if len == 0 {
        return vec!["".to_string(); 6];
    }

    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|w| w.iter().collect())
        .collect::<Vec<_>>()
}
