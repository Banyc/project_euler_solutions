//! # Smallest multiple
//!
//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//!
//! ## References
//!
//! - <https://projecteuler.net/problem=5>

pub fn smallest_multiple(max_factor: usize) -> Option<usize> {
    for n in max_factor..usize::MAX {
        let mut all_divisible = true;
        for factor in 1..=max_factor {
            if n % factor != 0 {
                all_divisible = false;
                break;
            }
        }
        if all_divisible {
            return Some(n);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_factor_10() {
        let max_factor = 10;
        let multiple = smallest_multiple(max_factor).unwrap();
        assert_eq!(multiple, 2520);
    }

    #[test]
    fn max_factor_20() {
        let max_factor = 20;
        let multiple = smallest_multiple(max_factor).unwrap();
        assert_eq!(multiple, 232792560);
    }
}
