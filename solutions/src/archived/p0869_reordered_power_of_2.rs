///
/// # 869. Reordered Power of 2
///
/// You are given an integer `n`. We reorder the digits in any order (including the original order) such that the leading digit is not zero.
///
/// Return `true` *if and only if we can do this so that the resulting number is a power of two*.
///
/// **Example 1:**
///
/// ```
/// Input: n = 1
/// Output: true
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 10
/// Output: false
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/reordered-power-of-2/
// discuss: https://leetcode.com/problems/reordered-power-of-2/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut freq = [0; 10];
        let mut digits = 0;
        let mut x = n as usize;

        while x > 0 {
            freq[x % 10] += 1;
            digits += 1;
            x /= 10;
        }

        let mut cur = 1;

        loop {
            let mut cur_freq = [0; 10];
            let mut cur_digits = 0;
            let mut x = cur;

            while x > 0 {
                cur_freq[x % 10] += 1;
                cur_digits += 1;
                x /= 10;
            }

            if cur_digits > digits {
                return false;
            }

            if freq == cur_freq {
                return true;
            }

            cur *= 2;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_869() {
        let n = 1;
        let expected = true;
        assert_eq!(Solution::reordered_power_of2(n), expected);
        let n = 10;
        let expected = false;
        assert_eq!(Solution::reordered_power_of2(n), expected);
    }
}
