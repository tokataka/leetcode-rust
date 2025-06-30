///
/// # 2311. Longest Binary Subsequence Less Than or Equal to K
///
/// You are given a binary string `s` and a positive integer `k`.
///
/// Return *the length of the **longest** subsequence of* `s` *that makes up a **binary** number less than or equal to* `k`.
///
/// Note:
///
/// * The subsequence can contain **leading zeroes**.
/// * The empty string is considered to be equal to `0`.
/// * A **subsequence** is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
///
/// **Example 1:**
///
/// ```
/// Input: s = "1001010", k = 5
/// Output: 5
/// Explanation: The longest subsequence of s that makes up a binary number less than or equal to 5 is "00010", as this number is equal to 2 in decimal.
/// Note that "00100" and "00101" are also possible, which are equal to 4 and 5 in decimal, respectively.
/// The length of this subsequence is 5, so 5 is returned.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "00101001", k = 1
/// Output: 6
/// Explanation: "000001" is the longest subsequence of s that makes up a binary number less than or equal to 1, as this number is equal to 1 in decimal.
/// The length of this subsequence is 6, so 6 is returned.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 1000`
/// * `s[i]` is either `'0'` or `'1'`.
/// * `1 <= k <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/
// discuss: https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let k_len = k.ilog2() as usize + 1;

        if s.len() < k_len {
            return s.len() as i32;
        }

        let (s_left, s_right) = s.as_bytes().split_at(s.len() - k_len);

        let mut result = s.len() - s_left.iter().filter(|&&x| x == b'1').count();

        let s_right_num = s_right
            .iter()
            .fold(0, |acc, &x| (acc << 1) + (x - b'0') as i32);

        if s_right_num > k {
            result -= 1;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2311() {
        // let s = "1001010".to_owned();
        // let k = 5;
        // let expected = 5;
        // assert_eq!(Solution::longest_subsequence(s, k), expected);
        // let s = "00101001".to_owned();
        // let k = 1;
        // let expected = 6;
        // assert_eq!(Solution::longest_subsequence(s, k), expected);
        let s = "110001111001001000001001010001001011000110100110011101111100111000010010110111100110000010001011100011101010100100010110110101111100110100101010111100000000001000001000010010010101000011110110101111110011001100101000010101000011011010100110110000001110000101010110100111010001011101100001001111010100000100100101001111010100010000011100100111010010011101110010100010111111001111001001001001000101110011100010111100101110011101000101001101101000001110101011101010000001110101111111011011010010111110010011110010011111111001010001010101110001001000101101010110000011100101101101001111110010000101100010110000011011111001100100010101000010100110110110001010001100101110001110011011110011111110111000110011011100010000010100110000111110100110000001011100010001000111110111000111110000110001101010000010101001000001100101110110001100111001101011101110110011001000101101010111010111101011101100011100101".to_owned();
        let k = 41212881;
        let expected = 470;
        assert_eq!(Solution::longest_subsequence(s, k), expected);
    }
}
