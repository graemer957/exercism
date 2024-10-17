pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_stack = vec![];
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => bracket_stack.push(c),
            ']' | '}' | ')' => match (bracket_stack.pop(), c) {
                (Some('['), ']') => continue,
                (Some('{'), '}') => continue,
                (Some('('), ')') => continue,
                _ => return false,
            },
            _ => continue,
        };
    }

    bracket_stack.is_empty()
}
