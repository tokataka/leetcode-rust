///
/// # 1432. Max Difference You Can Get From Changing an Integer
///
/// You are given an integer `num`. You will apply the following steps to `num` **two** separate times:
///
/// * Pick a digit `x (0 <= x <= 9)`.
/// * Pick another digit `y (0 <= y <= 9)`. Note `y` can be equal to `x`.
/// * Replace all the occurrences of `x` in the decimal representation of `num` by `y`.
///
/// Let `a` and `b` be the two results from applying the operation to `num` *independently*.
///
/// Return *the max difference* between `a` and `b`.
///
/// Note that neither `a` nor `b` may have any leading zeros, and **must not** be 0.
///
/// **Example 1:**
///
/// ```
/// Input: num = 555
/// Output: 888
/// Explanation: The first time pick x = 5 and y = 9 and store the new integer in a.
/// The second time pick x = 5 and y = 1 and store the new integer in b.
/// We have now a = 999 and b = 111 and max difference = 888
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: num = 9
/// Output: 8
/// Explanation: The first time pick x = 9 and y = 9 and store the new integer in a.
/// The second time pick x = 9 and y = 1 and store the new integer in b.
/// We have now a = 9 and b = 1 and max difference = 8
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= num <= 10<sup>8</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/
// discuss: https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_diff(mut num: i32) -> i32 {
        let mut num_digits = vec![];

        while num > 0 {
            num_digits.push(num % 10);
            num /= 10;
        }

        let mut a_x = 9;

        for &num in num_digits.iter().rev() {
            if num != 9 {
                a_x = num;
                break;
            }
        }

        let mut b_x = 0;
        let mut b_y = 0;

        if *num_digits.last().unwrap() != 1 {
            b_x = *num_digits.last().unwrap();
            b_y = 1;
        } else {
            for &num in num_digits.iter().rev().skip(1) {
                if num != 0 && num != 1 {
                    b_x = num;
                    break;
                }
            }
        }

        let mut a = 0;
        let mut b = 0;

        for num in num_digits.into_iter().rev() {
            a *= 10;
            b *= 10;

            a += if num == a_x { 9 } else { num };
            b += if num == b_x { b_y } else { num };
        }

        a - b
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1432() {
        let num = 555;
        let expected = 888;
        assert_eq!(Solution::max_diff(num), expected);
        let num = 9;
        let expected = 8;
        assert_eq!(Solution::max_diff(num), expected);
    }
}
