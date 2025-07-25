///
/// # 3136. Valid Word
///
/// A word is considered **valid** if:
///
/// * It contains a **minimum** of 3 characters.
/// * It contains only digits (0-9), and English letters (uppercase and lowercase).
/// * It includes **at least** one **vowel**.
/// * It includes **at least** one **consonant**.
///
/// You are given a string `word`.
///
/// Return `true` if `word` is valid, otherwise, return `false`.
///
/// **Notes:**
///
/// * `'a'`, `'e'`, `'i'`, `'o'`, `'u'`, and their uppercases are **vowels**.
/// * A **consonant** is an English letter that is not a vowel.
///
/// **Example 1:**
///
/// **Input:** word = "234Adas"
///
/// **Output:** true
///
/// **Explanation:**
///
/// This word satisfies the conditions.
///
/// **Example 2:**
///
/// **Input:** word = "b3"
///
/// **Output:** false
///
/// **Explanation:**
///
/// The length of this word is fewer than 3, and does not have a vowel.
///
/// **Example 3:**
///
/// **Input:** word = "a3$e"
///
/// **Output:** false
///
/// **Explanation:**
///
/// This word contains a `'$'` character and does not have a consonant.
///
/// **Constraints:**
///
/// * `1 <= word.length <= 20`
/// * `word` consists of English uppercase and lowercase letters, digits, `'@'`, `'#'`, and `'$'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-word/
// discuss: https://leetcode.com/problems/valid-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut has_vowel = false;
        let mut has_consonant = false;

        for ch in word.to_lowercase().bytes() {
            match ch {
                b'a' | b'e' | b'i' | b'o' | b'u' => has_vowel = true,
                b'a'..=b'z' => has_consonant = true,
                b'0'..=b'9' => (),
                _ => return false,
            };
        }

        has_vowel && has_consonant
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3136() {
        let word = "234Adas".to_owned();
        let expected = true;
        assert_eq!(Solution::is_valid(word), expected);
        let word = "b3".to_owned();
        let expected = false;
        assert_eq!(Solution::is_valid(word), expected);
        let word = "a3$e".to_owned();
        let expected = false;
        assert_eq!(Solution::is_valid(word), expected);
    }
}
