///
/// # 1625. Lexicographically Smallest String After Applying Operations
///
/// You are given a string `s` of **even length** consisting of digits from `0` to `9`, and two integers `a` and `b`.
///
/// You can apply either of the following two operations any number of times and in any order on `s`:
///
/// * Add `a` to all odd indices of `s` **(0-indexed)**. Digits post `9` are cycled back to `0`. For example, if `s = "3456"` and `a = 5`, `s` becomes `"3951"`.
/// * Rotate `s` to the right by `b` positions. For example, if `s = "3456"` and `b = 1`, `s` becomes `"6345"`.
///
/// Return *the **lexicographically smallest** string you can obtain by applying the above operations any number of times on* `s`.
///
/// A string `a` is lexicographically smaller than a string `b` (of the same length) if in the first position where `a` and `b` differ, string `a` has a letter that appears earlier in the alphabet than the corresponding letter in `b`. For example, `"0158"` is lexicographically smaller than `"0190"` because the first position they differ is at the third letter, and `'5'` comes before `'9'`.
///
/// **Example 1:**
///
/// ```
/// Input: s = "5525", a = 9, b = 2
/// Output: "2050"
/// Explanation: We can apply the following operations:
/// Start:  "5525"
/// Rotate: "2555"
/// Add:    "2454"
/// Add:    "2353"
/// Rotate: "5323"
/// Add:    "5222"
/// Add:    "5121"
/// Rotate: "2151"
/// Add:    "2050"​​​​​
/// There is no way to obtain a string that is lexicographically smaller than "2050".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "74", a = 5, b = 1
/// Output: "24"
/// Explanation: We can apply the following operations:
/// Start:  "74"
/// Rotate: "47"
/// ​​​​​​​Add:    "42"
/// ​​​​​​​Rotate: "24"​​​​​​​​​​​​
/// There is no way to obtain a string that is lexicographically smaller than "24".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "0011", a = 4, b = 2
/// Output: "0011"
/// Explanation: There are no sequence of operations that will give us a lexicographically smaller string than "0011".
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= s.length <= 100`
/// * `s.length` is even.
/// * `s` consists of digits from `0` to `9` only.
/// * `1 <= a <= 9`
/// * `1 <= b <= s.length - 1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/
// discuss: https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let a = a as u8;
        let b = b as usize;

        let mut result = s.clone();

        let s = s.repeat(2).bytes().map(|ch| ch - b'0').collect::<Vec<_>>();

        for start in (0..b * n).step_by(b).map(|s| s % n) {
            let mut s = s[start..start + n].to_vec();

            let mut odd_add = 0;

            for add in 1..10 {
                if (s[1] + add * a) % 10 < (s[1] + odd_add) % 10 {
                    odd_add = add * a;
                }
            }

            for i in (0..n).skip(1).step_by(2) {
                s[i] = (s[i] + odd_add) % 10;
            }

            if b % 2 != 0 {
                let mut even_add = 0;

                for add in 1..10 {
                    if (s[0] + add * a) % 10 < (s[0] + even_add) % 10 {
                        even_add = add * a;
                    }
                }

                for i in (0..n).step_by(2) {
                    s[i] = (s[i] + even_add) % 10;
                }
            }

            result = result.min(s.into_iter().map(|ch| (ch + b'0') as char).collect());
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1625() {
        let s = "5525".to_owned();
        let a = 9;
        let b = 2;
        let expected = "2050".to_owned();
        assert_eq!(Solution::find_lex_smallest_string(s, a, b), expected);
        let s = "74".to_owned();
        let a = 5;
        let b = 1;
        let expected = "24".to_owned();
        assert_eq!(Solution::find_lex_smallest_string(s, a, b), expected);
        let s = "0011".to_owned();
        let a = 4;
        let b = 2;
        let expected = "0011".to_owned();
        assert_eq!(Solution::find_lex_smallest_string(s, a, b), expected);
    }
}
