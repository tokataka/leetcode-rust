///
/// # 336. Palindrome Pairs
///
/// You are given a **0-indexed** array of **unique** strings `words`.
///
/// A **palindrome pair** is a pair of integers `(i, j)` such that:
///
/// * `0 <= i, j < words.length`,
/// * `i != j`, and
/// * `words[i] + words[j]` (the concatenation of the two strings) is a palindrome.
///
/// Return *an array of all the **palindrome pairs** of* `words`.
///
/// You must write an algorithm with `O(sum of words[i].length)` runtime complexity.
///
/// **Example 1:**
///
/// ```
/// Input: words = ["abcd","dcba","lls","s","sssll"]
/// Output: [[0,1],[1,0],[3,2],[2,4]]
/// Explanation: The palindromes are ["abcddcba","dcbaabcd","slls","llssssll"]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: words = ["bat","tab","cat"]
/// Output: [[0,1],[1,0]]
/// Explanation: The palindromes are ["battab","tabbat"]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: words = ["a",""]
/// Output: [[0,1],[1,0]]
/// Explanation: The palindromes are ["a","a"]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= words.length <= 5000`
/// * `0 <= words[i].length <= 300`
/// * `words[i]` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-pairs/
// discuss: https://leetcode.com/problems/palindrome-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        #[derive(Default)]
        struct TrieNode {
            children: HashMap<u8, TrieNode>,
            word_index: Option<i32>,
            palindrome_indices: Vec<i32>,
        }

        fn is_palindrome(word: &[u8]) -> bool {
            if word.is_empty() {
                return true;
            }

            let mut left = 0;
            let mut right = word.len() - 1;

            while left < right {
                if word[left] != word[right] {
                    return false;
                }

                left += 1;
                right -= 1;
            }

            true
        }

        let mut root = TrieNode::default();

        for (i, word) in words.iter().enumerate() {
            let i = i as i32;
            let word = word.as_bytes();

            let mut curr = &mut root;
            for (j, &c) in word.iter().enumerate().rev() {
                if is_palindrome(&word[..=j]) {
                    curr.palindrome_indices.push(i);
                }
                curr = curr.children.entry(c).or_default();
            }

            curr.palindrome_indices.push(i);
            curr.word_index = Some(i);
        }

        let mut result = Vec::new();

        'outer: for (i, word) in words.iter().enumerate() {
            let i = i as i32;
            let word = word.as_bytes();

            let mut curr = &root;

            for (j, &c) in word.iter().enumerate() {
                if let Some(word_idx) = curr.word_index {
                    if word_idx != i && is_palindrome(&word[j..]) {
                        result.push(vec![i, word_idx]);
                    }
                }

                if let Some(next) = curr.children.get(&c) {
                    curr = next;
                } else {
                    continue 'outer;
                }
            }

            for &word_idx in &curr.palindrome_indices {
                if word_idx != i {
                    result.push(vec![i, word_idx]);
                }
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
    fn test_336() {
        assert_eq!(
            Solution::palindrome_pairs(vec_string!["abcd", "dcba", "lls", "s", "sssll"]),
            nd_vec![[0, 1], [1, 0], [3, 2], [2, 4]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec_string!["bat", "tab", "cat"]),
            nd_vec![[0, 1], [1, 0]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec_string!["a", ""]),
            nd_vec![[0, 1], [1, 0]]
        );
    }
}
