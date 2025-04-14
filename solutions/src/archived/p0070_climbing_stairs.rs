///
/// # 70. Climbing Stairs
///
/// You are climbing a staircase. It takes `n` steps to reach the top.
///
/// Each time you can either climb `1` or `2` steps. In how many distinct ways can you climb to the top?
///
/// **Example 1:**
///
/// ```
/// Input: n = 2
/// Output: 2
/// Explanation: There are two ways to climb to the top.
/// 1. 1 step + 1 step
/// 2. 2 steps
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 3
/// Output: 3
/// Explanation: There are three ways to climb to the top.
/// 1. 1 step + 1 step + 1 step
/// 2. 1 step + 2 steps
/// 3. 2 steps + 1 step
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 45`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/climbing-stairs/
// discuss: https://leetcode.com/problems/climbing-stairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut cache = vec![0; n  + 2];
        cache[0] = 1;

        for i in 0..n {
            cache[i+1] += cache[i];
            cache[i+2] += cache[i];
        }

        cache[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70() {
        let n = 2;
        let expected = 2;
        assert_eq!(Solution::climb_stairs(n), expected);
        let n = 3;
        let expected = 3;
        assert_eq!(Solution::climb_stairs(n), expected);
    }
}
