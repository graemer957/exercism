use std::iter;

pub fn abbreviate(phrase: &str) -> String {
    iter::once(' ')
        .chain(phrase.chars())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|window| {
            (" -_".contains(window[0]) || (window[0].is_lowercase() && window[1].is_uppercase()))
                && window[1].is_alphabetic()
        })
        .map(|window| window[1].to_ascii_uppercase())
        .collect()
}
