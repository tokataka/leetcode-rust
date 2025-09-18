///
/// # 3541. Find Most Frequent Vowel and Consonant
///
/// You are given a string `s` consisting of lowercase English letters (`'a'` to `'z'`).
///
/// Your task is to:
///
/// * Find the vowel (one of `'a'`, `'e'`, `'i'`, `'o'`, or `'u'`) with the **maximum** frequency.
/// * Find the consonant (all other letters excluding vowels) with the **maximum** frequency.
///
/// Return the sum of the two frequencies.
///
/// **Note**: If multiple vowels or consonants have the same maximum frequency, you may choose any one of them. If there are no vowels or no consonants in the string, consider their frequency as 0.
///
/// The **frequency** of a letter `x` is the number of times it occurs in the string.
///
/// **Example 1:**
///
/// **Input:** s = "successes"
///
/// **Output:** 6
///
/// **Explanation:**
///
/// * The vowels are: `'u'` (frequency 1), `'e'` (frequency 2). The maximum frequency is 2.
/// * The consonants are: `'s'` (frequency 4), `'c'` (frequency 2). The maximum frequency is 4.
/// * The output is `2 + 4 = 6`.
///
/// **Example 2:**
///
/// **Input:** s = "aeiaeia"
///
/// **Output:** 3
///
/// **Explanation:**
///
/// * The vowels are: `'a'` (frequency 3), `'e'` ( frequency 2), `'i'` (frequency 2). The maximum frequency is 3.
/// * There are no consonants in `s`. Hence, maximum consonant frequency = 0.
/// * The output is `3 + 0 = 3`.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 100`
/// * `s` consists of lowercase English letters only.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/
// discuss: https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut freq = [0; 26];

        let mut max_vowel = 0;
        let mut max_consonant = 0;

        for ch in s.bytes() {
            freq[(ch - b'a') as usize] += 1;

            if b"aeiou".contains(&ch) {
                max_vowel = max_vowel.max(freq[(ch - b'a') as usize]);
            } else {
                max_consonant = max_consonant.max(freq[(ch - b'a') as usize]);
            }
        }

        max_vowel + max_consonant
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3541() {
        let s = "successes".to_owned();
        let expected = 6;
        assert_eq!(Solution::max_freq_sum(s), expected);
        let s = "aeiaeia".to_owned();
        let expected = 3;
        assert_eq!(Solution::max_freq_sum(s), expected);
    }
}
