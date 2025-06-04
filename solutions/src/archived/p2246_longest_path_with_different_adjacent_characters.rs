///
/// # 2246. Longest Path With Different Adjacent Characters
///
/// You are given a **tree** (i.e. a connected, undirected graph that has no cycles) **rooted** at node `0` consisting of `n` nodes numbered from `0` to `n - 1`. The tree is represented by a **0-indexed** array `parent` of size `n`, where `parent[i]` is the parent of node `i`. Since node `0` is the root, `parent[0] == -1`.
///
/// You are also given a string `s` of length `n`, where `s[i]` is the character assigned to node `i`.
///
/// Return *the length of the **longest path** in the tree such that no pair of **adjacent** nodes on the path have the same character assigned to them.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/25/testingdrawio.png)
///
/// ```
/// Input: parent = [-1,0,0,1,1,2], s = "abacbe"
/// Output: 3
/// Explanation: The longest path where each two adjacent nodes have different characters in the tree is the path: 0 -> 1 -> 3. The length of this path is 3, so 3 is returned.
/// It can be proven that there is no longer path that satisfies the conditions.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/25/graph2drawio.png)
///
/// ```
/// Input: parent = [-1,0,0,0], s = "aabc"
/// Output: 3
/// Explanation: The longest path where each two adjacent nodes have different characters is the path: 2 -> 0 -> 3. The length of this path is 3, so 3 is returned.
///
/// ```
///
/// **Constraints:**
///
/// * `n == parent.length == s.length`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `0 <= parent[i] <= n - 1` for all `i >= 1`
/// * `parent[0] == -1`
/// * `parent` represents a valid tree.
/// * `s` consists of only lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-path-with-different-adjacent-characters/
// discuss: https://leetcode.com/problems/longest-path-with-different-adjacent-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        fn _longest_path(cur: usize, children: &Vec<Vec<usize>>, s: &[u8]) -> (i32, i32) {
            let mut result = 1;
            let mut children_length = vec![];

            let cur_ch = s[cur];

            for &child in &children[cur] {
                let (child_max, child_length) = _longest_path(child, children, s);

                result = result.max(child_max);

                if cur_ch != s[child] {
                    children_length.push(child_length);
                }
            }

            children_length.sort_unstable_by(|a, b| b.cmp(a));

            result = result.max(1 + children_length.iter().take(2).sum::<i32>());
            let cur_length = 1 + children_length.first().unwrap_or(&0);

            (result, cur_length)
        }

        let n = parent.len();

        let mut children = vec![vec![]; n];

        for (c, &p) in parent.iter().enumerate() {
            if p >= 0 {
                children[p as usize].push(c);
            }
        }

        _longest_path(0, &children, s.as_bytes()).0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2246() {
        let parent = vec![-1, 0, 0, 1, 1, 2];
        let s = "abacbe".to_owned();
        let expected = 3;
        assert_eq!(Solution::longest_path(parent, s), expected);
        let parent = vec![-1, 0, 0, 0];
        let s = "aabc".to_owned();
        let expected = 3;
        assert_eq!(Solution::longest_path(parent, s), expected);
    }
}
