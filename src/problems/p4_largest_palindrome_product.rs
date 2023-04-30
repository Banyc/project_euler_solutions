//! # Largest palindrome product
//!
//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! ## References
//!
//! - <https://projecteuler.net/problem=4>

pub fn largest_palindrome_product(term_digits: u32) -> usize {
    // l\r 99 98 97
    // 99  .
    // 98  .  .
    // 97  .  .  .
    let max_term = n_iter(term_digits).rev().next().unwrap();
    let mut max_product = 0;
    for (i, l) in n_iter(term_digits).rev().enumerate() {
        for r in ((max_term - i)..=max_term).rev() {
            let product = l * r;
            // println!("{} * {} = {}", l, r, product);
            if is_palindrome(product) {
                max_product = max_product.max(product);
            }
        }
    }
    max_product
}

pub fn n_iter(digits: u32) -> impl DoubleEndedIterator<Item = usize> {
    assert!(digits > 0);
    let min = 10usize.pow(digits - 1);
    let max_e = 10usize.pow(digits);
    min..max_e
}

pub fn is_palindrome(n: usize) -> bool {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 0 {
        let digit = n % 10;
        n /= 10;
        digits.push(digit);
    }
    if digits.is_empty() {
        return true;
    }
    let mut l = 0;
    let mut r = digits.len() - 1;
    while l < r {
        if digits[l] != digits[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        assert_eq!(n_iter(2).next().unwrap(), 10);
        assert_eq!(n_iter(2).last().unwrap(), 99);
    }

    #[test]
    fn palindrome() {
        assert!(is_palindrome(9009));
        assert!(!is_palindrome(9008));
    }

    #[test]
    fn two_digits() {
        let term_digits = 2;
        let product = largest_palindrome_product(term_digits);
        assert_eq!(product, 9009);
    }

    #[test]
    fn three_digits() {
        let term_digits = 3;
        let product = largest_palindrome_product(term_digits);
        assert_eq!(product, 906609);
    }
}
