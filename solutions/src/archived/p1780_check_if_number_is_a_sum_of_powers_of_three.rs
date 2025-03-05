///
/// # 1780. Check if Number is a Sum of Powers of Three
///
/// Given an integer `n`, return `true` *if it is possible to represent* `n` *as the sum of distinct powers of three.* Otherwise, return `false`.
///
/// An integer `y` is a power of three if there exists an integer `x` such that `y == 3<sup>x</sup>`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 12
/// Output: true
/// Explanation: 12 = 31 + 32
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 91
/// Output: true
/// Explanation: 91 = 30 + 32 + 34
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 21
/// Output: false
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>7</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/
// discuss: https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            while n % 3 == 0 {
                n /= 3;
            }

            if n % 3 != 1 {
                return false;
            }

            n -= 1;
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1780() {
        let n = 12;
        let expected = true;
        assert_eq!(Solution::check_powers_of_three(n), expected);
        let n = 91;
        let expected = true;
        assert_eq!(Solution::check_powers_of_three(n), expected);
        let n = 21;
        let expected = false;
        assert_eq!(Solution::check_powers_of_three(n), expected);
    }
}
