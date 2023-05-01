//! # 10001st prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?
//!
//! ## References
//!
//! - <https://projecteuler.net/problem=7>

use super::p3_largest_prime_factor::is_prime;

pub fn nth_prime(nth: usize) -> usize {
    let mut i = 0;
    let mut n = 0;
    while i != nth {
        loop {
            n += 1;
            if is_prime(n) {
                break;
            }
        }
        i += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_6th() {
        let nth = 6;
        let prime = nth_prime(nth);
        assert_eq!(prime, 13);
    }

    #[test]
    fn prime_10001st() {
        let nth = 10001;
        let prime = nth_prime(nth);
        assert_eq!(prime, 104743);
    }
}
