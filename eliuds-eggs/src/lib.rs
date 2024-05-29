pub fn egg_count(display_value: u32) -> usize {
    if display_value == 0 {
        return 0;
    };

    let number_of_bits = display_value.ilog2() + 1;
    let mut egg_total = 0;
    for i in 0..number_of_bits {
        let thing = 1 << i;
        if (thing & display_value) == thing {
            egg_total += 1;
        }
    }

    egg_total
}
