use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 2203. Minimum Weighted Subgraph With the Required Paths
///
/// You are given an integer `n` denoting the number of nodes of a **weighted directed** graph. The nodes are numbered from `0` to `n - 1`.
///
/// You are also given a 2D integer array `edges` where `edges[i] = [from<sub>i</sub>, to<sub>i</sub>, weight<sub>i</sub>]` denotes that there exists a **directed** edge from `from<sub>i</sub>` to `to<sub>i</sub>` with weight `weight<sub>i</sub>`.
///
/// Lastly, you are given three **distinct** integers `src1`, `src2`, and `dest` denoting three distinct nodes of the graph.
///
/// Return *the **minimum weight** of a subgraph of the graph such that it is **possible** to reach* `dest` *from both* `src1` *and* `src2` *via a set of edges of this subgraph*. In case such a subgraph does not exist, return `-1`.
///
/// A **subgraph** is a graph whose vertices and edges are subsets of the original graph. The **weight** of a subgraph is the sum of weights of its constituent edges.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/02/17/example1drawio.png)
///
/// ```
/// Input: n = 6, edges = [[0,2,2],[0,5,6],[1,0,3],[1,4,5],[2,1,1],[2,3,3],[2,3,4],[3,4,2],[4,5,1]], src1 = 0, src2 = 1, dest = 5
/// Output: 9
/// Explanation:
/// The above figure represents the input graph.
/// The blue edges represent one of the subgraphs that yield the optimal answer.
/// Note that the subgraph [[1,0,3],[0,5,6]] also yields the optimal answer. It is not possible to get a subgraph with less weight satisfying all the constraints.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/02/17/example2-1drawio.png)
///
/// ```
/// Input: n = 3, edges = [[0,1,1],[2,1,1]], src1 = 0, src2 = 1, dest = 2
/// Output: -1
/// Explanation:
/// The above figure represents the input graph.
/// It can be seen that there does not exist any path from node 1 to node 2, hence there are no subgraphs satisfying all the constraints.
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= n <= 10<sup>5</sup>`
/// * `0 <= edges.length <= 10<sup>5</sup>`
/// * `edges[i].length == 3`
/// * `0 <= from<sub>i</sub>, to<sub>i</sub>, src1, src2, dest <= n - 1`
/// * `from<sub>i</sub> != to<sub>i</sub>`
/// * `src1`, `src2`, and `dest` are pairwise distinct.
/// * `1 <= weight[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-weighted-subgraph-with-the-required-paths/
// discuss: https://leetcode.com/problems/minimum-weighted-subgraph-with-the-required-paths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as usize;

        let mut graph = vec![vec![]; n];
        let mut graph_rev = vec![vec![]; n];

        for edge in edges {
            let (a, b, w) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);

            graph[a].push((b, w));
            graph_rev[b].push((a, w));
        }

        let weights1 = dijkstra(src1 as usize, &graph);
        let weights2 = dijkstra(src2 as usize, &graph);
        let weights_dest = dijkstra(dest as usize, &graph_rev);

        (0..n)
            .map(|i| {
                weights1[i]
                    .saturating_add(weights2[i])
                    .saturating_add(weights_dest[i])
            })
            .filter(|&x| x != i64::MAX)
            .min()
            .unwrap_or(-1)
    }
}

fn dijkstra(from: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let mut weights = vec![i64::MAX; graph.len()];
    let mut visited = vec![false; graph.len()];

    weights[from] = 0;

    let mut pq = BinaryHeap::from([(Reverse(0), from)]);

    while let Some((_, cur)) = pq.pop() {
        if visited[cur] {
            continue;
        }

        visited[cur] = true;

        for &(next, next_weight) in &graph[cur] {
            if weights[cur] + next_weight < weights[next] {
                weights[next] = weights[cur] + next_weight;
                pq.push((Reverse(weights[next]), next));
            }
        }
    }

    weights
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2203() {
        let n = 6;
        let edges = nd_vec![
            [0, 2, 2],
            [0, 5, 6],
            [1, 0, 3],
            [1, 4, 5],
            [2, 1, 1],
            [2, 3, 3],
            [2, 3, 4],
            [3, 4, 2],
            [4, 5, 1]
        ];
        let src1 = 0;
        let src2 = 1;
        let dest = 5;
        let expected = 9;
        assert_eq!(
            Solution::minimum_weight(n, edges, src1, src2, dest),
            expected
        );
        let n = 3;
        let edges = nd_vec![[0, 1, 1], [2, 1, 1]];
        let src1 = 0;
        let src2 = 1;
        let dest = 2;
        let expected = -1;
        assert_eq!(
            Solution::minimum_weight(n, edges, src1, src2, dest),
            expected
        );
    }
}
