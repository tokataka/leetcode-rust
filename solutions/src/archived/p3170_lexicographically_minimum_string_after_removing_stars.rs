use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 3170. Lexicographically Minimum String After Removing Stars
///
/// You are given a string `s`. It may contain any number of `'*'` characters. Your task is to remove all `'*'` characters.
///
/// While there is a `'*'`, do the following operation:
///
/// * Delete the leftmost `'*'` and the **smallest** non-`'*'` character to its *left*. If there are several smallest characters, you can delete any of them.
///
/// Return the lexicographically smallest resulting string after removing all `'*'` characters.
///
/// **Example 1:**
///
/// **Input:** s = "aaba\*"
///
/// **Output:** "aab"
///
/// **Explanation:**
///
/// We should delete one of the `'a'` characters with `'*'`. If we choose `s[3]`, `s` becomes the lexicographically smallest.
///
/// **Example 2:**
///
/// **Input:** s = "abc"
///
/// **Output:** "abc"
///
/// **Explanation:**
///
/// There is no `'*'` in the string.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s` consists only of lowercase English letters and `'*'`.
/// * The input is generated such that it is possible to delete all `'*'` characters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/lexicographically-minimum-string-after-removing-stars/
// discuss: https://leetcode.com/problems/lexicographically-minimum-string-after-removing-stars/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut removed = vec![false; s.len()];
        let mut pq: BinaryHeap<(Reverse<u8>, usize)> = BinaryHeap::new();

        for (i, ch) in s.bytes().enumerate() {
            match ch {
                b'*' => {
                    removed[i] = true;
                    removed[pq.pop().unwrap().1] = true;
                }
                ch => pq.push((Reverse(ch), i)),
            }
        }

        s.chars()
            .enumerate()
            .filter(|&(i, _)| !removed[i])
            .map(|(_, ch)| ch)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3170() {
        let s = "aaba*".to_owned();
        let expected = "aab".to_owned();
        assert_eq!(Solution::clear_stars(s), expected);
        let s = "abc".to_owned();
        let expected = "abc".to_owned();
        assert_eq!(Solution::clear_stars(s), expected);
    }
}
