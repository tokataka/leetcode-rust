///
/// # 342. Power of Four
///
/// Given an integer `n`, return *`true` if it is a power of four. Otherwise, return `false`*.
///
/// An integer `n` is a power of four, if there exists an integer `x` such that `n == 4<sup>x</sup>`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 16
/// Output: true
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 5
/// Output: false
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 1
/// Output: true
///
/// ```
///
/// **Constraints:**
///
/// * `-2<sup>31</sup> <= n <= 2<sup>31</sup> - 1`
///
/// **Follow up:** Could you solve it without loops/recursion?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/power-of-four/
// discuss: https://leetcode.com/problems/power-of-four/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && n.count_ones() == 1 && n.leading_zeros() % 2 == 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_342() {
        let n = 16;
        let expected = true;
        assert_eq!(Solution::is_power_of_four(n), expected);
        let n = 5;
        let expected = false;
        assert_eq!(Solution::is_power_of_four(n), expected);
        let n = 1;
        let expected = true;
        assert_eq!(Solution::is_power_of_four(n), expected);
    }
}
