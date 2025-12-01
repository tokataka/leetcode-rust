///
/// # 633. Sum of Square Numbers
///
/// Given a non-negative integer `c`, decide whether there're two integers `a` and `b` such that `a<sup>2</sup> + b<sup>2</sup> = c`.
///
/// **Example 1:**
///
/// ```
/// Input: c = 5
/// Output: true
/// Explanation: 1 * 1 + 2 * 2 = 5
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: c = 3
/// Output: false
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= c <= 2<sup>31</sup> - 1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-square-numbers/
// discuss: https://leetcode.com/problems/sum-of-square-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        for a in ((c + 1) / 2).isqrt()..=c.isqrt() {
            let b = (c - a * a).isqrt();

            if a * a + b * b == c {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_633() {
        let c = 5;
        let expected = true;
        assert_eq!(Solution::judge_square_sum(c), expected);
        let c = 3;
        let expected = false;
        assert_eq!(Solution::judge_square_sum(c), expected);
    }
}
