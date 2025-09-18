///
/// # 2264. Largest 3-Same-Digit Number in String
///
/// You are given a string `num` representing a large integer. An integer is **good** if it meets the following conditions:
///
/// * It is a **substring** of `num` with length `3`.
/// * It consists of only one unique digit.
///
/// Return *the **maximum good** integer as a **string** or an empty string* `""` *if no such integer exists*.
///
/// Note:
///
/// * A **substring** is a contiguous sequence of characters within a string.
/// * There may be **leading zeroes** in `num` or a good integer.
///
/// **Example 1:**
///
/// ```
/// Input: num = "6777133339"
/// Output: "777"
/// Explanation: There are two distinct good integers: "777" and "333".
/// "777" is the largest, so we return "777".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: num = "2300019"
/// Output: "000"
/// Explanation: "000" is the only good integer.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: num = "42352338"
/// Output: ""
/// Explanation: No substring of length 3 consists of only one unique digit. Therefore, there are no good integers.
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= num.length <= 1000`
/// * `num` only consists of digits.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-3-same-digit-number-in-string/
// discuss: https://leetcode.com/problems/largest-3-same-digit-number-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut max_digit = 0;
        let mut cur_digit = 0;
        let mut cur_count = 0;

        for digit in num.bytes() {
            if cur_digit != digit {
                cur_digit = digit;
                cur_count = 0;
            }

            if digit <= max_digit {
                continue;
            }

            cur_count += 1;

            if cur_count >= 3 && cur_digit > max_digit {
                max_digit = cur_digit;
            }
        }

        match max_digit {
            0 => String::new(),
            x => (x - b'0').to_string().repeat(3),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2264() {
        let num = "6 777 133339".to_owned();
        let expected = "777".to_owned();
        assert_eq!(Solution::largest_good_integer(num), expected);
        let num = "23 000 19".to_owned();
        let expected = "000".to_owned();
        assert_eq!(Solution::largest_good_integer(num), expected);
        let num = "42352338".to_owned();
        let expected = "".to_owned();
        assert_eq!(Solution::largest_good_integer(num), expected);
    }
}
