///
/// # 3370. Smallest Number With All Set Bits
///
/// You are given a *positive* number `n`.
///
/// Return the **smallest** number `x` **greater than** or **equal to** `n`, such that the binary representation of `x` contains only set bits
///
/// **Example 1:**
///
/// **Input:** n = 5
///
/// **Output:** 7
///
/// **Explanation:**
///
/// The binary representation of 7 is `"111"`.
///
/// **Example 2:**
///
/// **Input:** n = 10
///
/// **Output:** 15
///
/// **Explanation:**
///
/// The binary representation of 15 is `"1111"`.
///
/// **Example 3:**
///
/// **Input:** n = 3
///
/// **Output:** 3
///
/// **Explanation:**
///
/// The binary representation of 3 is `"11"`.
///
/// **Constraints:**
///
/// * `1 <= n <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-number-with-all-set-bits/
// discuss: https://leetcode.com/problems/smallest-number-with-all-set-bits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        (1 << (n.ilog2() + 1)) - 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3370() {
        let n = 5;
        let expected = 7;
        assert_eq!(Solution::smallest_number(n), expected);
        let n = 10;
        let expected = 15;
        assert_eq!(Solution::smallest_number(n), expected);
        let n = 3;
        let expected = 3;
        assert_eq!(Solution::smallest_number(n), expected);
    }
}
