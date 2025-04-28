///
/// # 2193. Minimum Number of Moves to Make Palindrome
///
/// You are given a string `s` consisting only of lowercase English letters.
///
/// In one **move**, you can select any two **adjacent** characters of `s` and swap them.
///
/// Return *the **minimum number of moves** needed to make* `s` *a palindrome*.
///
/// **Note** that the input will be generated such that `s` can always be converted to a palindrome.
///
/// **Example 1:**
///
/// ```
/// Input: s = "aabb"
/// Output: 2
/// Explanation:
/// We can obtain two palindromes from s, "abba" and "baab".
/// - We can obtain "abba" from s in 2 moves: "aabb" -> "abab" -> "abba".
/// - We can obtain "baab" from s in 2 moves: "aabb" -> "abab" -> "baab".
/// Thus, the minimum number of moves needed to make s a palindrome is 2.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "letelt"
/// Output: 2
/// Explanation:
/// One of the palindromes we can obtain from s in 2 moves is "lettel".
/// One of the ways we can obtain it is "letelt" -> "letetl" -> "lettel".
/// Other palindromes such as "tleelt" can also be obtained in 2 moves.
/// It can be shown that it is not possible to obtain a palindrome in less than 2 moves.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 2000`
/// * `s` consists only of lowercase English letters.
/// * `s` can be converted to a palindrome using a finite number of moves.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-moves-to-make-palindrome/
// discuss: https://leetcode.com/problems/minimum-number-of-moves-to-make-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_moves_to_make_palindrome(mut s: String) -> i32 {
        let mut result = 0;

        while s.len() > 1 {
            let n = s.len();

            let mut ch_index = [(usize::MAX, 0); 26];
            let mut min_ch = usize::MAX;
            let mut min_distance = usize::MAX;

            for (i, ch) in s.bytes().enumerate() {
                let ch = (ch - b'a') as usize;

                let prev = ch_index[ch];
                let cur = (prev.0.min(i), prev.1.max(i));
                ch_index[ch] = cur;

                if n - 1 + cur.0 - cur.1 < min_distance {
                    min_ch = ch;
                    min_distance = n - 1 + cur.0 - cur.1;
                }
            }

            result += min_distance;
            s.remove(ch_index[min_ch].0);
            s.remove(ch_index[min_ch].1 - 1);
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2193() {
        let s = "aabb".to_owned();
        let expected = 2;
        assert_eq!(Solution::min_moves_to_make_palindrome(s), expected);
        let s = "letelt".to_owned();
        let expected = 2;
        assert_eq!(Solution::min_moves_to_make_palindrome(s), expected);
    }
}
