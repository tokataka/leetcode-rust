///
/// # 1717. Maximum Score From Removing Substrings
///
/// You are given a string `s` and two integers `x` and `y`. You can perform two types of operations any number of times.
///
/// * Remove substring `"ab"` and gain `x` points.
///   * For example, when removing `"ab"` from `"cabxbae"` it becomes `"cxbae"`.
///
/// * Remove substring `"ba"` and gain `y` points.
///   * For example, when removing `"ba"` from `"cabxbae"` it becomes `"cabxe"`.
///
/// Return *the maximum points you can gain after applying the above operations on* `s`.
///
/// **Example 1:**
///
/// ```
/// Input: s = "cdbcbbaaabab", x = 4, y = 5
/// Output: 19
/// Explanation:
/// - Remove the "ba" underlined in "cdbcbbaaabab". Now, s = "cdbcbbaaab" and 5 points are added to the score.
/// - Remove the "ab" underlined in "cdbcbbaaab". Now, s = "cdbcbbaa" and 4 points are added to the score.
/// - Remove the "ba" underlined in "cdbcbbaa". Now, s = "cdbcba" and 5 points are added to the score.
/// - Remove the "ba" underlined in "cdbcba". Now, s = "cdbc" and 5 points are added to the score.
/// Total score = 5 + 4 + 5 + 5 = 19.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "aabbaaxybbaabb", x = 5, y = 4
/// Output: 20
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `1 <= x, y <= 10<sup>4</sup>`
/// * `s` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-score-from-removing-substrings/
// discuss: https://leetcode.com/problems/maximum-score-from-removing-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut s = s.into_bytes();
        let n = s.len();

        let (a, b, ab_score, ba_score) = match x > y {
            true => (b'a', b'b', x, y),
            false => (b'b', b'a', y, x),
        };

        let mut result = 0;

        let mut st_len = 0;

        for idx in (0..n).rev() {
            let ch = s[idx];

            if ch == a && st_len > 0 && s[n - st_len] == b {
                st_len -= 1;
                result += ab_score;
            } else {
                st_len += 1;
                s[n - st_len] = ch;
            }
        }

        let idx_start = n - st_len;
        let mut st_len = 0;

        for idx in idx_start..n {
            let ch = s[idx];

            if ch == a && st_len > 0 && s[st_len - 1] == b {
                st_len -= 1;
                result += ba_score;
            } else {
                st_len += 1;
                s[st_len - 1] = ch;
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
    fn test_1717() {
        let s = "cdbcbbaaabab".to_owned();
        let x = 4;
        let y = 5;
        let expected = 19;
        assert_eq!(Solution::maximum_gain(s, x, y), expected);
        let s = "aabbaaxybbaabb".to_owned();
        let x = 5;
        let y = 4;
        let expected = 20;
        assert_eq!(Solution::maximum_gain(s, x, y), expected);
    }
}
