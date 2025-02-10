///
/// # 3435. Frequencies of Shortest Supersequences
///
/// You are given an array of strings `words`. Find all **shortest common supersequences (SCS)** of `words` that are not permutations of each other.
///
/// A **shortest common supersequence** is a string of **minimum** length that contains each string in `words` as a subsequence.
///
/// Return a 2D array of integers `freqs` that represent all the SCSs. Each `freqs[i]` is an array of size 26, representing the frequency of each letter in the lowercase English alphabet for a single SCS. You may return the frequency arrays in any order.
///
/// **Example 1:**
///
/// **Input:** words = ["ab","ba"]
///
/// **Output:** [[1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]
///
/// **Explanation:**
///
/// The two SCSs are `"aba"` and `"bab"`. The output is the letter frequencies for each one.
///
/// **Example 2:**
///
/// **Input:** words = ["aa","ac"]
///
/// **Output:** [[2,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]
///
/// **Explanation:**
///
/// The two SCSs are `"aac"` and `"aca"`. Since they are permutations of each other, keep only `"aac"`.
///
/// **Example 3:**
///
/// **Input:** words = ["aa","bb","cc"]
///
/// **Output:** [[2,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]
///
/// **Explanation:**
///
/// `"aabbcc"` and all its permutations are SCSs.
///
/// **Constraints:**
///
/// * `1 <= words.length <= 256`
/// * `words[i].length == 2`
/// * All strings in `words` will altogether be composed of no more than 16 unique lowercase letters.
/// * All strings in `words` are unique.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/frequencies-of-shortest-supersequences/
// discuss: https://leetcode.com/problems/frequencies-of-shortest-supersequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3435() {
        let words = vec_string!["ab", "ba"];
        let expected = nd_vec![
            [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ];
        assert_eq!(Solution::supersequences(words), expected);
        let words = vec_string!["aa", "ac"];
        let expected =
            nd_vec![[2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(Solution::supersequences(words), expected);
        let words = vec_string!["aa", "bb", "cc"];
        let expected =
            nd_vec![[2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(Solution::supersequences(words), expected);
    }
}
