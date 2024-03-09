use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    let word_letter_count = letter_count(word);
    for possible_anagram in possible_anagrams {
        if possible_anagram.to_lowercase() == word.to_lowercase() {
            continue;
        }

        if word_letter_count == letter_count(possible_anagram) {
            set.insert(*possible_anagram);
        }
    }

    set
}

fn letter_count(word: &str) -> HashMap<char, usize> {
    let mut letters = HashMap::new();
    word.to_lowercase().chars().for_each(|x| {
        *letters.entry(x).or_default() += 1;
    });

    letters
}
