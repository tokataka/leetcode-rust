///
/// # 3442. Maximum Difference Between Even and Odd Frequency I
///
/// You are given a string `s` consisting of lowercase English letters. Your task is to find the **maximum** difference between the frequency of **two** characters in the string such that:
///
/// * One of the characters has an **even frequency** in the string.
/// * The other character has an **odd frequency** in the string.
///
/// Return the **maximum** difference, calculated as the frequency of the character with an **odd** frequency **minus** the frequency of the character with an **even** frequency.
///
/// **Example 1:**
///
/// **Input:** s = "aaaaabbc"
///
/// **Output:** 3
///
/// **Explanation:**
///
/// * The character `'a'` has an **odd frequency** of `5`, and `'b'` has an **even frequency** of `2`.
/// * The maximum difference is `5 - 2 = 3`.
///
/// **Example 2:**
///
/// **Input:** s = "abcabcab"
///
/// **Output:** 1
///
/// **Explanation:**
///
/// * The character `'a'` has an **odd frequency** of `3`, and `'c'` has an **even frequency** of 2.
/// * The maximum difference is `3 - 2 = 1`.
///
/// **Constraints:**
///
/// * `3 <= s.length <= 100`
/// * `s` consists only of lowercase English letters.
/// * `s` contains at least one character with an odd frequency and one with an even frequency.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/
// discuss: https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq = [0_i32; 26];

        for ch in s.chars() {
            freq[ch as usize - 'a' as usize] += 1;
        }

        let odd = freq.iter().filter(|&&x| x & 1 == 1).max().unwrap();
        let even = freq.iter().filter(|&&x| x != 0 && x & 1 == 0).min().unwrap();

        odd - even
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3442() {
        let s = "aaaaabbc".to_owned();
        let expected = 3;
        assert_eq!(Solution::max_difference(s), expected);
        let s = "abcabcab".to_owned();
        let expected = 1;
        assert_eq!(Solution::max_difference(s), expected);
    }
}
