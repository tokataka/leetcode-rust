///
/// # 3330. Find the Original Typed String I
///
/// Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and **may** press a key for too long, resulting in a character being typed **multiple** times.
///
/// Although Alice tried to focus on her typing, she is aware that she may still have done this **at most** *once*.
///
/// You are given a string `word`, which represents the **final** output displayed on Alice's screen.
///
/// Return the total number of *possible* original strings that Alice *might* have intended to type.
///
/// **Example 1:**
///
/// **Input:** word = "abbcccc"
///
/// **Output:** 5
///
/// **Explanation:**
///
/// The possible strings are: `"abbcccc"`, `"abbccc"`, `"abbcc"`, `"abbc"`, and `"abcccc"`.
///
/// **Example 2:**
///
/// **Input:** word = "abcd"
///
/// **Output:** 1
///
/// **Explanation:**
///
/// The only possible string is `"abcd"`.
///
/// **Example 3:**
///
/// **Input:** word = "aaaa"
///
/// **Output:** 4
///
/// **Constraints:**
///
/// * `1 <= word.length <= 100`
/// * `word` consists only of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-original-typed-string-i/
// discuss: https://leetcode.com/problems/find-the-original-typed-string-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        word.as_bytes().windows(2).filter(|x| x[0] == x[1]).count() as i32 + 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3330() {
        let word = "abbcccc".to_owned();
        let expected = 5;
        assert_eq!(Solution::possible_string_count(word), expected);
        let word = "abcd".to_owned();
        let expected = 1;
        assert_eq!(Solution::possible_string_count(word), expected);
        let word = "aaaa".to_owned();
        let expected = 4;
        assert_eq!(Solution::possible_string_count(word), expected);
    }
}
