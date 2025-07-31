///
/// # 2213. Longest Substring of One Repeating Character
///
/// You are given a **0-indexed** string `s`. You are also given a **0-indexed** string `queryCharacters` of length `k` and a **0-indexed** array of integer **indices** `queryIndices` of length `k`, both of which are used to describe `k` queries.
///
/// The `i<sup>th</sup>` query updates the character in `s` at index `queryIndices[i]` to the character `queryCharacters[i]`.
///
/// Return *an array* `lengths` *of length* `k` *where* `lengths[i]` *is the **length** of the **longest substring** of* `s` *consisting of **only one repeating** character **after** the* `i<sup>th</sup>` *query* *is performed.*
///
/// **Example 1:**
///
/// ```
/// Input: s = "babacc", queryCharacters = "bcb", queryIndices = [1,3,3]
/// Output: [3,3,4]
/// Explanation:
/// - 1st query updates s = "bbbacc". The longest substring consisting of one repeating character is "bbb" with length 3.
/// - 2nd query updates s = "bbbccc".
///   The longest substring consisting of one repeating character can be "bbb" or "ccc" with length 3.
/// - 3rd query updates s = "bbbbcc". The longest substring consisting of one repeating character is "bbbb" with length 4.
/// Thus, we return [3,3,4].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "abyzz", queryCharacters = "aa", queryIndices = [2,1]
/// Output: [2,3]
/// Explanation:
/// - 1st query updates s = "abazz". The longest substring consisting of one repeating character is "zz" with length 2.
/// - 2nd query updates s = "aaazz". The longest substring consisting of one repeating character is "aaa" with length 3.
/// Thus, we return [2,3].
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s` consists of lowercase English letters.
/// * `k == queryCharacters.length == queryIndices.length`
/// * `1 <= k <= 10<sup>5</sup>`
/// * `queryCharacters` consists of lowercase English letters.
/// * `0 <= queryIndices[i] < s.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-of-one-repeating-character/
// discuss: https://leetcode.com/problems/longest-substring-of-one-repeating-character/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

#[derive(Clone, Copy, Default)]
struct Node {
    total_len: usize,
    max_len: usize,
    left_char: u8,
    left_len: usize,
    right_char: u8,
    right_len: usize,
}

impl Node {
    fn new(ch: u8) -> Self {
        Self {
            total_len: 1,
            max_len: 1,
            left_char: ch,
            left_len: 1,
            right_char: ch,
            right_len: 1,
        }
    }
}

impl std::ops::Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        let mut max_len = self.max_len.max(rhs.max_len);
        let mut left_len = self.left_len;
        let mut right_len = rhs.right_len;

        if self.right_char == rhs.left_char {
            max_len = max_len.max(self.right_len + rhs.left_len);

            if self.total_len == self.left_len {
                left_len += rhs.left_len;
            }

            if rhs.total_len == rhs.right_len {
                right_len += self.right_len;
            }
        }

        Node {
            total_len: self.total_len + rhs.total_len,
            max_len,
            left_char: self.left_char,
            left_len,
            right_char: rhs.right_char,
            right_len,
        }
    }
}

struct SegTree {
    n: usize,
    data: Vec<Node>,
}

impl SegTree {
    fn build(s: &str) -> Self {
        let s = s.as_bytes();
        let n = s.len();

        let mut seg_tree = SegTree {
            n,
            data: vec![Node::default(); n * 4],
        };

        seg_tree._build(1, 0, n - 1, s);

        seg_tree
    }

    fn _build(&mut self, node: usize, left: usize, right: usize, s: &[u8]) {
        if left == right {
            self.data[node] = Node::new(s[left]);
            return;
        }

        let mid = (left + right) / 2;

        self._build(2 * node, left, mid, s);
        self._build(2 * node + 1, mid + 1, right, s);

        self.data[node] = self.data[2 * node] + self.data[2 * node + 1];
    }

    fn update(&mut self, node: usize, left: usize, right: usize, idx: usize, ch: u8) {
        if left == right {
            self.data[node].left_char = ch;
            self.data[node].right_char = ch;
            return;
        }

        let mid = (left + right) / 2;

        if left <= idx && idx <= mid {
            self.update(2 * node, left, mid, idx, ch);
        } else {
            self.update(2 * node + 1, mid + 1, right, idx, ch);
        }

        self.data[node] = self.data[2 * node] + self.data[2 * node + 1];
    }
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let mut seg_tree = SegTree::build(&s);

        let mut result = vec![];

        for (idx, ch) in query_indices
            .into_iter()
            .map(|x| x as usize)
            .zip(query_characters.bytes())
        {
            seg_tree.update(1, 0, s.len() - 1, idx, ch);
            result.push(seg_tree.data[1].max_len as i32);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2213() {
        let s = "babacc".to_owned();
        let query_characters = "bcb".to_owned();
        let query_indices = vec![1, 3, 3];
        let expected = vec![3, 3, 4];
        assert_eq!(
            Solution::longest_repeating(s, query_characters, query_indices),
            expected
        );
        let s = "abyzz".to_owned();
        let query_characters = "aa".to_owned();
        let query_indices = vec![2, 1];
        let expected = vec![2, 3];
        assert_eq!(
            Solution::longest_repeating(s, query_characters, query_indices),
            expected
        );
    }
}
