///
/// # 1957. Delete Characters to Make Fancy String
///
/// A **fancy string** is a string where no **three** **consecutive** characters are equal.
///
/// Given a string `s`, delete the **minimum** possible number of characters from `s` to make it **fancy**.
///
/// Return *the final string after the deletion*. It can be shown that the answer will always be **unique**.
///
/// **Example 1:**
///
/// ```
/// Input: s = "leeetcode"
/// Output: "leetcode"
/// Explanation:
/// Remove an 'e' from the first group of 'e's to create "leetcode".
/// No three consecutive characters are equal, so return "leetcode".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "aaabaaaa"
/// Output: "aabaa"
/// Explanation:
/// Remove an 'a' from the first group of 'a's to create "aabaaaa".
/// Remove two 'a's from the second group of 'a's to create "aabaa".
/// No three consecutive characters are equal, so return "aabaa".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "aab"
/// Output: "aab"
/// Explanation: No three consecutive characters are equal, so return "aab".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s` consists only of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-characters-to-make-fancy-string/
// discuss: https://leetcode.com/problems/delete-characters-to-make-fancy-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = String::with_capacity(s.len());

        let mut last = ' ';
        let mut count = 0;

        for ch in s.chars() {
            match ch {
                ' ' => continue,
                ch if ch == last => count += 1,
                ch => {
                    last = ch;
                    count = 1;
                }
            }

            if count < 3 {
                result.push(ch);
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
    fn test_1957() {
        let s = "le e etcode".to_owned();
        let expected = "leetcode".to_owned();
        assert_eq!(Solution::make_fancy_string(s), expected);
        let s = " a aab aa aa".to_owned();
        let expected = "aabaa".to_owned();
        assert_eq!(Solution::make_fancy_string(s), expected);
        let s = "aab".to_owned();
        let expected = "aab".to_owned();
        assert_eq!(Solution::make_fancy_string(s), expected);
    }
}
