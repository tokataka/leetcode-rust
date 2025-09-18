///
/// # 2749. Minimum Operations to Make the Integer Zero
///
/// You are given two integers `num1` and `num2`.
///
/// In one operation, you can choose integer `i` in the range `[0, 60]` and subtract `2<sup>i</sup> + num2` from `num1`.
///
/// Return *the integer denoting the **minimum** number of operations needed to make* `num1` *equal to* `0`.
///
/// If it is impossible to make `num1` equal to `0`, return `-1`.
///
/// **Example 1:**
///
/// ```
/// Input: num1 = 3, num2 = -2
/// Output: 3
/// Explanation: We can make 3 equal to 0 with the following operations:
/// - We choose i = 2 and subtract 22 + (-2) from 3, 3 - (4 + (-2)) = 1.
/// - We choose i = 2 and subtract 22 + (-2) from 1, 1 - (4 + (-2)) = -1.
/// - We choose i = 0 and subtract 20 + (-2) from -1, (-1) - (1 + (-2)) = 0.
/// It can be proven, that 3 is the minimum number of operations that we need to perform.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: num1 = 5, num2 = 7
/// Output: -1
/// Explanation: It can be proven, that it is impossible to make 5 equal to 0 with the given operation.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= num1 <= 10<sup>9</sup>`
/// * `-10<sup>9</sup> <= num2 <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/
// discuss: https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let mut num1 = num1 as i64;
        let num2 = num2 as i64;

        for i in 1.. {
            num1 -= num2;

            if num1 < 0 || i > num1 {
                return -1;
            }

            if i as u32 >= num1.count_ones() {
                return i as i32;
            }
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2749() {
        let num1 = 3;
        let num2 = -2;
        let expected = 3;
        assert_eq!(Solution::make_the_integer_zero(num1, num2), expected);
        let num1 = 5;
        let num2 = 7;
        let expected = -1;
        assert_eq!(Solution::make_the_integer_zero(num1, num2), expected);
    }
}
