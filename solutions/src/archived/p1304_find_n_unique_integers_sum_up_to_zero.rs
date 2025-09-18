///
/// # 1304. Find N Unique Integers Sum up to Zero
///
/// Given an integer `n`, return **any** array containing `n` **unique** integers such that they add up to `0`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 5
/// Output: [-7,-1,1,3,4]
/// Explanation: These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 3
/// Output: [-1,0,1]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 1
/// Output: [0]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/
// discuss: https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = vec![];

        if n % 2 == 1 {
            result.push(0);
        }

        for x in 1..=n / 2 {
            result.push(x);
            result.push(-x);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1304() {
        let n = 5;
        let expected = vec![-7, -1, 1, 3, 4];
        assert_eq!(Solution::sum_zero(n), expected);
        let n = 3;
        let expected = vec![-1, 0, 1];
        assert_eq!(Solution::sum_zero(n), expected);
        let n = 1;
        let expected = vec![0];
        assert_eq!(Solution::sum_zero(n), expected);
    }
}
