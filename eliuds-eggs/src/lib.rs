pub fn egg_count(mut display_value: u32) -> usize {
    // The use of standard libraries `count_ones` seems against the spirit of this challenge the
    // function body would simply be `display_value.count_ones() as usize` ;-)
    //
    // See: https://doc.rust-lang.org/std/primitive.u32.html#method.count_ones

    let mut egg_total = 0;

    while display_value > 0 {
        egg_total += display_value & 1;

        display_value >>= 1;
    }

    egg_total as usize
}
