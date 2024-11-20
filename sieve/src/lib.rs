pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }

    let mut sieve = (2..=upper_bound).map(|_| true).collect::<Vec<_>>();

    (2..=upper_bound as usize).for_each(|number| {
        if sieve[number - 2] {
            (number..=upper_bound as usize)
                .step_by(number)
                .skip(1)
                .for_each(|i| sieve[i - 2] = false);
        }
    });

    sieve
        .iter()
        .enumerate()
        .filter_map(|(index, is_prime)| {
            if *is_prime {
                Some(index as u64 + 2)
            } else {
                None
            }
        })
        .collect()
}
