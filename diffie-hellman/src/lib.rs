use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}

/// See https://en.wikipedia.org/wiki/Modular_exponentiation
///
/// Right-to-left binary method
fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut base = base as u128;
    let mut exponent = exponent as u128;
    let modulus = modulus as u128;
    let mut result = 1;

    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus
        }

        exponent >>= 1;
        base = (base * base) % modulus
    }

    result as u64
}
