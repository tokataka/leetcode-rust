///
/// # 50. Pow(x, n)
///
/// Implement [pow(x, n)](http://www.cplusplus.com/reference/valarray/pow/), which calculates `x` raised to the power `n` (i.e., `x<sup>n</sup>`).
///
/// **Example 1:**
///
/// ```
/// Input: x = 2.00000, n = 10
/// Output: 1024.00000
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: x = 2.10000, n = 3
/// Output: 9.26100
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: x = 2.00000, n = -2
/// Output: 0.25000
/// Explanation: 2-2 = 1/22 = 1/4 = 0.25
///
/// ```
///
/// **Constraints:**
///
/// * `-100.0 < x < 100.0`
/// * `-2<sup>31</sup> <= n <= 2<sup>31</sup>-1`
/// * `n` is an integer.
/// * Either `x` is not zero or `n > 0`.
/// * `-10<sup>4</sup> <= x<sup>n</sup> <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/powx-n/
// discuss: https://leetcode.com/problems/powx-n/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let (mut n, mut x) = match n {
            n if n >= 0 => (n as i64, x),
            n => (-(n as i64), 1.0 / x),
        };

        let mut result = 1.0;

        while n > 0 {
            if n % 2 == 1 {
                result *= x;
            }

            x *= x;
            n /= 2;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_50() {
        let x = 2.00000;
        let n = 10;
        let expected = 1024.00000;
        assert_eq!(Solution::my_pow(x, n), expected);
        let x = 2.10000;
        let n = 3;
        let expected = 9.26100;
        assert_eq!(Solution::my_pow(x, n), expected);
        let x = 2.00000;
        let n = -2;
        let expected = 0.25000;
        assert_eq!(Solution::my_pow(x, n), expected);
    }
}
