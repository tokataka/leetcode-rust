///
/// # 326. Power of Three
///
/// Given an integer `n`, return *`true` if it is a power of three. Otherwise, return `false`*.
///
/// An integer `n` is a power of three, if there exists an integer `x` such that `n == 3<sup>x</sup>`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 27
/// Output: true
/// Explanation: 27 = 33
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 0
/// Output: false
/// Explanation: There is no x where 3x = 0.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = -1
/// Output: false
/// Explanation: There is no x where 3x = (-1).
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

// problem: https://leetcode.com/problems/power-of-three/
// discuss: https://leetcode.com/problems/power-of-three/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1162261467 % n == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_326() {
        let n = 27;
        let expected = true;
        assert_eq!(Solution::is_power_of_three(n), expected);
        let n = 0;
        let expected = false;
        assert_eq!(Solution::is_power_of_three(n), expected);
        let n = -1;
        let expected = false;
        assert_eq!(Solution::is_power_of_three(n), expected);
    }
}
