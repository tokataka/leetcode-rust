///
/// # 231. Power of Two
///
/// Given an integer `n`, return *`true` if it is a power of two. Otherwise, return `false`*.
///
/// An integer `n` is a power of two, if there exists an integer `x` such that `n == 2<sup>x</sup>`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 1
/// Output: true
/// Explanation: 20 = 1
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 16
/// Output: true
/// Explanation: 24 = 16
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 3
/// Output: false
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

// problem: https://leetcode.com/problems/power-of-two/
// discuss: https://leetcode.com/problems/power-of-two/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        n.count_ones() == 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_231() {
        let n = 1;
        let expected = true;
        assert_eq!(Solution::is_power_of_two(n), expected);
        let n = 16;
        let expected = true;
        assert_eq!(Solution::is_power_of_two(n), expected);
        let n = 3;
        let expected = false;
        assert_eq!(Solution::is_power_of_two(n), expected);
    }
}
