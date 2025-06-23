///
/// # 9. Palindrome Number
///
/// Given an integer `x`, return `true` *if* `x` *is a* ***palindrome****, and* `false` *otherwise*.
///
/// **Example 1:**
///
/// ```
/// Input: x = 121
/// Output: true
/// Explanation: 121 reads as 121 from left to right and from right to left.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: x = -121
/// Output: false
/// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: x = 10
/// Output: false
/// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
///
/// ```
///
/// **Constraints:**
///
/// * `-2<sup>31</sup> <= x <= 2<sup>31</sup> - 1`
///
/// **Follow up:** Could you solve it without converting the integer to a string?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-number/
// discuss: https://leetcode.com/problems/palindrome-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut digits = vec![];

        while x > 0 {
            digits.push(x % 10);
            x /= 10;
        }

        for i in 0..(digits.len() + 1) / 2 {
            if digits[i] != digits[digits.len() - 1 - i] {
                return false;
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        let x = 121;
        let expected = true;
        assert_eq!(Solution::is_palindrome(x), expected);
        let x = -121;
        let expected = false;
        assert_eq!(Solution::is_palindrome(x), expected);
        let x = 10;
        let expected = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }
}
