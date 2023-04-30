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
    let factors = reduce_indivisible(max_factor);
    for n in max_factor..usize::MAX {
        let mut all_divisible = true;
        for factor in &factors {
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

/// # References
///
/// - <https://stackoverflow.com/a/8025847/9920172>
pub fn reduce_indivisible(max_factor: usize) -> Vec<usize> {
    let mut reduced = vec![max_factor];
    for n in (1..max_factor).rev() {
        let mut indivisible = true;
        for reduced_n in &reduced {
            if reduced_n % n == 0 {
                indivisible = false;
                break;
            }
        }
        if indivisible {
            reduced.push(n);
        }
    }
    reduced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce() {
        assert_eq!(reduce_indivisible(10), vec![10, 9, 8, 7, 6]);
    }

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
