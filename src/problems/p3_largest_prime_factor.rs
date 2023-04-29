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

fn is_prime(n: usize) -> bool {
    let max = n / 2;
    for i in 2..max {
        let other = n / i;
        if other * i == n {
            return false;
        }
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
