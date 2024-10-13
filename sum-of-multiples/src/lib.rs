pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&x| {
            factors
                .iter()
                .filter(|&&factor| factor > 0)
                .any(|&factor| x % factor == 0)
        })
        .sum()
}
