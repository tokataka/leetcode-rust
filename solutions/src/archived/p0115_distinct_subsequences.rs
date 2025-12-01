///
/// # 115. Distinct Subsequences
///
/// Given two strings s and t, return *the number of distinct* ***subsequences*** *of* s *which equals* t.
///
/// The test cases are generated so that the answer fits on a 32-bit signed integer.
///
/// **Example 1:**
///
/// ```
/// Input: s = "rabbbit", t = "rabbit"
/// Output: 3
/// Explanation:
/// As shown below, there are 3 ways you can generate "rabbit" from s.
/// rabbbit
/// rabbbit
/// rabbbit
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "babgbag", t = "bag"
/// Output: 5
/// Explanation:
/// As shown below, there are 5 ways you can generate "bag" from s.
/// babgbag
/// babgbag
/// babgbag
/// babgbag
/// babgbag
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length, t.length <= 1000`
/// * `s` and `t` consist of English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-subsequences/
// discuss: https://leetcode.com/problems/distinct-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        if t.len() > s.len() {
            return 0;
        }

        let mut dp = vec![1; s.len()];

        for t_idx in 0..t.len() {
            for s_idx in t_idx..s.len() {
                let i = s_idx - t_idx;

                if s[s_idx] != t[t_idx] {
                    dp[i] = 0;
                };

                if i > 0 {
                    dp[i] += dp[i - 1];
                }
            }
        }

        dp[s.len() - t.len()]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_115() {
        let s = "rabbbit".to_owned();
        let t = "rabbit".to_owned();
        let expected = 3;
        assert_eq!(Solution::num_distinct(s, t), expected);
        let s = "babgbag".to_owned();
        let t = "bag".to_owned();
        let expected = 5;
        assert_eq!(Solution::num_distinct(s, t), expected);
        let s = "fff".to_owned();
        let t = "ffff".to_owned();
        let expected = 0;
        assert_eq!(Solution::num_distinct(s, t), expected);
    }
}
