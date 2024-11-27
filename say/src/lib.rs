use std::borrow::Cow;

const HUNDRED: u64 = 100;
const UNITS: [(u32, &str); 6] = [
    (18, "quintillion"),
    (15, "quadrillion"),
    (12, "trillion"),
    (9, "billion"),
    (6, "million"),
    (3, "thousand"),
];

fn ones(n: u64) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => unreachable!(),
    }
}

fn ten_to_nineteen(n: u64) -> &'static str {
    match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => unreachable!(),
    }
}

fn tens(n: u64) -> &'static str {
    match n {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => unreachable!(),
    }
}

fn less_than_one_hundred(n: u64) -> String {
    match n {
        1..=9 => ones(n).to_string(),
        11..=19 => ten_to_nineteen(n).to_string(),
        20..=99 => {
            let tens = tens(n / 10);
            let remaining = n % 10;
            if remaining == 0 {
                tens.to_string()
            } else {
                format!("{}-{}", tens, ones(remaining))
            }
        }
        _ => unreachable!(),
    }
}

fn hundreds(n: u64) -> Vec<Cow<'static, str>> {
    let mut result = Vec::new();

    let hundreds = n / HUNDRED;
    if hundreds != 0 {
        result.push(Cow::from(format!("{} hundred", ones(hundreds))));
    }

    let remaining = n % HUNDRED;
    if remaining != 0 {
        result.push(Cow::from(less_than_one_hundred(remaining)));
    }

    result
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut remaining = n;
    let mut result: Vec<Cow<str>> = Vec::new();

    for (zeros, unit_description) in UNITS {
        let size = 10_u64.pow(zeros);
        let quantity = remaining / size;
        if quantity > 0 {
            result.append(&mut hundreds(quantity));
            result.push(Cow::from(unit_description));
        }
        remaining %= size;
    }
    if remaining > 0 {
        result.append(&mut hundreds(remaining));
    }

    result.join(" ")
}
