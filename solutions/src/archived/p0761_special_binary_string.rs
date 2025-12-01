///
/// # 761. Special Binary String
///
/// **Special binary strings** are binary strings with the following two properties:
///
/// * The number of `0`'s is equal to the number of `1`'s.
/// * Every prefix of the binary string has at least as many `1`'s as `0`'s.
///
/// You are given a **special binary** string `s`.
///
/// A move consists of choosing two consecutive, non-empty, special substrings of `s`, and swapping them. Two strings are consecutive if the last character of the first string is exactly one index before the first character of the second string.
///
/// Return *the lexicographically largest resulting string possible after applying the mentioned operations on the string*.
///
/// **Example 1:**
///
/// ```
/// Input: s = "11011000"
/// Output: "11100100"
/// Explanation: The strings "10" [occuring at s[1]] and "1100" [at s[3]] are swapped.
/// This is the lexicographically largest string possible after some number of swaps.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "10"
/// Output: "10"
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 50`
/// * `s[i]` is either `'0'` or `'1'`.
/// * `s` is a special binary string.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/special-binary-string/
// discuss: https://leetcode.com/problems/special-binary-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut st: Vec<Vec<String>> = vec![vec![]];

        for ch in s.chars() {
            match ch {
                '1' => {
                    st.push(vec![]);
                }
                _ => {
                    let mut last = st.pop().unwrap();
                    last.sort_unstable_by(|a, b| b.cmp(a));

                    st.last_mut().unwrap().push(format!("1{}0", last.concat()));
                }
            }
        }

        st[0].sort_unstable_by(|a, b| b.cmp(a));
        st[0].concat()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_761() {
        let s = "11011000".to_owned();
        let expected = "11100100".to_owned();
        assert_eq!(Solution::make_largest_special(s), expected);
        let s = "10".to_owned();
        let expected = "10".to_owned();
        assert_eq!(Solution::make_largest_special(s), expected);
    }
}
