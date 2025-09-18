///
/// # 49. Group Anagrams
///
/// Given an array of strings `strs`, group the anagrams together. You can return the answer in **any order**.
///
/// **Example 1:**
///
/// **Input:** strs = ["eat","tea","tan","ate","nat","bat"]
///
/// **Output:** [["bat"],["nat","tan"],["ate","eat","tea"]]
///
/// **Explanation:**
///
/// * There is no string in strs that can be rearranged to form `"bat"`.
/// * The strings `"nat"` and `"tan"` are anagrams as they can be rearranged to form each other.
/// * The strings `"ate"`, `"eat"`, and `"tea"` are anagrams as they can be rearranged to form each other.
///
/// **Example 2:**
///
/// **Input:** strs = [""]
///
/// **Output:** [[""]]
///
/// **Example 3:**
///
/// **Input:** strs = ["a"]
///
/// **Output:** [["a"]]
///
/// **Constraints:**
///
/// * `1 <= strs.length <= 10<sup>4</sup>`
/// * `0 <= strs[i].length <= 100`
/// * `strs[i]` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/group-anagrams/
// discuss: https://leetcode.com/problems/group-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut freq = [0; 26];

            for ch in s.bytes() {
                freq[(ch - b'a') as usize] += 1;
            }

            map.entry(freq).or_default().push(s);
        }

        map.into_values().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_49() {
        let strs = vec_string!["eat", "tea", "tan", "ate", "nat", "bat"];
        let expected = nd_vec_string![["bat"], ["nat", "tan"], ["ate", "eat", "tea"]];
        assert_eq!(Solution::group_anagrams(strs), expected);
        let strs = vec_string![""];
        let expected = nd_vec_string![[""]];
        assert_eq!(Solution::group_anagrams(strs), expected);
        let strs = vec_string!["a"];
        let expected = nd_vec_string![["a"]];
        assert_eq!(Solution::group_anagrams(strs), expected);
    }
}
