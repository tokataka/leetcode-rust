///
/// # 1312. Minimum Insertion Steps to Make a String Palindrome
///
/// Given a string `s`. In one step you can insert any character at any index of the string.
///
/// Return *the minimum number of steps* to make `s` palindrome.
///
/// A **Palindrome String** is one that reads the same backward as well as forward.
///
/// **Example 1:**
///
/// ```
/// Input: s = "zzazz"
/// Output: 0
/// Explanation: The string "zzazz" is already palindrome we do not need any insertions.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "mbadm"
/// Output: 2
/// Explanation: String can be "mbdadbm" or "mdbabdm".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "leetcode"
/// Output: 5
/// Explanation: Inserting 5 characters the string becomes "leetcodocteel".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 500`
/// * `s` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/
// discuss: https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        let mut dp = vec![vec![0; n]; n + 1];

        for i in 2..n + 1 {
            for j in 0..n + 1 - i {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i - 1][j + 1]);

                if s[j] == s[j + i - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 2][j + 1]);
                }
            }
        }

        dp[n][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1312() {
        let s = "zzazz".to_owned();
        let expected = 0;
        assert_eq!(Solution::min_insertions(s), expected);
        let s = "mbadm".to_owned();
        let expected = 2;
        assert_eq!(Solution::min_insertions(s), expected);
        let s = "leetcode".to_owned();
        let expected = 5;
        assert_eq!(Solution::min_insertions(s), expected);
    }
}
