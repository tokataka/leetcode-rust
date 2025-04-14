///
/// # 8. String to Integer (atoi)
///
/// Implement the `myAtoi(string s)` function, which converts a string to a 32-bit signed integer.
///
/// The algorithm for `myAtoi(string s)` is as follows:
///
/// 1. **Whitespace**: Ignore any leading whitespace (`" "`).
/// 2. **Signedness**: Determine the sign by checking if the next character is `'-'` or `'+'`, assuming positivity if neither present.
/// 3. **Conversion**: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
/// 4. **Rounding**: If the integer is out of the 32-bit signed integer range `[-2<sup>31</sup>, 2<sup>31</sup> - 1]`, then round the integer to remain in the range. Specifically, integers less than `-2<sup>31</sup>` should be rounded to `-2<sup>31</sup>`, and integers greater than `2<sup>31</sup> - 1` should be rounded to `2<sup>31</sup> - 1`.
///
/// Return the integer as the final result.
///
/// **Example 1:**
///
/// **Input:** s = "42"
///
/// **Output:** 42
///
/// **Explanation:**
///
/// ```
/// The underlined characters are what is read in and the caret is the current reader position.
/// Step 1: "42" (no characters read because there is no leading whitespace)
///          ^
/// Step 2: "42" (no characters read because there is neither a '-' nor '+')
///          ^
/// Step 3: "42" ("42" is read in)
///            ^
///
/// ```
///
/// **Example 2:**
///
/// **Input:** s = " -042"
///
/// **Output:** -42
///
/// **Explanation:**
///
/// ```
/// Step 1: "   -042" (leading whitespace is read and ignored)
///             ^
/// Step 2: "   -042" ('-' is read, so the result should be negative)
///              ^
/// Step 3: "   -042" ("042" is read in, leading zeros ignored in the result)
///                ^
///
/// ```
///
/// **Example 3:**
///
/// **Input:** s = "1337c0d3"
///
/// **Output:** 1337
///
/// **Explanation:**
///
/// ```
/// Step 1: "1337c0d3" (no characters read because there is no leading whitespace)
///          ^
/// Step 2: "1337c0d3" (no characters read because there is neither a '-' nor '+')
///          ^
/// Step 3: "1337c0d3" ("1337" is read in; reading stops because the next character is a non-digit)
///              ^
///
/// ```
///
/// **Example 4:**
///
/// **Input:** s = "0-1"
///
/// **Output:** 0
///
/// **Explanation:**
///
/// ```
/// Step 1: "0-1" (no characters read because there is no leading whitespace)
///          ^
/// Step 2: "0-1" (no characters read because there is neither a '-' nor '+')
///          ^
/// Step 3: "0-1" ("0" is read in; reading stops because the next character is a non-digit)
///           ^
///
/// ```
///
/// **Example 5:**
///
/// **Input:** s = "words and 987"
///
/// **Output:** 0
///
/// **Explanation:**
///
/// Reading stops at the first non-digit character 'w'.
///
/// **Constraints:**
///
/// * `0 <= s.length <= 200`
/// * `s` consists of English letters (lower-case and upper-case), digits (`0-9`), `' '`, `'+'`, `'-'`, and `'.'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/string-to-integer-atoi/
// discuss: https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sign = 1;
        let mut it = s.bytes().peekable();

        while let Some(&b' ') = it.peek() {
            it.next();
        }

        match it.peek() {
            Some(&b'-') => {
                sign = -1;
                it.next();
            }
            Some(&b'+') => {
                it.next();
            }
            _ => (),
        };

        let mut result = 0_i32;

        for x in it {
            if !x.is_ascii_digit() {
                return result;
            }

            let x = (x - b'0') as i32;

            result = result.saturating_mul(10).saturating_add(x * sign);

            if result == i32::MAX || result == i32::MIN {
                return result;
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
    fn test_8() {
        let s = "42".to_owned();
        let expected = 42;
        assert_eq!(Solution::my_atoi(s), expected);
        let s = " -042".to_owned();
        let expected = -42;
        assert_eq!(Solution::my_atoi(s), expected);
        let s = "1337c0d3".to_owned();
        let expected = 1337;
        assert_eq!(Solution::my_atoi(s), expected);
        let s = "0-1".to_owned();
        let expected = 0;
        assert_eq!(Solution::my_atoi(s), expected);
        let s = "words and 987".to_owned();
        let expected = 0;
        assert_eq!(Solution::my_atoi(s), expected);
        let s = "-91283472332".to_owned();
        let expected = -2147483648;
        assert_eq!(Solution::my_atoi(s), expected);
    }
}
