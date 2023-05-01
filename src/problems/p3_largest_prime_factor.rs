//! # Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?
//!
//! ## References
//!
//! - <https://projecteuler.net/problem=3>

pub fn largest_prime_factor(n: usize) -> Option<usize> {
    let max = n / 2;
    for other in 2..max {
        let this = n / other;
        if this * other != n {
            // `this` is not a factor of `n`
            continue;
        }
        if is_prime(this) {
            return Some(this);
        }
    }

    None
}

/// # References
///
/// - <https://en.wikipedia.org/wiki/Primality_test>
pub fn is_prime(n: usize) -> bool {
    // n = 0, 1, 2, 3, 4, 5, 6, 7, ...
    if n == 0 || n == 1 {
        return false;
    }
    // n = 2, 3, 4, 5, 6, 7, 8, ...
    if n < 4 {
        return true;
    }
    // n = 4, 5, 6, 7, 8, 9, 10, 11, ...
    if n % 2 == 0 {
        return false;
    }
    // n = 5, 7, 9, 11, 13, 15, 17, ...
    if n % 3 == 0 {
        return false;
    }
    // n = 5, 7, 11, 13, 17, 19, 23, ...
    // `i` is a factor of `n`
    let mut i = 5;
    while i * i <= n {
        // i     = 5,    11,     17,     23,     ..., sqrt(n)
        // i + 2 =    7,     13,     19,     25, ...,         sqrt(n) + 2
        if n % i == 0 {
            return false;
        }
        if n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_13195() {
        let n = 13195;
        let prime = largest_prime_factor(n);
        assert_eq!(prime.unwrap(), 29);
    }

    #[test]
    fn n_600851475143() {
        let n = 600851475143;
        let prime = largest_prime_factor(n);
        assert_eq!(prime.unwrap(), 6857);
    }
}
