///
/// # 3544. Subtree Inversion Sum
///
/// You are given an undirected tree rooted at node `0`, with `n` nodes numbered from 0 to `n - 1`. The tree is represented by a 2D integer array `edges` of length `n - 1`, where `edges[i] = [u<sub>i</sub>, v<sub>i</sub>]` indicates an edge between nodes `u<sub>i</sub>` and `v<sub>i</sub>`.
///
/// You are also given an integer array `nums` of length `n`, where `nums[i]` represents the value at node `i`, and an integer `k`.
///
/// You may perform **inversion operations** on a subset of nodes subject to the following rules:
///
/// * **Subtree Inversion Operation:**
///
///   * When you invert a node, every value in the subtree rooted at that node is multiplied by -1.
///
/// * **Distance Constraint on Inversions:**
///
///   * You may only invert a node if it is "sufficiently far" from any other inverted node.
///
///   * Specifically, if you invert two nodes `a` and `b` such that one is an ancestor of the other (i.e., if `LCA(a, b) = a` or `LCA(a, b) = b`), then the distance (the number of edges on the unique path between them) must be at least `k`.
///
/// Return the **maximum** possible **sum** of the tree's node values after applying **inversion operations**.
///
/// **Example 1:**
///
/// **Input:** edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]], nums = [4,-8,-6,3,7,-2,5], k = 2
///
/// **Output:** 27
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/03/29/tree1-3.jpg)
///
/// * Apply inversion operations at nodes 0, 3, 4 and 6.
/// * The final `nums` array is `[-4, 8, 6, 3, 7, 2, 5]`, and the total sum is 27.
///
/// **Example 2:**
///
/// **Input:** edges = [[0,1],[1,2],[2,3],[3,4]], nums = [-1,3,-2,4,-5], k = 2
///
/// **Output:** 9
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/03/29/tree2-1.jpg)
///
/// * Apply the inversion operation at node 4.
/// * The final `nums` array becomes `[-1, 3, -2, 4, 5]`, and the total sum is 9.
///
/// **Example 3:**
///
/// **Input:** edges = [[0,1],[0,2]], nums = [0,-1,-2], k = 3
///
/// **Output:** 3
///
/// **Explanation:**
///
/// Apply inversion operations at nodes 1 and 2.
///
/// **Constraints:**
///
/// * `2 <= n <= 5 * 10<sup>4</sup>`
/// * `edges.length == n - 1`
/// * `edges[i] = [u<sub>i</sub>, v<sub>i</sub>]`
/// * `0 <= u<sub>i</sub>, v<sub>i</sub> < n`
/// * `nums.length == n`
/// * `-5 * 10<sup>4</sup> <= nums[i] <= 5 * 10<sup>4</sup>`
/// * `1 <= k <= 50`
/// * The input is generated such that `edges` represents a valid tree.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/subtree-inversion-sum/
// discuss: https://leetcode.com/problems/subtree-inversion-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
        fn _subtree_inversion_sum(
            cur: usize,
            prev: usize,
            graph: &Vec<Vec<usize>>,
            nums: &Vec<i32>,
            k: usize,
        ) -> (Vec<i64>, Vec<i64>) {
            let mut maxs = vec![nums[cur] as i64; k];
            let mut mins = vec![nums[cur] as i64; k];

            let mut inverse_max = -nums[cur] as i64;
            let mut inverse_min = -nums[cur] as i64;

            for &child in graph[cur].iter().filter(|&&x| x != prev) {
                let (child_maxs, child_mins) = _subtree_inversion_sum(child, cur, graph, nums, k);

                for i in 0..k - 1 {
                    maxs[i] += child_maxs[i + 1];
                    mins[i] += child_mins[i + 1];
                }

                maxs[k - 1] += child_maxs[k - 1];
                mins[k - 1] += child_mins[k - 1];

                inverse_max -= child_mins[0];
                inverse_min -= child_maxs[0];
            }

            maxs[k - 1] = maxs[k - 1].max(inverse_max);
            mins[k - 1] = mins[k - 1].min(inverse_min);

            (maxs, mins)
        }

        let n = nums.len();
        let k = k as usize;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            graph[a].push(b);
            graph[b].push(a);
        }

        _subtree_inversion_sum(0, 0, &graph, &nums, k)
            .0
            .into_iter()
            .max()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3544() {
        let edges = nd_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]];
        let nums = vec![4, -8, -6, 3, 7, -2, 5];
        let k = 2;
        let expected = 27;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), expected);
        let edges = nd_vec![[0, 1], [1, 2], [2, 3], [3, 4]];
        let nums = vec![-1, 3, -2, 4, -5];
        let k = 2;
        let expected = 9;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), expected);
        let edges = nd_vec![[0, 1], [0, 2]];
        let nums = vec![0, -1, -2];
        let k = 3;
        let expected = 3;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), expected);
    }
}
