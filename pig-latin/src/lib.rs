const VOWEL_SOUNDS: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yt"];
const CONSONANT_SOUNDS: [&str; 25] = [
    "thr", "sch", "ch", "qu", "th", "rh", "b", "c", "d", "f", "g", "j", "k", "l", "m", "n", "p",
    "q", "r", "s", "t", "v", "x", "z", "y",
];

pub fn translate(input: &str) -> String {
    let mut words = vec![];

    'outer: for word in input.split_whitespace() {
        let mut translation = String::with_capacity(word.len() + 2);

        // Rule #4
        if word.len() == 2 && word.ends_with('y') {
            translation.push('y');
            translation.push_str(&word[0..=0]);
            translation.push_str("ay");

            words.push(translation);
            continue;
        }

        // Rule #1
        for sound in VOWEL_SOUNDS {
            if word.starts_with(sound) {
                translation.push_str(word);
                translation.push_str("ay");

                words.push(translation);
                continue 'outer;
            }
        }

        // Rule #2
        for sound in CONSONANT_SOUNDS {
            if let Some(remaining) = word.strip_prefix(sound) {
                // Rule #3
                if let Some(special_case) = remaining.strip_prefix("qu") {
                    translation.push_str(special_case);
                    translation.push_str(sound);
                    translation.push_str("qu");
                } else {
                    translation.push_str(remaining);
                    translation.push_str(sound);
                }
                translation.push_str("ay");

                words.push(translation);
                continue 'outer;
            }
        }
    }

    words.join(" ")
}
