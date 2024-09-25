/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.split_whitespace().collect::<String>();
    if code.len() <= 1 || !code.bytes().all(|c| c.is_ascii_digit()) {
        return false;
    }

    code.chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(index, mut digit)| {
            if index % 2 != 0 {
                digit *= 2;
                if digit > 9 {
                    digit -= 9
                }
            }
            digit
        })
        .sum::<u32>()
        % 10
        == 0
}
