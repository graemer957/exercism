use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| ![' ', '-'].contains(c))
        .fold(HashMap::new(), |mut acc: HashMap<char, i8>, x| {
            *acc.entry(x).or_default() += 1;
            acc
        })
        .values()
        .all(|&count| count == 1)
}
