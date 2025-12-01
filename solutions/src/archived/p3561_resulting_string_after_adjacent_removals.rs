///
/// # 3561. Resulting String After Adjacent Removals
///
/// You are given a string `s` consisting of lowercase English letters.
///
/// You **must** repeatedly perform the following operation while the string `s` has **at least** two **consecutive** characters:
///
/// * Remove the **leftmost** pair of **adjacent** characters in the string that are **consecutive** in the alphabet, in either order (e.g., `'a'` and `'b'`, or `'b'` and `'a'`).
/// * Shift the remaining characters to the left to fill the gap.
///
/// Return the resulting string after no more operations can be performed.
///
/// **Note:** Consider the alphabet as circular, thus `'a'` and `'z'` are consecutive.
///
/// **Example 1:**
///
/// **Input:** s = "abc"
///
/// **Output:** "c"
///
/// **Explanation:**
///
/// * Remove `"ab"` from the string, leaving `"c"` as the remaining string.
/// * No further operations are possible. Thus, the resulting string after all possible removals is `"c"`.
///
/// **Example 2:**
///
/// **Input:** s = "adcb"
///
/// **Output:** ""
///
/// **Explanation:**
///
/// * Remove `"dc"` from the string, leaving `"ab"` as the remaining string.
/// * Remove `"ab"` from the string, leaving `""` as the remaining string.
/// * No further operations are possible. Thus, the resulting string after all possible removals is `""`.
///
/// **Example 3:**
///
/// **Input:** s = "zadb"
///
/// **Output:** "db"
///
/// **Explanation:**
///
/// * Remove `"za"` from the string, leaving `"db"` as the remaining string.
/// * No further operations are possible. Thus, the resulting string after all possible removals is `"db"`.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s` consists only of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/resulting-string-after-adjacent-removals/
// discuss: https://leetcode.com/problems/resulting-string-after-adjacent-removals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn resulting_string(s: String) -> String {
        let mut result: Vec<u8> = vec![];

        for ch in s.bytes() {
            if let Some(&prev) = result.last() {
                if prev.abs_diff(ch) == 1 || prev.abs_diff(ch) == 25 {
                    result.pop();
                    continue;
                }
            }

            result.push(ch);
        }

        String::from_utf8(result).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3561() {
        let s = "abc".to_owned();
        let expected = "c".to_owned();
        assert_eq!(Solution::resulting_string(s), expected);
        let s = "adcb".to_owned();
        let expected = "".to_owned();
        assert_eq!(Solution::resulting_string(s), expected);
        let s = "zadb".to_owned();
        let expected = "db".to_owned();
        assert_eq!(Solution::resulting_string(s), expected);
    }
}
