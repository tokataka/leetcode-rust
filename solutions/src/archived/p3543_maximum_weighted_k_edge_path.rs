///
/// # 3543. Maximum Weighted K-Edge Path
///
/// You are given an integer `n` and a **Directed Acyclic Graph (DAG)** with `n` nodes labeled from 0 to `n - 1`. This is represented by a 2D array `edges`, where `edges[i] = [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]` indicates a directed edge from node `u<sub>i</sub>` to `v<sub>i</sub>` with weight `w<sub>i</sub>`.
///
/// You are also given two integers, `k` and `t`.
///
/// Your task is to determine the **maximum** possible sum of edge weights for any path in the graph such that:
///
/// * The path contains **exactly** `k` edges.
/// * The total sum of edge weights in the path is **strictly** less than `t`.
///
/// Return the **maximum** possible sum of weights for such a path. If no such path exists, return `-1`.
///
/// **Example 1:**
///
/// **Input:** n = 3, edges = [[0,1,1],[1,2,2]], k = 2, t = 4
///
/// **Output:** 3
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/04/09/screenshot-2025-04-10-at-061326.png)
///
/// * The only path with `k = 2` edges is `0 -> 1 -> 2` with weight `1 + 2 = 3 < t`.
/// * Thus, the maximum possible sum of weights less than `t` is 3.
///
/// **Example 2:**
///
/// **Input:** n = 3, edges = [[0,1,2],[0,2,3]], k = 1, t = 3
///
/// **Output:** 2
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/04/09/screenshot-2025-04-10-at-061406.png)
///
/// * There are two paths with `k = 1` edge:
///   * `0 -> 1` with weight `2 < t`.
///   * `0 -> 2` with weight `3 = t`, which is not strictly less than `t`.
///
/// * Thus, the maximum possible sum of weights less than `t` is 2.
///
/// **Example 3:**
///
/// **Input:** n = 3, edges = [[0,1,6],[1,2,8]], k = 1, t = 6
///
/// **Output:** -1
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/04/09/screenshot-2025-04-10-at-061442.png)
///
/// * There are two paths with k = 1 edge:
///   * `0 -> 1` with weight `6 = t`, which is not strictly less than `t`.
///   * `1 -> 2` with weight `8 > t`, which is not strictly less than `t`.
///
/// * Since there is no path with sum of weights strictly less than `t`, the answer is -1.
///
/// **Constraints:**
///
/// * `1 <= n <= 300`
/// * `0 <= edges.length <= 300`
/// * `edges[i] = [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]`
/// * `0 <= u<sub>i</sub>, v<sub>i</sub> < n`
/// * `u<sub>i</sub> != v<sub>i</sub>`
/// * `1 <= w<sub>i</sub> <= 10`
/// * `0 <= k <= 300`
/// * `1 <= t <= 600`
/// * The input graph is **guaranteed** to be a **DAG**.
/// * There are no duplicate edges.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-weighted-k-edge-path/
// discuss: https://leetcode.com/problems/maximum-weighted-k-edge-path/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_weight(n: i32, edges: Vec<Vec<i32>>, k: i32, t: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        if k == 0 {
            return 0;
        }

        let mut graph = vec![vec![]; n];
        let mut in_degree = vec![0; n];

        for edge in edges {
            let (a, b, w) = (edge[0] as usize, edge[1] as usize, edge[2]);

            graph[a].push((b, w));
            in_degree[b] += 1;
        }

        let mut q = (0..n)
            .filter(|&i| in_degree[i] == 0)
            .collect::<VecDeque<_>>();

        let mut path_weights = vec![vec![HashSet::new(); n]; k + 1];

        for i in 0..n {
            path_weights[0][i].insert(0);
        }

        while let Some(cur) = q.pop_front() {
            for &(next, weight) in &graph[cur] {
                for i in 0..k {
                    let (path_weights_cur, path_weights_next) = path_weights.split_at_mut(i + 1);

                    path_weights_next[0][next].extend(
                        path_weights_cur[i][cur]
                            .iter()
                            .map(|&w| w + weight)
                            .filter(|&w| w < t),
                    );
                }

                in_degree[next] -= 1;

                if in_degree[next] == 0 {
                    q.push_back(next);
                }
            }
        }

        *path_weights[k].iter().flatten().max().unwrap_or(&-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3543() {
        let n = 3;
        let edges = nd_vec![[0, 1, 1], [1, 2, 2]];
        let k = 2;
        let t = 4;
        let expected = 3;
        assert_eq!(Solution::max_weight(n, edges, k, t), expected);
        let n = 3;
        let edges = nd_vec![[0, 1, 2], [0, 2, 3]];
        let k = 1;
        let t = 3;
        let expected = 2;
        assert_eq!(Solution::max_weight(n, edges, k, t), expected);
        let n = 3;
        let edges = nd_vec![[0, 1, 6], [1, 2, 8]];
        let k = 1;
        let t = 6;
        let expected = -1;
        assert_eq!(Solution::max_weight(n, edges, k, t), expected);
    }
}
