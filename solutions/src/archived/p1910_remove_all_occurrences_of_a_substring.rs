///
/// # 1910. Remove All Occurrences of a Substring
///
/// Given two strings `s` and `part`, perform the following operation on `s` until **all** occurrences of the substring `part` are removed:
///
/// * Find the **leftmost** occurrence of the substring `part` and **remove** it from `s`.
///
/// Return `s` *after removing all occurrences of* `part`.
///
/// A **substring** is a contiguous sequence of characters in a string.
///
/// **Example 1:**
///
/// ```
/// Input: s = "daabcbaabcbc", part = "abc"
/// Output: "dab"
/// Explanation: The following operations are done:
/// - s = "daabcbaabcbc", remove "abc" starting at index 2, so s = "dabaabcbc".
/// - s = "dabaabcbc", remove "abc" starting at index 4, so s = "dababc".
/// - s = "dababc", remove "abc" starting at index 3, so s = "dab".
/// Now s has no occurrences of "abc".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "axxxxyyyyb", part = "xy"
/// Output: "ab"
/// Explanation: The following operations are done:
/// - s = "axxxxyyyyb", remove "xy" starting at index 4 so s = "axxxyyyb".
/// - s = "axxxyyyb", remove "xy" starting at index 3 so s = "axxyyb".
/// - s = "axxyyb", remove "xy" starting at index 2 so s = "axyb".
/// - s = "axyb", remove "xy" starting at index 1 so s = "ab".
/// Now s has no occurrences of "xy".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 1000`
/// * `1 <= part.length <= 1000`
/// * `s`​​​​​​ and `part` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
// discuss: https://leetcode.com/problems/remove-all-occurrences-of-a-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut temp = vec![];
        let last_part = part.chars().last().unwrap();

        for ch in s.chars() {
            if ch == last_part {
                if part.chars().rev().skip(1).all(|p| {
                    if let Some(top) = result.pop() {
                        temp.push(top);

                        p == top
                    } else {
                        false
                    }
                }) {
                    temp.clear();
                    continue;
                } else {
                    result.extend(temp.drain(..).rev());
                }
            }

            result.push(ch);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1910() {
        let s = "daabcbaabcbc".to_owned();
        let part = "abc".to_owned();
        let expected = "dab".to_owned();
        assert_eq!(Solution::remove_occurrences(s, part), expected);
        let s = "axxxxyyyyb".to_owned();
        let part = "xy".to_owned();
        let expected = "ab".to_owned();
        assert_eq!(Solution::remove_occurrences(s, part), expected);
    }
}
