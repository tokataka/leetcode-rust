use std::collections::HashMap;

///
/// # 3093. Longest Common Suffix Queries
///
/// You are given two arrays of strings `wordsContainer` and `wordsQuery`.
///
/// For each `wordsQuery[i]`, you need to find a string from `wordsContainer` that has the **longest common suffix** with `wordsQuery[i]`. If there are two or more strings in `wordsContainer` that share the longest common suffix, find the string that is the **smallest** in length. If there are two or more such strings that have the **same** smallest length, find the one that occurred **earlier** in `wordsContainer`.
///
/// Return *an array of integers* `ans`*, where* `ans[i]` *is the index of the string in* `wordsContainer` *that has the **longest common suffix** with* `wordsQuery[i]`*.*
///
/// **Example 1:**
///
/// **Input:** wordsContainer = ["abcd","bcd","xbcd"], wordsQuery = ["cd","bcd","xyz"]
///
/// **Output:** [1,1,1]
///
/// **Explanation:**
///
/// Let's look at each `wordsQuery[i]` separately:
///
/// * For `wordsQuery[0] = "cd"`, strings from `wordsContainer` that share the longest common suffix `"cd"` are at indices 0, 1, and 2. Among these, the answer is the string at index 1 because it has the shortest length of 3.
/// * For `wordsQuery[1] = "bcd"`, strings from `wordsContainer` that share the longest common suffix `"bcd"` are at indices 0, 1, and 2. Among these, the answer is the string at index 1 because it has the shortest length of 3.
/// * For `wordsQuery[2] = "xyz"`, there is no string from `wordsContainer` that shares a common suffix. Hence the longest common suffix is `""`, that is shared with strings at index 0, 1, and 2. Among these, the answer is the string at index 1 because it has the shortest length of 3.
///
/// **Example 2:**
///
/// **Input:** wordsContainer = ["abcdefgh","poiuygh","ghghgh"], wordsQuery = ["gh","acbfgh","acbfegh"]
///
/// **Output:** [2,0,2]
///
/// **Explanation:**
///
/// Let's look at each `wordsQuery[i]` separately:
///
/// * For `wordsQuery[0] = "gh"`, strings from `wordsContainer` that share the longest common suffix `"gh"` are at indices 0, 1, and 2. Among these, the answer is the string at index 2 because it has the shortest length of 6.
/// * For `wordsQuery[1] = "acbfgh"`, only the string at index 0 shares the longest common suffix `"fgh"`. Hence it is the answer, even though the string at index 2 is shorter.
/// * For `wordsQuery[2] = "acbfegh"`, strings from `wordsContainer` that share the longest common suffix `"gh"` are at indices 0, 1, and 2. Among these, the answer is the string at index 2 because it has the shortest length of 6.
///
/// **Constraints:**
///
/// * `1 <= wordsContainer.length, wordsQuery.length <= 10<sup>4</sup>`
/// * `1 <= wordsContainer[i].length <= 5 * 10<sup>3</sup>`
/// * `1 <= wordsQuery[i].length <= 5 * 10<sup>3</sup>`
/// * `wordsContainer[i]` consists only of lowercase English letters.
/// * `wordsQuery[i]` consists only of lowercase English letters.
/// * Sum of `wordsContainer[i].length` is at most `5 * 10<sup>5</sup>`.
/// * Sum of `wordsQuery[i].length` is at most `5 * 10<sup>5</sup>`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-suffix-queries/
// discuss: https://leetcode.com/problems/longest-common-suffix-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct Trie {
    smallest_length: usize,
    smallest_length_word_idx: usize,
    children: HashMap<u8, Box<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Self {
            smallest_length: usize::MAX,
            smallest_length_word_idx: usize::MAX,
            children: HashMap::new(),
        }
    }

    fn build(&mut self, word: &[u8], word_idx: usize) {
        if word.len() < self.smallest_length {
            self.smallest_length = word.len();
            self.smallest_length_word_idx = word_idx;
        }

        if let Some((&last, remain)) = word.split_last() {
            self.children
                .entry(last)
                .or_insert(Box::new(Self::new()))
                .build(remain, word_idx);
        }
    }

    fn query(&self, word: &[u8]) -> usize {
        if let Some((last, remain)) = word.split_last() {
            if let Some(child) = self.children.get(last) {
                return child.query(remain);
            }
        }

        self.smallest_length_word_idx
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();

        for (i, word) in words_container.iter().enumerate() {
            trie.build(word.as_bytes(), i);
        }

        words_query
            .into_iter()
            .map(|word| trie.query(word.as_bytes()) as i32)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3093() {
        let words_container = vec_string!["abcd", "bcd", "xbcd"];
        let words_query = vec_string!["cd", "bcd", "xyz"];
        let expected = vec![1, 1, 1];
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            expected
        );
        let words_container = vec_string!["abcdefgh", "poiuygh", "ghghgh"];
        let words_query = vec_string!["gh", "acbfgh", "acbfegh"];
        let expected = vec![2, 0, 2];
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            expected
        );
    }
}
