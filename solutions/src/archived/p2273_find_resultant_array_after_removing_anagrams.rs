///
/// # 2273. Find Resultant Array After Removing Anagrams
///
/// You are given a **0-indexed** string array `words`, where `words[i]` consists of lowercase English letters.
///
/// In one operation, select any index `i` such that `0 < i < words.length` and `words[i - 1]` and `words[i]` are **anagrams**, and **delete** `words[i]` from `words`. Keep performing this operation as long as you can select an index that satisfies the conditions.
///
/// Return `words` *after performing all operations*. It can be shown that selecting the indices for each operation in **any** arbitrary order will lead to the same result.
///
/// An **Anagram** is a word or phrase formed by rearranging the letters of a different word or phrase using all the original letters exactly once. For example, `"dacb"` is an anagram of `"abdc"`.
///
/// **Example 1:**
///
/// ```
/// Input: words = ["abba","baba","bbaa","cd","cd"]
/// Output: ["abba","cd"]
/// Explanation:
/// One of the ways we can obtain the resultant array is by using the following operations:
/// - Since words[2] = "bbaa" and words[1] = "baba" are anagrams, we choose index 2 and delete words[2].
///   Now words = ["abba","baba","cd","cd"].
/// - Since words[1] = "baba" and words[0] = "abba" are anagrams, we choose index 1 and delete words[1].
///   Now words = ["abba","cd","cd"].
/// - Since words[2] = "cd" and words[1] = "cd" are anagrams, we choose index 2 and delete words[2].
///   Now words = ["abba","cd"].
/// We can no longer perform any operations, so ["abba","cd"] is the final answer.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: words = ["a","b","c","d","e"]
/// Output: ["a","b","c","d","e"]
/// Explanation:
/// No two adjacent strings in words are anagrams of each other, so no operations are performed.
/// ```
///
/// **Constraints:**
///
/// * `1 <= words.length <= 100`
/// * `1 <= words[i].length <= 10`
/// * `words[i]` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/
// discuss: https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut prev = [0; 26];

        for word in words {
            let mut cur = [0; 26];

            for ch in word.bytes() {
                cur[(ch - b'a') as usize] += 1;
            }

            if prev != cur {
                result.push(word);
                prev = cur;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2273() {
        let words = vec_string!["abba", "baba", "bbaa", "cd", "cd"];
        let expected = vec_string!["abba", "cd"];
        assert_eq!(Solution::remove_anagrams(words), expected);
        let words = vec_string!["a", "b", "c", "d", "e"];
        let expected = vec_string!["a", "b", "c", "d", "e"];
        assert_eq!(Solution::remove_anagrams(words), expected);
    }
}
