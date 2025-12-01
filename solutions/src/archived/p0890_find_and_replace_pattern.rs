///
/// # 890. Find and Replace Pattern
///
/// Given a list of strings `words` and a string `pattern`, return *a list of* `words[i]` *that match* `pattern`. You may return the answer in **any order**.
///
/// A word matches the pattern if there exists a permutation of letters `p` so that after replacing every letter `x` in the pattern with `p(x)`, we get the desired word.
///
/// Recall that a permutation of letters is a bijection from letters to letters: every letter maps to another letter, and no two letters map to the same letter.
///
/// **Example 1:**
///
/// ```
/// Input: words = ["abc","deq","mee","aqq","dkd","ccc"], pattern = "abb"
/// Output: ["mee","aqq"]
/// Explanation: "mee" matches the pattern because there is a permutation {a -> m, b -> e, ...}.
/// "ccc" does not match the pattern because {a -> c, b -> c, ...} is not a permutation, since a and b map to the same letter.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: words = ["a","b","c"], pattern = "a"
/// Output: ["a","b","c"]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= pattern.length <= 20`
/// * `1 <= words.length <= 50`
/// * `words[i].length == pattern.length`
/// * `pattern` and `words[i]` are lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-and-replace-pattern/
// discuss: https://leetcode.com/problems/find-and-replace-pattern/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let perm_letters = |word: &str| -> Vec<(usize, usize)> {
            let mut result = vec![];
            let mut map = [usize::MAX; 26];
            let mut new_idx = 0;

            for chunk in word.as_bytes().chunk_by(|a, b| a == b) {
                let ch = (chunk[0] - b'a') as usize;

                if map[ch] == usize::MAX {
                    map[ch] = new_idx;
                    new_idx += 1;
                }

                result.push((map[ch], chunk.len()));
            }

            result
        };

        let pattern = perm_letters(&pattern);

        words
            .into_iter()
            .filter(|word| perm_letters(word) == pattern)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_890() {
        assert_eq!(
            Solution::find_and_replace_pattern(
                vec_string!["abc", "deq", "mee", "aqq", "dkd", "ccc"],
                "abb".to_owned()
            ),
            vec_string!["mee", "aqq"]
        );
        assert_eq!(
            Solution::find_and_replace_pattern(vec_string!["a", "b", "c"], "a".to_owned()),
            vec_string!["a", "b", "c"]
        );
    }
}
