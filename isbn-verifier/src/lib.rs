/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.split('-').collect::<String>();
    if isbn.len() != 10 {
        return false;
    }

    let mut sum = 0;
    for (i, char) in (1..=10).rev().zip(isbn.chars()) {
        sum += i * match char.to_digit(10) {
            Some(digit) => digit,
            None if i == 1 && char == 'X' => 10,
            None => return false,
        };
    }

    sum % 11 == 0
}
