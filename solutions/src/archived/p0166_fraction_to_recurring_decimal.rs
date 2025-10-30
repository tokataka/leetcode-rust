///
/// # 166. Fraction to Recurring Decimal
///
/// Given two integers representing the `numerator` and `denominator` of a fraction, return *the fraction in string format*.
///
/// If the fractional part is repeating, enclose the repeating part in parentheses.
///
/// If multiple answers are possible, return **any of them**.
///
/// It is **guaranteed** that the length of the answer string is less than `10<sup>4</sup>` for all the given inputs.
///
/// **Example 1:**
///
/// ```
/// Input: numerator = 1, denominator = 2
/// Output: "0.5"
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: numerator = 2, denominator = 1
/// Output: "2"
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: numerator = 4, denominator = 333
/// Output: "0.(012)"
///
/// ```
///
/// **Constraints:**
///
/// * `-2<sup>31</sup> <= numerator, denominator <= 2<sup>31</sup> - 1`
/// * `denominator != 0`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/fraction-to-recurring-decimal/
// discuss: https://leetcode.com/problems/fraction-to-recurring-decimal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let sign = match numerator.signum() * denominator.signum() {
            -1 => "-",
            _ => "",
        };

        let numerator = (numerator as i64).abs();
        let denominator = (denominator as i64).abs();

        let decimal = numerator / denominator;
        let mut cur = numerator % denominator;

        if cur == 0 {
            return format!("{sign}{decimal}");
        }

        let mut fractional = String::new();
        let mut occurance = HashMap::new();

        while cur > 0 {
            match occurance.get(&cur) {
                Some(&idx) => {
                    let (fractional, repeat) = fractional.split_at(idx);
                    return format!("{sign}{decimal}.{fractional}({repeat})",);
                }
                None => occurance.insert(cur, fractional.len()),
            };

            cur *= 10;
            fractional.push((b'0' + (cur / denominator) as u8) as char);
            cur %= denominator;
        }

        format!("{sign}{decimal}.{fractional}")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_166() {
        let numerator = 1;
        let denominator = 2;
        let expected = "0.5".to_owned();
        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            expected
        );
        let numerator = 2;
        let denominator = 1;
        let expected = "2".to_owned();
        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            expected
        );
        let numerator = 4;
        let denominator = 333;
        let expected = "0.(012)".to_owned();
        assert_eq!(
            Solution::fraction_to_decimal(numerator, denominator),
            expected
        );
    }
}
