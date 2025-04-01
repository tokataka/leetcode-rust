///
/// # 20. Valid Parentheses
///
/// Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.
///
/// An input string is valid if:
///
/// 1. Open brackets must be closed by the same type of brackets.
/// 2. Open brackets must be closed in the correct order.
/// 3. Every close bracket has a corresponding open bracket of the same type.
///
/// **Example 1:**
///
/// **Input:** s = "()"
///
/// **Output:** true
///
/// **Example 2:**
///
/// **Input:** s = "()[]{}"
///
/// **Output:** true
///
/// **Example 3:**
///
/// **Input:** s = "(]"
///
/// **Output:** false
///
/// **Example 4:**
///
/// **Input:** s = "([])"
///
/// **Output:** true
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>4</sup>`
/// * `s` consists of parentheses only `'()[]{}'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-parentheses/
// discuss: https://leetcode.com/problems/valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = vec![];

        let mut map = [0; 128];
        map[b')' as usize] = b'(';
        map[b']' as usize] = b'[';
        map[b'}' as usize] = b'{';

        for ch in s.bytes() {
            match ch {
                ch @ (b'(' | b'[' | b'{') => st.push(ch),
                ch @ (b')' | b']' | b'}') => {
                    if let Some(last) = st.pop() {
                        if map[ch as usize] != last {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        st.is_empty()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        let s = "()".to_owned();
        let expected = true;
        assert_eq!(Solution::is_valid(s), expected);
        let s = "()[]{}".to_owned();
        let expected = true;
        assert_eq!(Solution::is_valid(s), expected);
        let s = "(]".to_owned();
        let expected = false;
        assert_eq!(Solution::is_valid(s), expected);
        let s = "([])".to_owned();
        let expected = true;
        assert_eq!(Solution::is_valid(s), expected);
    }
}
