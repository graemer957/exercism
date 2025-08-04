use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Roman {
    thousands: u32,
    hundreds: u32,
    tens: u32,
    ones: u32,
}

fn number_to_numerals(
    f: &mut Formatter<'_>,
    number: u32,
    lower: &'static str,
    mid: &'static str,
    upper: &'static str,
) -> Result {
    match number {
        1..=3 => {
            for _ in 1..=number {
                write!(f, "{lower}")?;
            }
        }
        4 => write!(f, "{lower}{mid}")?,
        5 => write!(f, "{mid}")?,
        6..=8 => {
            write!(f, "{mid}")?;
            for _ in 6..=number {
                write!(f, "{lower}")?;
            }
        }
        9 => write!(f, "{lower}{upper}")?,
        _ => {}
    }

    Ok(())
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.thousands {
            1 => write!(f, "M")?,
            2 => write!(f, "MM")?,
            3 => write!(f, "MMM")?,
            _ => {}
        }
        number_to_numerals(f, self.hundreds, "C", "D", "M")?;
        number_to_numerals(f, self.tens, "X", "L", "C")?;
        number_to_numerals(f, self.ones, "I", "V", "X")
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
