// Minor tweak after reviewing community solutions
//
// Great solution https://exercism.org/tracks/rust/exercises/collatz-conjecture/solutions/jcsr
// more idiomatic than mine :-)

pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;

    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
        }

        steps += 1;
    }

    Some(steps)
}
