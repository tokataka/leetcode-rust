///
/// # 906. Super Palindromes
///
/// Let's say a positive integer is a **super-palindrome** if it is a palindrome, and it is also the square of a palindrome.
///
/// Given two positive integers `left` and `right` represented as strings, return *the number of **super-palindromes** integers in the inclusive range* `[left, right]`.
///
/// **Example 1:**
///
/// ```
/// Input: left = "4", right = "1000"
/// Output: 4
/// Explanation: 4, 9, 121, and 484 are superpalindromes.
/// Note that 676 is not a superpalindrome: 26 * 26 = 676, but 26 is not a palindrome.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: left = "1", right = "2"
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= left.length, right.length <= 18`
/// * `left` and `right` consist of only digits.
/// * `left` and `right` cannot have leading zeros.
/// * `left` and `right` represent integers in the range `[1, 10<sup>18</sup> - 1]`.
/// * `left` is less than or equal to `right`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/super-palindromes/
// discuss: https://leetcode.com/problems/super-palindromes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        fn is_palindrome(x: i64) -> bool {
            let mut x = x;
            let log = x.ilog10();
            let mut d = 10i64.pow(log);

            for _ in 0..log.div_ceil(2) {
                if x / d != x % 10 {
                    return false;
                }

                x = (x % d) / 10;
                d /= 100;
            }

            true
        }

        let mut super_palindromes = vec![];

        for digit_len in 0..5 {
            let d = 10i64.pow(digit_len);

            for left in d / 10..d {
                let mut x = left;
                let mut right = 0;

                while x > 0 {
                    right = right * 10 + x % 10;
                    x /= 10;
                }

                let cur = left * d + right;

                if cur > 0 && is_palindrome(cur * cur) {
                    super_palindromes.push(cur * cur);
                }

                for k in 0..=9 {
                    let cur = left * d * 10 + k * d + right;

                    if cur > 0 && is_palindrome(cur * cur) {
                        super_palindromes.push(cur * cur);
                    }
                }
            }
        }

        super_palindromes.sort_unstable();

        let left = left.parse::<i64>().unwrap();
        let right = right.parse::<i64>().unwrap();

        (super_palindromes.partition_point(|&x| x <= right)
            - super_palindromes.partition_point(|&x| x < left)) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_906() {
        let left = "4".to_owned();
        let right = "1000".to_owned();
        let expected = 4;
        assert_eq!(Solution::superpalindromes_in_range(left, right), expected);
        let left = "1".to_owned();
        let right = "2".to_owned();
        let expected = 1;
        assert_eq!(Solution::superpalindromes_in_range(left, right), expected);
    }
}
