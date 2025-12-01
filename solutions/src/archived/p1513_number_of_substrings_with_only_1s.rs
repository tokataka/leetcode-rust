///
/// # 1513. Number of Substrings With Only 1s
///
/// Given a binary string `s`, return *the number of substrings with all characters* `1`*'s*. Since the answer may be too large, return it modulo `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ```
/// Input: s = "0110111"
/// Output: 9
/// Explanation: There are 9 substring in total with only 1's characters.
/// "1" -> 5 times.
/// "11" -> 3 times.
/// "111" -> 1 time.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "101"
/// Output: 2
/// Explanation: Substring "1" is shown 2 times in s.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "111111"
/// Output: 21
/// Explanation: Each substring contains only 1's characters.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s[i]` is either `'0'` or `'1'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-substrings-with-only-1s/
// discuss: https://leetcode.com/problems/number-of-substrings-with-only-1s/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let mut result = 0;
        let mut prev = 0;

        for ch in s.bytes() {
            match ch {
                b'1' => {
                    prev += 1;
                    result = (result + prev) % MOD;
                }
                _ => prev = 0,
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
    fn test_1513() {
        let s = "0110111".to_owned();
        let expected = 9;
        assert_eq!(Solution::num_sub(s), expected);
        let s = "101".to_owned();
        let expected = 2;
        assert_eq!(Solution::num_sub(s), expected);
        let s = "111111".to_owned();
        let expected = 21;
        assert_eq!(Solution::num_sub(s), expected);
    }
}
