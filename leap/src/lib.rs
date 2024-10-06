pub fn is_leap_year(year: u64) -> bool {
    let is_century = year % 100 == 0;
    let is_divisible_by_400 = year % 400 == 0;
    let is_divisible_by_4 = year % 4 == 0;

    if is_divisible_by_400 || (!is_century && is_divisible_by_4) {
        true
    } else {
        false
    }
}
