//! # Sum square difference
//!
//! The sum of the squares of the first ten natural numbers is,
//! $$
//! 1^2 + 2^2 + \cdots + 10^2 = 385
//! $$
//!
//! The square of the sum of the first ten natural numbers is,
//! $$
//! (1 + 2 + \cdots + 10)^2 = 55^2 = 3025
//! $$
//!
//! Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
//! $3025 - 385 = 2640$.
//!
//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
//!
//! ## References
//!
//! - <https://projecteuler.net/problem=6>

pub fn sum_square_difference(max_term: usize) -> usize {
    let sum_of_squares = (1..=max_term).map(|i| i.pow(2)).sum::<usize>();
    let square_of_sum = (1..=max_term).sum::<usize>().pow(2);
    square_of_sum - sum_of_squares
}

pub fn sum_square_difference_o1(max_term: usize) -> usize {
    sum(max_term).pow(2) - sum_of_squares(max_term)
}

/// $$
/// \frac{n (n + 1)}{2}
/// $$
///
/// # References
///
/// - <https://lucidmanager.org/data-science/project-euler-6/>
pub fn sum(max_term: usize) -> usize {
    max_term * (max_term + 1) / 2
}

/// $$
/// \frac{n (n + 1) (2n + 1)}{6}
/// $$
///
/// # References
///
/// - <https://lucidmanager.org/data-science/project-euler-6/>
pub fn sum_of_squares(max_term: usize) -> usize {
    max_term * (max_term + 1) * (2 * max_term + 1) / 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_term_10() {
        let max_term = 10;
        let diff = sum_square_difference(max_term);
        assert_eq!(diff, 2640);
        let diff = sum_square_difference_o1(max_term);
        assert_eq!(diff, 2640);
    }

    #[test]
    fn max_term_100() {
        let max_term = 100;
        let diff = sum_square_difference(max_term);
        assert_eq!(diff, 25164150);
        let diff = sum_square_difference_o1(max_term);
        assert_eq!(diff, 25164150);
    }
}
