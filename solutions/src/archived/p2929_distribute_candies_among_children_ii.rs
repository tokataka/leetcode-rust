///
/// # 2929. Distribute Candies Among Children II
///
/// You are given two positive integers `n` and `limit`.
///
/// Return *the **total number** of ways to distribute* `n` *candies among* `3` *children such that no child gets more than* `limit` *candies.*
///
/// **Example 1:**
///
/// ```
/// Input: n = 5, limit = 2
/// Output: 3
/// Explanation: There are 3 ways to distribute 5 candies such that no child gets more than 2 candies: (1, 2, 2), (2, 1, 2) and (2, 2, 1).
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 3, limit = 3
/// Output: 10
/// Explanation: There are 10 ways to distribute 3 candies such that no child gets more than 3 candies: (0, 0, 3), (0, 1, 2), (0, 2, 1), (0, 3, 0), (1, 0, 2), (1, 1, 1), (1, 2, 0), (2, 0, 1), (2, 1, 0) and (3, 0, 0).
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>6</sup>`
/// * `1 <= limit <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/distribute-candies-among-children-ii/
// discuss: https://leetcode.com/problems/distribute-candies-among-children-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let n = n as i64;
        let limit = limit as i64;

        let mut result = 0;

        for first in 0..=limit {
            let sum = n - first;

            result += (sum.min(limit) - (sum - limit).max(0) + 1).max(0);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2929() {
        let n = 5;
        let limit = 2;
        let expected = 3;
        assert_eq!(Solution::distribute_candies(n, limit), expected);
        let n = 3;
        let limit = 3;
        let expected = 10;
        assert_eq!(Solution::distribute_candies(n, limit), expected);
    }
}
