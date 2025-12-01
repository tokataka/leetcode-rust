///
/// # 940. Distinct Subsequences II
///
/// Given a string s, return *the number of **distinct non-empty subsequences** of* `s`. Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// A **subsequence** of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., `"ace"` is a subsequence of `"abcde"` while `"aec"` is not.
///
/// **Example 1:**
///
/// ```
/// Input: s = "abc"
/// Output: 7
/// Explanation: The 7 distinct subsequences are "a", "b", "c", "ab", "ac", "bc", and "abc".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "aba"
/// Output: 6
/// Explanation: The 6 distinct subsequences are "a", "b", "ab", "aa", "ba", and "aba".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "aaa"
/// Output: 3
/// Explanation: The 3 distinct subsequences are "a", "aa" and "aaa".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 2000`
/// * `s` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-subsequences-ii/
// discuss: https://leetcode.com/problems/distinct-subsequences-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let mut total = 0;
        let mut dp = [0; 26];

        for ch in s.bytes() {
            let ch = ch as usize - b'a' as usize;

            let dup = dp[ch];
            dp[ch] = (total + 1) % MOD;
            total = (total * 2 % MOD + MOD - dup + 1) % MOD;
        }

        total
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_940() {
        // assert_eq!(Solution::distinct_subseq_ii("abc".to_owned()), 7);
        // assert_eq!(Solution::distinct_subseq_ii("aba".to_owned()), 6);
        // assert_eq!(Solution::distinct_subseq_ii("aaa".to_owned()), 3);
        assert_eq!(Solution::distinct_subseq_ii("zchmliaqdgvwncfatcfivphddpzjkgyygueikthqzyeeiebczqbqhdytkoawkehkbizdmcnilcjjlpoeoqqoqpswtqdpvszfaksn".to_owned()), 97915677);
    }
}
