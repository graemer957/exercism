use primal::Primes;

pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];
    let mut remainder = n;
    let mut all_primes = Primes::all();

    while remainder > 1 {
        if let Some(prime) = all_primes.next() {
            let prime = prime as u64;

            while remainder > 1 && remainder % prime == 0 {
                prime_factors.push(prime);
                remainder /= prime;
            }
        }
    }

    prime_factors
}
