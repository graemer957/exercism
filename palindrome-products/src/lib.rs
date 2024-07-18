#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::collections::HashSet;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value <= 9 {
            return Some(Self(value));
        }

        let mut mutable_value = value;
        let mut new_value = 0;
        while mutable_value > 0 {
            let first_digit = mutable_value % 10;
            mutable_value /= 10;
            new_value *= 10;
            new_value += first_digit;
        }
        if new_value == value {
            return Some(Self(value));
        }

        None
    }

    /// Get the value of this palindrome.
    #[must_use]
    pub const fn into_inner(self) -> u64 {
        self.0
    }
}

#[must_use]
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromic_numbers = HashSet::new();

    for number in min..=max {
        let products = (number..=max).map(|n| number * n);
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
