///
/// # 2872. Maximum Number of K-Divisible Components
///
/// There is an undirected tree with `n` nodes labeled from `0` to `n - 1`. You are given the integer `n` and a 2D integer array `edges` of length `n - 1`, where `edges[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an edge between nodes `a<sub>i</sub>` and `b<sub>i</sub>` in the tree.
///
/// You are also given a **0-indexed** integer array `values` of length `n`, where `values[i]` is the **value** associated with the `i<sup>th</sup>` node, and an integer `k`.
///
/// A **valid split** of the tree is obtained by removing any set of edges, possibly empty, from the tree such that the resulting components all have values that are divisible by `k`, where the **value of a connected component** is the sum of the values of its nodes.
///
/// Return *the **maximum number of components** in any valid split*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2023/08/07/example12-cropped2svg.jpg)
///
/// ```
/// Input: n = 5, edges = [[0,2],[1,2],[1,3],[2,4]], values = [1,8,1,4,4], k = 6
/// Output: 2
/// Explanation: We remove the edge connecting node 1 with 2. The resulting split is valid because:
/// - The value of the component containing nodes 1 and 3 is values[1] + values[3] = 12.
/// - The value of the component containing nodes 0, 2, and 4 is values[0] + values[2] + values[4] = 6.
/// It can be shown that no other valid split has more than 2 connected components.
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2023/08/07/example21svg-1.jpg)
///
/// ```
/// Input: n = 7, edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]], values = [3,0,6,1,5,2,1], k = 3
/// Output: 3
/// Explanation: We remove the edge connecting node 0 with 2, and the edge connecting node 0 with 1. The resulting split is valid because:
/// - The value of the component containing node 0 is values[0] = 3.
/// - The value of the component containing nodes 2, 5, and 6 is values[2] + values[5] + values[6] = 9.
/// - The value of the component containing nodes 1, 3, and 4 is values[1] + values[3] + values[4] = 6.
/// It can be shown that no other valid split has more than 3 connected components.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 3 * 10<sup>4</sup>`
/// * `edges.length == n - 1`
/// * `edges[i].length == 2`
/// * `0 <= a<sub>i</sub>, b<sub>i</sub> < n`
/// * `values.length == n`
/// * `0 <= values[i] <= 10<sup>9</sup>`
/// * `1 <= k <= 10<sup>9</sup>`
/// * Sum of `values` is divisible by `k`.
/// * The input is generated such that `edges` represents a valid tree.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-k-divisible-components/
// discuss: https://leetcode.com/problems/maximum-number-of-k-divisible-components/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        fn _max_k_divisible_components(
            cur: usize,
            prev: usize,
            graph: &Vec<Vec<usize>>,
            values: &[i32],
            k: i32,
        ) -> (i32, i32) {
            let mut cur_split = 0;
            let mut cur_sum = values[cur] % k;

            for &next in graph[cur].iter().filter(|&&x| x != prev) {
                let (next_split, next_sum) =
                    _max_k_divisible_components(next, cur, graph, values, k);

                cur_split += next_split;
                cur_sum = (cur_sum + next_sum) % k;
            }

            if cur_sum % k == 0 {
                cur_split += 1;
                cur_sum = 0;
            }

            (cur_split, cur_sum)
        }

        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
        }

        _max_k_divisible_components(0, 0, &graph, &values, k).0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2872() {
        assert_eq!(
            Solution::max_k_divisible_components(
                5,
                nd_vec![[0, 2], [1, 2], [1, 3], [2, 4]],
                vec![1, 8, 1, 4, 4],
                6
            ),
            2
        );
        assert_eq!(
            Solution::max_k_divisible_components(
                7,
                nd_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                vec![3, 0, 6, 1, 5, 2, 1],
                3
            ),
            3
        );
    }
}
