///
/// # 1092. Shortest Common Supersequence
///
/// Given two strings `str1` and `str2`, return *the shortest string that has both* `str1` *and* `str2` *as **subsequences***. If there are multiple valid strings, return **any** of them.
///
/// A string `s` is a **subsequence** of string `t` if deleting some number of characters from `t` (possibly `0`) results in the string `s`.
///
/// **Example 1:**
///
/// ```
/// Input: str1 = "abac", str2 = "cab"
/// Output: "cabac"
/// Explanation:
/// str1 = "abac" is a subsequence of "cabac" because we can delete the first "c".
/// str2 = "cab" is a subsequence of "cabac" because we can delete the last "ac".
/// The answer provided is the shortest such string that satisfies these properties.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: str1 = "aaaaaaaa", str2 = "aaaaaaaa"
/// Output: "aaaaaaaa"
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= str1.length, str2.length <= 1000`
/// * `str1` and `str2` consist of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-common-supersequence/
// discuss: https://leetcode.com/problems/shortest-common-supersequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];

        for (idx1, ch1) in str1.bytes().enumerate().rev() {
            for (idx2, ch2) in str2.bytes().enumerate().rev() {
                if ch1 == ch2 {
                    dp[idx1][idx2] = dp[idx1 + 1][idx2 + 1] + 1;
                } else {
                    dp[idx1][idx2] = dp[idx1 + 1][idx2].max(dp[idx1][idx2 + 1]);
                }
            }
        }

        let mut idx1 = 0;
        let mut idx2 = 0;

        let mut result = String::with_capacity(str1.len() + str2.len());

        while idx1 < str1.len() || idx2 < str2.len() {
            if idx1 < str1.len() && dp[idx1][idx2] == dp[idx1 + 1][idx2] {
                result.push(str1.chars().nth(idx1).unwrap());
                idx1 += 1;
            } else if idx2 < str2.len() && dp[idx1][idx2] == dp[idx1][idx2 + 1] {
                result.push(str2.chars().nth(idx2).unwrap());
                idx2 += 1;
            } else {
                result.push(str1.chars().nth(idx1).unwrap());
                idx1 += 1;
                idx2 += 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1092() {
        let str1 = "abac".to_owned();
        let str2 = "cab".to_owned();
        let expected = "cabac".to_owned();
        assert_eq!(
            Solution::shortest_common_supersequence(str1, str2),
            expected
        );
        let str1 = "aaaaaaaaa".to_owned();
        let str2 = "aaaaaaaa".to_owned();
        let expected = "aaaaaaaaa".to_owned();
        assert_eq!(
            Solution::shortest_common_supersequence(str1, str2),
            expected
        );
    }
}
