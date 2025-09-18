///
/// # 96. Unique Binary Search Trees
///
/// Given an integer `n`, return *the number of structurally unique **BST'**s (binary search trees) which has exactly* `n` *nodes of unique values from* `1` *to* `n`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg)
///
/// ```
/// Input: n = 3
/// Output: 5
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 1
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 19`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-binary-search-trees/
// discuss: https://leetcode.com/problems/unique-binary-search-trees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;

        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for left in 0..=i - 1 {
                dp[i] += dp[left] * dp[i - 1 - left];
            }
        }

        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_96() {
        let n = 3;
        let expected = 5;
        assert_eq!(Solution::num_trees(n), expected);
        let n = 1;
        let expected = 1;
        assert_eq!(Solution::num_trees(n), expected);
    }
}
