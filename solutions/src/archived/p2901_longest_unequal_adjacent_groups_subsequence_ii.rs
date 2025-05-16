///
/// # 2901. Longest Unequal Adjacent Groups Subsequence II
///
/// You are given a string array `words`, and an array `groups`, both arrays having length `n`.
///
/// The **hamming distance** between two strings of equal length is the number of positions at which the corresponding characters are **different**.
///
/// You need to select the **longest** subsequence from an array of indices `[0, 1, ..., n - 1]`, such that for the subsequence denoted as `[i<sub>0</sub>, i<sub>1</sub>, ..., i<sub>k-1</sub>]` having length `k`, the following holds:
///
/// * For **adjacent** indices in the subsequence, their corresponding groups are **unequal**, i.e., `groups[i<sub>j</sub>] != groups[i<sub>j+1</sub>]`, for each `j` where `0 < j + 1 < k`.
/// * `words[i<sub>j</sub>]` and `words[i<sub>j+1</sub>]` are **equal** in length, and the **hamming distance** between them is `1`, where `0 < j + 1 < k`, for all indices in the subsequence.
///
/// Return *a string array containing the words corresponding to the indices **(in order)** in the selected subsequence*. If there are multiple answers, return *any of them*.
///
/// **Note:** strings in `words` may be **unequal** in length.
///
/// **Example 1:**
///
/// **Input:** words = ["bab","dab","cab"], groups = [1,2,2]
///
/// **Output:** ["bab","cab"]
///
/// **Explanation:** A subsequence that can be selected is `[0,2]`.
///
/// * `groups[0] != groups[2]`
/// * `words[0].length == words[2].length`, and the hamming distance between them is 1.
///
/// So, a valid answer is `[words[0],words[2]] = ["bab","cab"]`.
///
/// Another subsequence that can be selected is `[0,1]`.
///
/// * `groups[0] != groups[1]`
/// * `words[0].length == words[1].length`, and the hamming distance between them is `1`.
///
/// So, another valid answer is `[words[0],words[1]] = ["bab","dab"]`.
///
/// It can be shown that the length of the longest subsequence of indices that satisfies the conditions is `2`.
///
/// **Example 2:**
///
/// **Input:** words = ["a","b","c","d"], groups = [1,2,3,4]
///
/// **Output:** ["a","b","c","d"]
///
/// **Explanation:** We can select the subsequence `[0,1,2,3]`.
///
/// It satisfies both conditions.
///
/// Hence, the answer is `[words[0],words[1],words[2],words[3]] = ["a","b","c","d"]`.
///
/// It has the longest length among all subsequences of indices that satisfy the conditions.
///
/// Hence, it is the only answer.
///
/// **Constraints:**
///
/// * `1 <= n == words.length == groups.length <= 1000`
/// * `1 <= words[i].length <= 10`
/// * `1 <= groups[i] <= n`
/// * `words` consists of **distinct** strings.
/// * `words[i]` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-ii/
// discuss: https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut dp = vec![(0, 0); n];

        for cur in (0..n).rev() {
            let cur_word = &words[cur];
            let cur_group = groups[cur];

            let mut max_count_idx = usize::MAX;
            let mut max_count = 0;

            for next in cur + 1..n {
                let next_word = &words[next];
                let next_group = groups[next];

                if cur_group == next_group
                    || cur_word.len() != next_word.len()
                    || cur_word
                        .bytes()
                        .zip(next_word.bytes())
                        .filter(|(a, b)| a != b)
                        .count()
                        != 1
                {
                    continue;
                }

                if dp[next].1 > max_count {
                    max_count = dp[next].1;
                    max_count_idx = next;
                }
            }

            dp[cur] = (max_count_idx, max_count + 1);
        }

        let mut cur = dp
            .iter()
            .enumerate()
            .max_by_key(|(_, (_, c))| c)
            .map(|(i, _)| i)
            .unwrap();

        let mut result = vec![];

        while let Some(&(next, _)) = dp.get(cur) {
            result.push(words[cur].clone());
            cur = next;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2901() {
        let words = vec_string!["bab", "dab", "cab"];
        let groups = vec![1, 2, 2];
        let expected = vec_string!["bab", "cab"];
        assert_eq!(
            Solution::get_words_in_longest_subsequence(words, groups),
            expected
        );
        let words = vec_string!["a", "b", "c", "d"];
        let groups = vec![1, 2, 3, 4];
        let expected = vec_string!["a", "b", "c", "d"];
        assert_eq!(
            Solution::get_words_in_longest_subsequence(words, groups),
            expected
        );
    }
}
