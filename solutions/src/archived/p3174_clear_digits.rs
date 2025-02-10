///
/// # 3174. Clear Digits
///
/// You are given a string `s`.
///
/// Your task is to remove **all** digits by doing this operation repeatedly:
///
/// * Delete the *first* digit and the **closest** **non-digit** character to its *left*.
///
/// Return the resulting string after removing all digits.
///
/// **Example 1:**
///
/// **Input:** s = "abc"
///
/// **Output:** "abc"
///
/// **Explanation:**
///
/// There is no digit in the string.
///
/// **Example 2:**
///
/// **Input:** s = "cb34"
///
/// **Output:** ""
///
/// **Explanation:**
///
/// First, we apply the operation on `s[2]`, and `s` becomes `"c4"`.
///
/// Then we apply the operation on `s[1]`, and `s` becomes `""`.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 100`
/// * `s` consists only of lowercase English letters and digits.
/// * The input is generated such that it is possible to delete all digits.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/clear-digits/
// discuss: https://leetcode.com/problems/clear-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut result = String::new();

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                result.pop();
            } else {
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
    fn test_3174() {
        let s = "abc".to_owned();
        let expected = "abc".to_owned();
        assert_eq!(Solution::clear_digits(s), expected);
        let s = "cb34".to_owned();
        let expected = "".to_owned();
        assert_eq!(Solution::clear_digits(s), expected);
    }
}
