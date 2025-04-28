///
/// # 29. Divide Two Integers
///
/// Given two integers `dividend` and `divisor`, divide two integers **without** using multiplication, division, and mod operator.
///
/// The integer division should truncate toward zero, which means losing its fractional part. For example, `8.345` would be truncated to `8`, and `-2.7335` would be truncated to `-2`.
///
/// Return *the **quotient** after dividing* `dividend` *by* `divisor`.
///
/// **Note:** Assume we are dealing with an environment that could only store integers within the **32-bit** signed integer range: `[−2<sup>31</sup>, 2<sup>31</sup> − 1]`. For this problem, if the quotient is **strictly greater than** `2<sup>31</sup> - 1`, then return `2<sup>31</sup> - 1`, and if the quotient is **strictly less than** `-2<sup>31</sup>`, then return `-2<sup>31</sup>`.
///
/// **Example 1:**
///
/// ```
/// Input: dividend = 10, divisor = 3
/// Output: 3
/// Explanation: 10/3 = 3.33333.. which is truncated to 3.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: dividend = 7, divisor = -3
/// Output: -2
/// Explanation: 7/-3 = -2.33333.. which is truncated to -2.
///
/// ```
///
/// **Constraints:**
///
/// * `-2<sup>31</sup> <= dividend, divisor <= 2<sup>31</sup> - 1`
/// * `divisor != 0`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-two-integers/
// discuss: https://leetcode.com/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let neg = dividend.signum() != divisor.signum();

        let mut dividend = if dividend > 0 { -dividend } else { dividend };
        let divisor = if divisor > 0 { -divisor } else { divisor };

        let mut result = 0;

        for i in (0..divisor.leading_ones()).rev() {
            if dividend <= divisor << i {
                result += -1 << i;
                dividend -= divisor << i;
            }
        }

        match (result, neg) {
            (result, true) => result,
            (i32::MIN, false) => i32::MAX,
            (result, false) => -result,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {
        let dividend = 10;
        let divisor = 3;
        let expected = 3;
        assert_eq!(Solution::divide(dividend, divisor), expected);
        let dividend = 7;
        let divisor = -3;
        let expected = -2;
        assert_eq!(Solution::divide(dividend, divisor), expected);
        let dividend = -2147483648;
        let divisor = -1;
        let expected = 2147483647;
        assert_eq!(Solution::divide(dividend, divisor), expected);
    }
}
