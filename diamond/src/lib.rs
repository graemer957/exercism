pub fn get_diamond(c: char) -> Vec<String> {
    let number_of_letters = c as u32 - 64;
    dbg!(number_of_letters);

    let mut result = Vec::with_capacity(number_of_letters + number_of_letters - 1);

    for letter_number in 1..=number_of_letters {
        let outter = number_of_letters - letter_number;
        dbg!(outter);

        let inner = inner_spacing(letter_number);
        dbg!(inner);
    }

    result
}

fn inner_spacing(letter_number: u32) -> u32 {
    match letter_number {
        1 => 0,
        2 => 1,
        _ => letter_number + (letter_number - 1) - 2,
    }
}
