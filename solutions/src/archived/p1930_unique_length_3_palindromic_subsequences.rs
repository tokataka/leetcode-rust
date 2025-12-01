///
/// # 1930. Unique Length-3 Palindromic Subsequences
///
/// Given a string `s`, return *the number of **unique palindromes of length three** that are a **subsequence** of* `s`.
///
/// Note that even if there are multiple ways to obtain the same subsequence, it is still only counted **once**.
///
/// A **palindrome** is a string that reads the same forwards and backwards.
///
/// A **subsequence** of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
///
/// * For example, `"ace"` is a subsequence of `"abcde"`.
///
/// **Example 1:**
///
/// ```
/// Input: s = "aabca"
/// Output: 3
/// Explanation: The 3 palindromic subsequences of length 3 are:
/// - "aba" (subsequence of "aabca")
/// - "aaa" (subsequence of "aabca")
/// - "aca" (subsequence of "aabca")
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "adc"
/// Output: 0
/// Explanation: There are no palindromic subsequences of length 3 in "adc".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "bbcbaba"
/// Output: 4
/// Explanation: The 4 palindromic subsequences of length 3 are:
/// - "bbb" (subsequence of "bbcbaba")
/// - "bcb" (subsequence of "bbcbaba")
/// - "bab" (subsequence of "bbcbaba")
/// - "aba" (subsequence of "bbcbaba")
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= s.length <= 10<sup>5</sup>`
/// * `s` consists of only lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
// discuss: https://leetcode.com/problems/unique-length-3-palindromic-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut result = [[false; 26]; 26];

        let mut idx_pairs = [(usize::MAX, usize::MAX); 26];

        for (i, ch) in s.bytes().enumerate() {
            let ch = ch as usize - b'a' as usize;

            if idx_pairs[ch].0 == usize::MAX {
                idx_pairs[ch].0 = i;
            } else {
                for (other_ch, &(_, j)) in idx_pairs.iter().enumerate() {
                    if j != usize::MAX && j > idx_pairs[ch].0 {
                        result[ch][other_ch] = true;
                    }
                }
            }
            idx_pairs[ch].1 = i;
        }

        result.into_iter().flatten().filter(|&x| x).count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1930() {
        assert_eq!(
            Solution::count_palindromic_subsequence("aabca".to_owned()),
            3
        );
        assert_eq!(Solution::count_palindromic_subsequence("adc".to_owned()), 0);
        assert_eq!(
            Solution::count_palindromic_subsequence("bbcbaba".to_owned()),
            4
        );
    }
}
