///
/// # 1317. Convert Integer to the Sum of Two No-Zero Integers
///
/// **No-Zero integer** is a positive integer that **does not contain any `0`** in its decimal representation.
///
/// Given an integer `n`, return *a list of two integers* `[a, b]` *where*:
///
/// * `a` and `b` are **No-Zero integers**.
/// * `a + b = n`
///
/// The test cases are generated so that there is at least one valid solution. If there are many valid solutions, you can return any of them.
///
/// **Example 1:**
///
/// ```
/// Input: n = 2
/// Output: [1,1]
/// Explanation: Let a = 1 and b = 1.
/// Both a and b are no-zero integers, and a + b = 2 = n.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 11
/// Output: [2,9]
/// Explanation: Let a = 2 and b = 9.
/// Both a and b are no-zero integers, and a + b = 11 = n.
/// Note that there are other valid answers as [8, 3] that can be accepted.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/
// discuss: https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn is_no_zero(mut x: i32) -> bool {
            while x > 0 {
                if x % 10 == 0 {
                    return false;
                }

                x /= 10;
            }

            true
        }

        for x in 1.. {
            if is_no_zero(x) && is_no_zero(n - x) {
                return vec![x, n - x];
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
    fn test_1317() {
        let n = 2;
        let expected = vec![1, 1];
        assert_eq!(Solution::get_no_zero_integers(n), expected);
        let n = 11;
        let expected = vec![2, 9];
        assert_eq!(Solution::get_no_zero_integers(n), expected);
    }
}
