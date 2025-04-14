///
/// # 5. Longest Palindromic Substring
///
/// Given a string `s`, return *the longest* *palindromic* *substring* in `s`.
///
/// **Example 1:**
///
/// ```
/// Input: s = "babad"
/// Output: "bab"
/// Explanation: "aba" is also a valid answer.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "cbbd"
/// Output: "bb"
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 1000`
/// * `s` consist of only digits and English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();

        let mut cur_ends = Vec::with_capacity(n);
        let mut next_ends = Vec::with_capacity(n);

        cur_ends.push(n);

        let mut result_range = n - 1..n;

        for i in (0..n - 1).rev() {
            for cur_end in cur_ends.drain(..) {
                if cur_end < n && s_bytes[i] == s_bytes[cur_end] {
                    next_ends.push(cur_end + 1);
                }
            }

            if s_bytes[i] == s_bytes[i + 1] {
                next_ends.push(i + 2);
            }

            next_ends.push(i + 1);

            for &next_end in &next_ends {
                if next_end - i > result_range.len() {
                    result_range = i..next_end;
                }
            }

            std::mem::swap(&mut cur_ends, &mut next_ends);
        }

        s[result_range].to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        let s = "babad".to_owned();
        let expected = "bab".to_owned();
        assert_eq!(Solution::longest_palindrome(s), expected);
        let s = "cbbd".to_owned();
        let expected = "bb".to_owned();
        assert_eq!(Solution::longest_palindrome(s), expected);
    }
}
