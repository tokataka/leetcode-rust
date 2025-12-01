///
/// # 1247. Minimum Swaps to Make Strings Equal
///
/// You are given two strings `s1` and `s2` of equal length consisting of letters `"x"` and `"y"` **only**. Your task is to make these two strings equal to each other. You can swap any two characters that belong to **different** strings, which means: swap `s1[i]` and `s2[j]`.
///
/// Return the minimum number of swaps required to make `s1` and `s2` equal, or return `-1` if it is impossible to do so.
///
/// **Example 1:**
///
/// ```
/// Input: s1 = "xx", s2 = "yy"
/// Output: 1
/// Explanation: Swap s1[0] and s2[1], s1 = "yx", s2 = "yx".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s1 = "xy", s2 = "yx"
/// Output: 2
/// Explanation: Swap s1[0] and s2[0], s1 = "yy", s2 = "xx".
/// Swap s1[0] and s2[1], s1 = "xy", s2 = "xy".
/// Note that you cannot swap s1[0] and s1[1] to make s1 equal to "yx", cause we can only swap chars in different strings.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s1 = "xx", s2 = "xy"
/// Output: -1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s1.length, s2.length <= 1000`
/// * `s1.length == s2.length`
/// * `s1, s2` only contain `'x'` or `'y'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-swaps-to-make-strings-equal/
// discuss: https://leetcode.com/problems/minimum-swaps-to-make-strings-equal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let (x, y) = s1
            .bytes()
            .zip(s2.bytes())
            .fold((0, 0), |(x, y), (ch1, ch2)| match (ch1, ch2) {
                (b'x', b'x') | (b'y', b'y') => (x, y),
                (b'x', _) => (x + 1, y),
                _ => (x, y + 1),
            });

        if (x + y) % 2 == 1 {
            return -1;
        }

        x / 2 + y / 2 + x % 2 + y % 2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1247() {
        let s1 = "xx".to_owned();
        let s2 = "yy".to_owned();
        let expected = 1;
        assert_eq!(Solution::minimum_swap(s1, s2), expected);
        let s1 = "xy".to_owned();
        let s2 = "yx".to_owned();
        let expected = 2;
        assert_eq!(Solution::minimum_swap(s1, s2), expected);
        let s1 = "xx".to_owned();
        let s2 = "xy".to_owned();
        let expected = -1;
        assert_eq!(Solution::minimum_swap(s1, s2), expected);
    }
}
