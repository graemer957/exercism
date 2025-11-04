/// # Panics
///
/// If `c` is not a between "A" to "Z"
#[must_use]
pub fn get_diamond(c: char) -> Vec<String> {
    let number_of_letters = c as u32 - 64;

    let rows = (number_of_letters * 2) - 1;
    let mut result = Vec::with_capacity(rows as usize);

    for row in 1..=rows {
        let mut line = String::new();
        let letter_number = if row > number_of_letters {
            rows - row + 1
        } else {
            row
        };

        let outter_spaces = number_of_letters - letter_number;
        let inner_spaces = inner_spacing(letter_number);

        let character =
            char::from_u32(letter_number + 64).expect("Letter should be between A to Z");
        (1..=outter_spaces).for_each(|_| line.push(' '));
        line.push(character);
        (1..=inner_spaces).for_each(|_| line.push(' '));
        if letter_number != 1 {
            line.push(character);
        }
        (1..=outter_spaces).for_each(|_| line.push(' '));

        result.push(line);
    }

    result
}

const fn inner_spacing(letter_number: u32) -> u32 {
    match letter_number {
        1 => 0,
        2 => 1,
        _ => letter_number + (letter_number - 1) - 2,
    }
}
