///
/// # 1220. Count Vowels Permutation
///
/// Given an integer `n`, your task is to count how many strings of length `n` can be formed under the following rules:
///
/// * Each character is a lower case vowel (`'a'`, `'e'`, `'i'`, `'o'`, `'u'`)
/// * Each vowel `'a'` may only be followed by an `'e'`.
/// * Each vowel `'e'` may only be followed by an `'a'` or an `'i'`.
/// * Each vowel `'i'` **may not** be followed by another `'i'`.
/// * Each vowel `'o'` may only be followed by an `'i'` or a `'u'`.
/// * Each vowel `'u'` may only be followed by an `'a'`.
///
/// Since the answer may be too large, return it modulo `10^9 + 7`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 1
/// Output: 5
/// Explanation: All possible strings are: "a", "e", "i" , "o" and "u".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 2
/// Output: 10
/// Explanation: All possible strings are: "ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" and "ua".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 5
/// Output: 68
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 2 * 10^4`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-vowels-permutation/
// discuss: https://leetcode.com/problems/count-vowels-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut dp = [1, 1, 1, 1, 1];

        for _ in 1..n {
            dp = [
                (dp[1] + dp[2] + dp[4]) % MOD,
                (dp[0] + dp[2]) % MOD,
                (dp[1] + dp[3]) % MOD,
                dp[2],
                (dp[2] + dp[3]) % MOD,
            ];
        }

        (dp.iter().sum::<i64>() % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1220() {
        let n = 1;
        let expected = 5;
        assert_eq!(Solution::count_vowel_permutation(n), expected);
        let n = 2;
        let expected = 10;
        assert_eq!(Solution::count_vowel_permutation(n), expected);
        let n = 5;
        let expected = 68;
        assert_eq!(Solution::count_vowel_permutation(n), expected);
    }
}
