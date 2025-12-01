///
/// # 3518. Smallest Palindromic Rearrangement II
///
/// You are given a **palindromic** string `s` and an integer `k`.
///
/// Return the **k-th** **lexicographically smallest** palindromic permutation of `s`. If there are fewer than `k` distinct palindromic permutations, return an empty string.
///
/// **Note:** Different rearrangements that yield the same palindromic string are considered identical and are counted once.
///
/// **Example 1:**
///
/// **Input:** s = "abba", k = 2
///
/// **Output:** "baab"
///
/// **Explanation:**
///
/// * The two distinct palindromic rearrangements of `"abba"` are `"abba"` and `"baab"`.
/// * Lexicographically, `"abba"` comes before `"baab"`. Since `k = 2`, the output is `"baab"`.
///
/// **Example 2:**
///
/// **Input:** s = "aa", k = 2
///
/// **Output:** ""
///
/// **Explanation:**
///
/// * There is only one palindromic rearrangement: `"aa"`.
/// * The output is an empty string since `k = 2` exceeds the number of possible rearrangements.
///
/// **Example 3:**
///
/// **Input:** s = "bacab", k = 1
///
/// **Output:** "abcba"
///
/// **Explanation:**
///
/// * The two distinct palindromic rearrangements of `"bacab"` are `"abcba"` and `"bacab"`.
/// * Lexicographically, `"abcba"` comes before `"bacab"`. Since `k = 1`, the output is `"abcba"`.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>4</sup>`
/// * `s` consists of lowercase English letters.
/// * `s` is guaranteed to be palindromic.
/// * `1 <= k <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-palindromic-rearrangement-ii/
// discuss: https://leetcode.com/problems/smallest-palindromic-rearrangement-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let n = s.len();

        let mut half = s[..n / 2].to_vec();
        half.sort_unstable();

        let mut k = k as i64 - 1;
        let mut rearrange = [0; 26];
        let mut count = 0;
        let mut perm = 1;

        while perm <= k {
            let ch_idx = match half.pop() {
                Some(ch) => (ch - b'a') as usize,
                None => return String::new(),
            };

            count += 1;
            rearrange[ch_idx] += 1;

            perm *= count;
            perm /= rearrange[ch_idx];
        }

        while count > 0 {
            for ch_idx in 0..26 {
                if rearrange[ch_idx] == 0 {
                    continue;
                }

                let mut next_perm = perm;
                next_perm *= rearrange[ch_idx];
                next_perm /= count;

                if k < next_perm {
                    rearrange[ch_idx] -= 1;
                    count -= 1;

                    half.push(ch_idx as u8 + b'a');
                    perm = next_perm;

                    break;
                }

                k -= next_perm;
            }
        }

        s[..n / 2].copy_from_slice(&half);
        half.reverse();
        s[n.div_ceil(2)..].copy_from_slice(&half);

        String::from_utf8(s).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3518() {
        let s = "abba".to_owned();
        let k = 2;
        let expected = "baab".to_owned();
        assert_eq!(Solution::smallest_palindrome(s, k), expected);
        let s = "aa".to_owned();
        let k = 2;
        let expected = "".to_owned();
        assert_eq!(Solution::smallest_palindrome(s, k), expected);
        let s = "bacab".to_owned();
        let k = 1;
        let expected = "abcba".to_owned();
        assert_eq!(Solution::smallest_palindrome(s, k), expected);
        let s = "xxnfnxx".to_owned();
        let k = 3;
        let expected = "xxnfnxx".to_owned();
        assert_eq!(Solution::smallest_palindrome(s, k), expected);
    }
}
