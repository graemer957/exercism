pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];
    let mut remainder = n;
    let mut candidates = 2..;

    while remainder > 1 {
        let Some(candidate) = candidates.next() else {
            break;
        };

        while remainder % candidate == 0 {
            prime_factors.push(candidate);
            remainder /= candidate;
        }
    }

    prime_factors
}
