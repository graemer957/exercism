use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Roman {
    thousands: u32,
    hundreds: u32,
    tens: u32,
    ones: u32,
}

fn number_to_numerals(
    number: u32,
    lower: &'static str,
    mid: &'static str,
    upper: &'static str,
) -> Vec<&'static str> {
    let mut numerals = Vec::new();

    match number {
        x if (1..=3).contains(&x) => (1..=x).for_each(|_| numerals.push(lower)),
        4 => {
            numerals.push(lower);
            numerals.push(mid);
        }
        5 => numerals.push(mid),
        x if (6..=8).contains(&x) => {
            numerals.push(mid);
            (6..=x).for_each(|_| numerals.push(lower));
        }
        9 => {
            numerals.push(lower);
            numerals.push(upper);
        }
        _ => {}
    }

    numerals
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut numerals = Vec::new();

        numerals.push(match self.thousands {
            1 => "M",
            2 => "MM",
            3 => "MMM",
            _ => "",
        });
        numerals.append(&mut number_to_numerals(self.hundreds, "C", "D", "M"));
        numerals.append(&mut number_to_numerals(self.tens, "X", "L", "C"));
        numerals.append(&mut number_to_numerals(self.ones, "I", "V", "X"));

        write!(f, "{}", numerals.join(""))
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut remaining = num;
        let thousands = remaining / 1000;
        remaining %= 1000;
        let hundreds = remaining / 100;
        remaining %= 100;
        let tens = remaining / 10;
        remaining %= 10;
        let ones = remaining;

        Self {
            thousands,
            hundreds,
            tens,
            ones,
        }
    }
}
