///
/// # 2472. Maximum Number of Non-overlapping Palindrome Substrings
///
/// You are given a string `s` and a **positive** integer `k`.
///
/// Select a set of **non-overlapping** substrings from the string `s` that satisfy the following conditions:
///
/// * The **length** of each substring is **at least** `k`.
/// * Each substring is a **palindrome**.
///
/// Return *the **maximum** number of substrings in an optimal selection*.
///
/// A **substring** is a contiguous sequence of characters within a string.
///
/// **Example 1:**
///
/// ```
/// Input: s = "abaccdbbd", k = 3
/// Output: 2
/// Explanation: We can select the substrings underlined in s = "abaccdbbd". Both "aba" and "dbbd" are palindromes and have a length of at least k = 3.
/// It can be shown that we cannot find a selection with more than two valid substrings.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "adbcda", k = 2
/// Output: 0
/// Explanation: There is no palindrome substring of length at least 2 in the string.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= k <= s.length <= 2000`
/// * `s` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-non-overlapping-palindrome-substrings/
// discuss: https://leetcode.com/problems/maximum-number-of-non-overlapping-palindrome-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s = s.as_bytes();

        let mut result = 0;
        let mut cur = 0;

        'outer: while cur + k <= s.len() {
            for l in [k, k + 1] {
                if cur + l > s.len() {
                    continue;
                }

                if (s.iter().skip(cur).take(l / 2))
                    .zip(s.iter().skip(cur).take(l).rev())
                    .all(|(a, b)| a == b)
                {
                    result += 1;
                    cur += l;
                    continue 'outer;
                }
            }

            cur += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2472() {
        let s = "abaccdbbd".to_owned();
        let k = 3;
        let expected = 2;
        assert_eq!(Solution::max_palindromes(s, k), expected);
        let s = "adbcda".to_owned();
        let k = 2;
        let expected = 0;
        assert_eq!(Solution::max_palindromes(s, k), expected);
    }
}
