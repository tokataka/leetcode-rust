///
/// # 2014. Longest Subsequence Repeated k Times
///
/// You are given a string `s` of length `n`, and an integer `k`. You are tasked to find the **longest subsequence repeated** `k` times in string `s`.
///
/// A **subsequence** is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
///
/// A subsequence `seq` is **repeated** `k` times in the string `s` if `seq * k` is a subsequence of `s`, where `seq * k` represents a string constructed by concatenating `seq` `k` times.
///
/// * For example, `"bba"` is repeated `2` times in the string `"bababcba"`, because the string `"bbabba"`, constructed by concatenating `"bba"` `2` times, is a subsequence of the string `"**b**a**bab**c**ba**"`.
///
/// Return *the **longest subsequence repeated*** `k` *times in string* `s`*. If multiple such subsequences are found, return the **lexicographically largest** one. If there is no such subsequence, return an **empty** string*.
///
/// **Example 1:**
///
/// ![example 1](https://assets.leetcode.com/uploads/2021/08/30/longest-subsequence-repeat-k-times.png)
///
/// ```
/// Input: s = "letsleetcode", k = 2
/// Output: "let"
/// Explanation: There are two longest subsequences repeated 2 times: "let" and "ete".
/// "let" is the lexicographically largest one.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "bb", k = 2
/// Output: "b"
/// Explanation: The longest subsequence repeated 2 times is "b".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "ab", k = 2
/// Output: ""
/// Explanation: There is no subsequence repeated 2 times. Empty string is returned.
///
/// ```
///
/// **Constraints:**
///
/// * `n == s.length`
/// * `2 <= n, k <= 2000`
/// * `2 <= n < k * 8`
/// * `s` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-subsequence-repeated-k-times/
// discuss: https://leetcode.com/problems/longest-subsequence-repeated-k-times/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let mut freq = [0; 26];

        for ch in s.bytes() {
            freq[(ch - b'a') as usize] += 1;
        }

        let mut candidates = freq.map(|x| x / k);
        let max_count = candidates.iter().sum::<i32>();

        fn backtrack(
            cur: &mut String,
            remain: i32,
            candidates: &mut [i32; 26],
            s: &str,
            k: i32,
        ) -> bool {
            if remain == 0 {
                let mut s_iter = s.bytes().peekable();

                'outer: for cur_ch in cur.bytes().cycle().take(cur.len() * k as usize) {
                    for s_ch in s_iter.by_ref() {
                        if cur_ch == s_ch {
                            continue 'outer;
                        }
                    }

                    if s_iter.peek().is_none() {
                        return false;
                    }
                }

                return true;
            }

            for ch in (0..26).rev() {
                if candidates[ch] == 0 {
                    continue;
                }

                candidates[ch] -= 1;
                cur.push((ch as u8 + b'a') as char);

                if backtrack(cur, remain - 1, candidates, s, k) {
                    return true;
                }

                cur.pop();
                candidates[ch] += 1;
            }

            false
        }

        for count in (1..=max_count).rev() {
            let mut result = String::new();
            if backtrack(&mut result, count, &mut candidates, &s, k) {
                return result;
            }
        }

        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2014() {
        let s = "letsleetcode".to_owned();
        let k = 2;
        let expected = "let".to_owned();
        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), expected);
        let s = "bb".to_owned();
        let k = 2;
        let expected = "b".to_owned();
        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), expected);
        let s = "ab".to_owned();
        let k = 2;
        let expected = "".to_owned();
        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), expected);
    }
}
