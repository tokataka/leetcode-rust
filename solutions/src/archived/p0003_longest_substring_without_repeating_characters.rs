///
/// # 3. Longest Substring Without Repeating Characters
///
/// Given a string `s`, find the length of the **longest** **substring** without duplicate characters.
///
/// **Example 1:**
///
/// ```
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= s.length <= 5 * 10<sup>4</sup>`
/// * `s` consists of English letters, digits, symbols and spaces.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut exist = [false; 128];
        let mut left = s.bytes().map(|ch| ch as usize);
        let mut cur = 0;
        let mut result = 0;

        for ch_right in s.bytes().map(|ch| ch as usize) {
            if exist[ch_right] {
                for ch_left in left.by_ref() {
                    exist[ch_left] = false;
                    cur -= 1;

                    if ch_left == ch_right {
                        break;
                    }
                }
            }

            exist[ch_right] = true;
            cur += 1;
            result = result.max(cur);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        let s = "abcabcbb".to_owned();
        let expected = 3;
        assert_eq!(Solution::length_of_longest_substring(s), expected);
        let s = "bbbbb".to_owned();
        let expected = 1;
        assert_eq!(Solution::length_of_longest_substring(s), expected);
        let s = "pwwkew".to_owned();
        let expected = 3;
        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }
}
