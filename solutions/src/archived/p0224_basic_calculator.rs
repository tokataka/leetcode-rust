///
/// # 224. Basic Calculator
///
/// Given a string `s` representing a valid expression, implement a basic calculator to evaluate it, and return *the result of the evaluation*.
///
/// **Note:** You are **not** allowed to use any built-in function which evaluates strings as mathematical expressions, such as `eval()`.
///
/// **Example 1:**
///
/// ```
/// Input: s = "1 + 1"
/// Output: 2
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = " 2-1 + 2 "
/// Output: 3
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "(1+(4+5+2)-3)+(6+8)"
/// Output: 23
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 3 * 10<sup>5</sup>`
/// * `s` consists of digits, `'+'`, `'-'`, `'('`, `')'`, and `' '`.
/// * `s` represents a valid expression.
/// * `'+'` is **not** used as a unary operation (i.e., `"+1"` and `"+(2 + 3)"` is invalid).
/// * `'-'` could be used as a unary operation (i.e., `"-1"` and `"-(2 + 3)"` is valid).
/// * There will be no two consecutive operators in the input.
/// * Every number and running calculation will fit in a signed 32-bit integer.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator/
// discuss: https://leetcode.com/problems/basic-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut current_number: i64 = 0;
        let mut sign: i32 = 1;
        let mut stack: Vec<(i32, i32)> = vec![];

        for ch in s.bytes() {
            match ch {
                b'0'..=b'9' => {
                    current_number = current_number * 10 + (ch - b'0') as i64;
                }
                b'+' => {
                    result += (sign as i64 * current_number) as i32;
                    current_number = 0;
                    sign = 1;
                }
                b'-' => {
                    result += (sign as i64 * current_number) as i32;
                    current_number = 0;
                    sign = -1;
                }
                b'(' => {
                    stack.push((result, sign));
                    result = 0;
                    sign = 1;
                }
                b')' => {
                    result += (sign as i64 * current_number) as i32;
                    current_number = 0;

                    if let Some((prev_result, prev_sign)) = stack.pop() {
                        result = prev_result + prev_sign * result;
                    }
                }
                _ => (),
            }
        }
        result + (sign as i64 * current_number) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_224() {
        assert_eq!(Solution::calculate("1 + 1".to_owned()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_owned()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
        assert_eq!(Solution::calculate("-2+ 1".to_owned()), -1);
        assert_eq!(Solution::calculate("-(2 + 3)".to_owned()), -5);
        assert_eq!(Solution::calculate("- (3 - (2 + 1))".to_owned()), 0);
        // assert_eq!(Solution::calculate("-2147483648".to_owned()), -2147483648);
    }
}
