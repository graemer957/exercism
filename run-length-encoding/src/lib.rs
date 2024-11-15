pub fn encode(source: &str) -> String {
    let mut result = String::new();

    let mut chars = source.chars().peekable();
    let mut char_count = 1;
    while let Some(char) = chars.next() {
        if Some(&char) == chars.peek() {
            char_count += 1;
            continue;
        }

        if char_count > 1 {
            result.push_str(&char_count.to_string())
        }
        result.push(char);

        char_count = 1;
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();

    let mut digits = String::new();
    for char in source.chars() {
        if char.is_ascii_digit() {
            digits.push(char);
            continue;
        }

        let repeat_count = digits.parse::<usize>().unwrap_or(1);
        digits.clear();

        result.push_str(&char.to_string().repeat(repeat_count));
    }

    result
}
