pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum: u32 = 0;
    let digits = num.to_string();
    let number_of_digits = digits.len() as u32;
    for char in digits.chars() {
        let digit = char.to_digit(10).unwrap(); // Safety: We already know this is a number!
        let (result, overflow) = sum.overflowing_add(digit.pow(number_of_digits));
        if !overflow {
            sum = result;
        }
    }

    num == sum
}
