///
/// # 854. K-Similar Strings
///
/// Strings `s1` and `s2` are `k`**-similar** (for some non-negative integer `k`) if we can swap the positions of two letters in `s1` exactly `k` times so that the resulting string equals `s2`.
///
/// Given two anagrams `s1` and `s2`, return the smallest `k` for which `s1` and `s2` are `k`**-similar**.
///
/// **Example 1:**
///
/// ```
/// Input: s1 = "ab", s2 = "ba"
/// Output: 1
/// Explanation: The two string are 1-similar because we can use one swap to change s1 to s2: "ab" --> "ba".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s1 = "abc", s2 = "bca"
/// Output: 2
/// Explanation: The two strings are 2-similar because we can use two swaps to change s1 to s2: "abc" --> "bac" --> "bca".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s1.length <= 20`
/// * `s2.length == s1.length`
/// * `s1` and `s2` contain only lowercase letters from the set `{'a', 'b', 'c', 'd', 'e', 'f'}`.
/// * `s2` is an anagram of `s1`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/k-similar-strings/
// discuss: https://leetcode.com/problems/k-similar-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        let s1 = s1.bytes().collect::<Vec<_>>();
        let s2 = s2.bytes().collect::<Vec<_>>();

        let mut pq = BinaryHeap::from([(0, Reverse(0), s1)]);
        let mut visited: HashSet<Vec<u8>> = HashSet::new();

        while let Some((score, Reverse(swaps), cur)) = pq.pop() {
            if cur == s2 {
                return swaps;
            }

            for i in 0..s2.len() - 1 {
                if cur[i] == s2[i] {
                    continue;
                }

                for j in i + 1..s2.len() {
                    if cur[j] == s2[j] || cur[i] != s2[j] && cur[j] != s2[i] {
                        continue;
                    }

                    let mut next = cur.clone();
                    next.swap(i, j);

                    if visited.contains(&next) {
                        continue;
                    }

                    visited.insert(next.clone());

                    let next_score = score
                        + match next[i] == s2[i] && next[j] == s2[j] {
                            true => 1,
                            false => 0,
                        };

                    pq.push((next_score, Reverse(swaps + 1), next));
                }
            }
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_854() {
        let s1 = "ab".to_owned();
        let s2 = "ba".to_owned();
        let expected = 1;
        assert_eq!(Solution::k_similarity(s1, s2), expected);
        let s1 = "abc".to_owned();
        let s2 = "bca".to_owned();
        let expected = 2;
        assert_eq!(Solution::k_similarity(s1, s2), expected);
    }
}
