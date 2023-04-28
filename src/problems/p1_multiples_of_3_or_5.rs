//! # Multiples of 3 or 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! # References
//!
//! - <https://projecteuler.net/problem=1>
//!

pub fn sum_3_or_5_multiples(below: usize) -> usize {
    let mut sum = 0;
    for i in 0..below {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

pub fn sum_3_or_5_multiples_iter(below: usize) -> usize {
    let iter = 0..below;
    iter.into_iter().filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn below_10() {
        let below = 10;
        let sum = sum_3_or_5_multiples(below);
        assert_eq!(sum, 23);
        let sum = sum_3_or_5_multiples_iter(below);
        assert_eq!(sum, 23);
    }

    #[test]
    fn below_1000() {
        let below = 1000;
        let sum = sum_3_or_5_multiples(below);
        assert_eq!(sum, 233168);
        let sum = sum_3_or_5_multiples_iter(below);
        assert_eq!(sum, 233168);
    }
}
