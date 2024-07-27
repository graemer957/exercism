use std::collections::HashSet;

// This is not really a new iteration, but a note to anyone looking, or my future self
// that there are two really good, short, solutions:
// - https://exercism.org/tracks/rust/exercises/palindrome-products/solutions/mattnewport
//   Clever and short solution, although I do not find it easy to follow
// - https://exercism.org/tracks/rust/exercises/palindrome-products/solutions/petertseng
//   Similar to mine, but slightly different approach and much faster, due to cleverly cutting down
//   the range of numbers to check
//
// I took a **long** time to produce this solution, mainly as I investigated a number of different
// approaches and spent some time on performance optimisations (using `cargo flamegraph`). In
// release mode it _looks like_ most of the time is spent calculating the palindrone, which I am
// not able to speed up more than this

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut remaining = value;
        let mut reversed_value = 0;
        while remaining > 0 {
            let first_digit = remaining % 10;
            remaining /= 10;
            reversed_value *= 10;
            reversed_value += first_digit;
        }

        if reversed_value == value {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromic_numbers = HashSet::new();

    for number in min..=max {
        let products = (number..=max).map(|n| number * n).filter(|n| n % 10 != 0);
        for product in products {
            if let Some(palindrome) = Palindrome::new(product) {
                palindromic_numbers.insert(palindrome);
            }
        }
    }

    let mut palindromic_numbers: Vec<_> = palindromic_numbers.iter().collect();
    palindromic_numbers.sort();

    match (palindromic_numbers.first(), palindromic_numbers.last()) {
        (Some(first), Some(last)) => Some((**first, **last)),
        _ => None,
    }
}
