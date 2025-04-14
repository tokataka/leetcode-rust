///
/// # 32. Longest Valid Parentheses
///
/// Given a string containing just the characters `'('` and `')'`, return *the length of the longest valid (well-formed) parentheses* *substring*.
///
/// **Example 1:**
///
/// ```
/// Input: s = "(()"
/// Output: 2
/// Explanation: The longest valid parentheses substring is "()".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = ")()())"
/// Output: 4
/// Explanation: The longest valid parentheses substring is "()()".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = ""
/// Output: 0
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= s.length <= 3 * 10<sup>4</sup>`
/// * `s[i]` is `'('`, or `')'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-valid-parentheses/
// discuss: https://leetcode.com/problems/longest-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut st = Vec::with_capacity(s.len());
        let mut result = 0;
        let mut cur = 0;

        for (i, ch) in s.bytes().enumerate() {
            match ch {
                b'(' => {
                    st.push((i, cur));
                    cur = 0;
                }
                b')' => {
                    if let Some((j, prev)) = st.pop() {
                        cur = prev + i - j + 1;
                        result = result.max(cur);
                    } else {
                        cur = 0;
                    }
                }
                _ => unreachable!(),
            }
        }

        result as i32
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        let s = "(()".to_owned();
        let expected = 2;
        assert_eq!(Solution::longest_valid_parentheses(s), expected);
        let s = ")()())".to_owned();
        let expected = 4;
        assert_eq!(Solution::longest_valid_parentheses(s), expected);
        let s = "".to_owned();
        let expected = 0;
        assert_eq!(Solution::longest_valid_parentheses(s), expected);
    }
}
