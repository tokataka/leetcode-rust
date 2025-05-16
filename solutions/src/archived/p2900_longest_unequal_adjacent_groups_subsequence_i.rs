///
/// # 2900. Longest Unequal Adjacent Groups Subsequence I
///
/// You are given a string array `words` and a **binary** array `groups` both of length `n`, where `words[i]` is associated with `groups[i]`.
///
/// Your task is to select the **longest alternating** subsequence from `words`. A subsequence of `words` is alternating if for any two consecutive strings in the sequence, their corresponding elements in the binary array `groups` differ. Essentially, you are to choose strings such that adjacent elements have non-matching corresponding bits in the `groups` array.
///
/// Formally, you need to find the longest subsequence of an array of indices `[0, 1, ..., n - 1]` denoted as `[i<sub>0</sub>, i<sub>1</sub>, ..., i<sub>k-1</sub>]`, such that `groups[i<sub>j</sub>] != groups[i<sub>j+1</sub>]` for each `0 <= j < k - 1` and then find the words corresponding to these indices.
///
/// Return *the selected subsequence. If there are multiple answers, return **any** of them.*
///
/// **Note:** The elements in `words` are distinct.
///
/// **Example 1:**
///
/// **Input:** words = ["e","a","b"], groups = [0,0,1]
///
/// **Output:** ["e","b"]
///
/// **Explanation:** A subsequence that can be selected is `["e","b"]` because `groups[0] != groups[2]`. Another subsequence that can be selected is `["a","b"]` because `groups[1] != groups[2]`. It can be demonstrated that the length of the longest subsequence of indices that satisfies the condition is `2`.
///
/// **Example 2:**
///
/// **Input:** words = ["a","b","c","d"], groups = [1,0,1,1]
///
/// **Output:** ["a","b","c"]
///
/// **Explanation:** A subsequence that can be selected is `["a","b","c"]` because `groups[0] != groups[1]` and `groups[1] != groups[2]`. Another subsequence that can be selected is `["a","b","d"]` because `groups[0] != groups[1]` and `groups[1] != groups[3]`. It can be shown that the length of the longest subsequence of indices that satisfies the condition is `3`.
///
/// **Constraints:**
///
/// * `1 <= n == words.length == groups.length <= 100`
/// * `1 <= words[i].length <= 10`
/// * `groups[i]` is either `0` or `1.`
/// * `words` consists of **distinct** strings.
/// * `words[i]` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i/
// discuss: https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut result = vec![];
        let mut cur_group = -1;

        for (word, group) in words.into_iter().zip(groups) {
            if group != cur_group {
                result.push(word);
                cur_group = group;
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
    fn test_2900() {
        let words = vec_string!["e", "a", "b"];
        let groups = vec![0, 0, 1];
        let expected = vec_string!["e", "b"];
        assert_eq!(Solution::get_longest_subsequence(words, groups), expected);
        let words = vec_string!["a", "b", "c", "d"];
        let groups = vec![1, 0, 1, 1];
        let expected = vec_string!["a", "b", "c"];
        assert_eq!(Solution::get_longest_subsequence(words, groups), expected);
    }
}
