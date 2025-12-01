///
/// # 3228. Maximum Number of Operations to Move Ones to the End
///
/// You are given a binary string `s`.
///
/// You can perform the following operation on the string **any** number of times:
///
/// * Choose **any** index `i` from the string where `i + 1 < s.length` such that `s[i] == '1'` and `s[i + 1] == '0'`.
/// * Move the character `s[i]` to the **right** until it reaches the end of the string or another `'1'`. For example, for `s = "010010"`, if we choose `i = 1`, the resulting string will be `s = "0**001**10"`.
///
/// Return the **maximum** number of operations that you can perform.
///
/// **Example 1:**
///
/// **Input:** s = "1001101"
///
/// **Output:** 4
///
/// **Explanation:**
///
/// We can perform the following operations:
///
/// * Choose index `i = 0`. The resulting string is `s = "**001**1101"`.
/// * Choose index `i = 4`. The resulting string is `s = "0011**01**1"`.
/// * Choose index `i = 3`. The resulting string is `s = "001**01**11"`.
/// * Choose index `i = 2`. The resulting string is `s = "00**01**111"`.
///
/// **Example 2:**
///
/// **Input:** s = "00111"
///
/// **Output:** 0
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s[i]` is either `'0'` or `'1'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/
// discuss: https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        s.trim_end_matches('1')
            .split('0')
            .map(|split| split.len() as i32)
            .filter(|&split| split > 0)
            .fold((0, 0), |(result, ones), x| (result + ones + x, ones + x))
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3228() {
        let s = "1001101".to_owned();
        let expected = 4;
        assert_eq!(Solution::max_operations(s), expected);
        let s = "00111".to_owned();
        let expected = 0;
        assert_eq!(Solution::max_operations(s), expected);
    }
}
