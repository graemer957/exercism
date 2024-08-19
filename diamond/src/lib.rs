pub fn get_diamond(c: char) -> Vec<String> {
    let number_of_letters = c as u32 - 64;
    dbg!(number_of_letters);

    let mut result =
        Vec::with_capacity(number_of_letters as usize + number_of_letters as usize - 1);

    let rows = (number_of_letters * 2) - 1;
    for row in 1..=rows {
        let mut line = String::new();
        dbg!(row);
        let letter_number = if dbg!(row > number_of_letters) {
            dbg!(rows - row + 1)
        } else {
            dbg!(row)
        };
        dbg!(letter_number);

        let outter = number_of_letters - letter_number;
        dbg!(outter);

        let inner = inner_spacing(letter_number);
        dbg!(inner);

        let c = char::from_u32(letter_number + 64).unwrap();
        (1..=outter).for_each(|_| line.push(' '));
        line.push(c);
        (1..=inner).for_each(|_| line.push(' '));
        if letter_number != 1 {
            line.push(c);
        };
        (1..=outter).for_each(|_| line.push(' '));

        result.push(line);
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
