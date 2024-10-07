pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    let mut primes = vec![2, 3];
    primes.reserve(n);

    let mut whole_number = 5;
    while primes.len() <= n {
        if (2..(whole_number as f32).sqrt() as u32 + 1).all(|x| whole_number % x != 0) {
            primes.push(whole_number);
        }

        whole_number += 2;
    }

    primes[n]
}
