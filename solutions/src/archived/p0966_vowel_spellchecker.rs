///
/// # 966. Vowel Spellchecker
///
/// Given a `wordlist`, we want to implement a spellchecker that converts a query word into a correct word.
///
/// For a given `query` word, the spell checker handles two categories of spelling mistakes:
///
/// * Capitalization: If the query matches a word in the wordlist (**case-insensitive**), then the query word is returned with the same case as the case in the wordlist.
///   * Example: `wordlist = ["yellow"]`, `query = "YellOw"`: `correct = "yellow"`
///   * Example: `wordlist = ["Yellow"]`, `query = "yellow"`: `correct = "Yellow"`
///   * Example: `wordlist = ["yellow"]`, `query = "yellow"`: `correct = "yellow"`
///
/// * Vowel Errors: If after replacing the vowels `('a', 'e', 'i', 'o', 'u')` of the query word with any vowel individually, it matches a word in the wordlist (**case-insensitive**), then the query word is returned with the same case as the match in the wordlist.
///   * Example: `wordlist = ["YellOw"]`, `query = "yollow"`: `correct = "YellOw"`
///   * Example: `wordlist = ["YellOw"]`, `query = "yeellow"`: `correct = ""` (no match)
///   * Example: `wordlist = ["YellOw"]`, `query = "yllw"`: `correct = ""` (no match)
///
/// In addition, the spell checker operates under the following precedence rules:
///
/// * When the query exactly matches a word in the wordlist (**case-sensitive**), you should return the same word back.
/// * When the query matches a word up to capitlization, you should return the first such match in the wordlist.
/// * When the query matches a word up to vowel errors, you should return the first such match in the wordlist.
/// * If the query has no matches in the wordlist, you should return the empty string.
///
/// Given some `queries`, return a list of words `answer`, where `answer[i]` is the correct word for `query = queries[i]`.
///
/// **Example 1:**
///
/// ```
/// Input: wordlist = ["KiTe","kite","hare","Hare"], queries = ["kite","Kite","KiTe","Hare","HARE","Hear","hear","keti","keet","keto"]
/// Output: ["kite","KiTe","KiTe","Hare","hare","","","KiTe","","KiTe"]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: wordlist = ["yellow"], queries = ["YellOw"]
/// Output: ["yellow"]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= wordlist.length, queries.length <= 5000`
/// * `1 <= wordlist[i].length, queries[i].length <= 7`
/// * `wordlist[i]` and `queries[i]` consist only of only English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/vowel-spellchecker/
// discuss: https://leetcode.com/problems/vowel-spellchecker/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut words = HashSet::new();
        let mut capitalization = HashMap::new();
        let mut vowel_errors = HashMap::new();

        for word in wordlist {
            words.insert(word.clone());

            let lowercase = word.to_lowercase();

            capitalization
                .entry(lowercase.clone())
                .or_insert(word.clone());

            let vowel_replaced = lowercase.replace(['a', 'e', 'i', 'o', 'u'], "_");

            vowel_errors.entry(vowel_replaced).or_insert(word);
        }

        queries
            .into_iter()
            .map(|query| {
                if words.contains(&query) {
                    return query;
                }

                let lowercase = query.to_lowercase();

                if let Some(x) = capitalization.get(&lowercase) {
                    return x.clone();
                }

                let vowel_replaced = lowercase.replace(['a', 'e', 'i', 'o', 'u'], "_");

                if let Some(x) = vowel_errors.get(&vowel_replaced) {
                    return x.clone();
                }

                String::new()
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_966() {
        let wordlist = vec_string!["KiTe", "kite", "hare", "Hare"];
        let queries = vec_string![
            "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"
        ];
        let expected =
            vec_string!["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"];
        assert_eq!(Solution::spellchecker(wordlist, queries), expected);
        let wordlist = vec_string!["yellow"];
        let queries = vec_string!["YellOw"];
        let expected = vec_string!["yellow"];
        assert_eq!(Solution::spellchecker(wordlist, queries), expected);
    }
}
