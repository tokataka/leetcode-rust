///
/// # 10. Regular Expression Matching
///
/// Given an input string `s` and a pattern `p`, implement regular expression matching with support for `'.'` and `'*'` where:
///
/// * `'.'` Matches any single character.​​​​
/// * `'*'` Matches zero or more of the preceding element.
///
/// The matching should cover the **entire** input string (not partial).
///
/// **Example 1:**
///
/// ```
/// Input: s = "aa", p = "a"
/// Output: false
/// Explanation: "a" does not match the entire string "aa".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "aa", p = "a*"
/// Output: true
/// Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "ab", p = ".*"
/// Output: true
/// Explanation: ".*" means "zero or more (*) of any character (.)".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 20`
/// * `1 <= p.length <= 20`
/// * `s` contains only lowercase English letters.
/// * `p` contains only lowercase English letters, `'.'`, and `'*'`.
/// * It is guaranteed for each appearance of the character `'*'`, there will be a previous valid character to match.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/regular-expression-matching/
// discuss: https://leetcode.com/problems/regular-expression-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut pattern: Vec<(u8, bool)> = vec![];

        for ch in p.bytes() {
            match ch {
                b'*' => pattern.last_mut().unwrap().1 = true,
                ch => pattern.push((ch, false)),
            }
        }

        let mut cur = vec![false; pattern.len() + 1];
        let mut next = vec![false; pattern.len() + 1];
        cur[0] = true;

        for s_ch in s.bytes() {
            next.fill(false);

            for (i, &(p_ch, is_asterisk)) in pattern.iter().enumerate() {
                if !cur[i] {
                    continue;
                }

                if p_ch == s_ch || p_ch == b'.' {
                    next[i + 1] = true;
                    next[i] |= is_asterisk;
                }

                if is_asterisk {
                    cur[i + 1] = true;
                }
            }

            (cur, next) = (next, cur);
        }

        let suffix_asterisk_count = pattern.iter().rev().take_while(|x| x.1).count();

        cur.iter().rev().take(1 + suffix_asterisk_count).any(|&x| x)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        // let s = "aa".to_owned();
        // let p = "a".to_owned();
        // let expected = false;
        // assert_eq!(Solution::is_match(s, p), expected);
        // let s = "aa".to_owned();
        // let p = "a*".to_owned();
        // let expected = true;
        // assert_eq!(Solution::is_match(s, p), expected);
        // let s = "ab".to_owned();
        // let p = ".*".to_owned();
        // let expected = true;
        // assert_eq!(Solution::is_match(s, p), expected);
        let s = "a".to_owned();
        let p = "ab*".to_owned();
        let expected = true;
        assert_eq!(Solution::is_match(s, p), expected);
    }
}
