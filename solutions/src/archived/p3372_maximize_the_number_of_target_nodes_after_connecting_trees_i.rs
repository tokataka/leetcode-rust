///
/// # 3372. Maximize the Number of Target Nodes After Connecting Trees I
///
/// There exist two **undirected** trees with `n` and `m` nodes, with **distinct** labels in ranges `[0, n - 1]` and `[0, m - 1]`, respectively.
///
/// You are given two 2D integer arrays `edges1` and `edges2` of lengths `n - 1` and `m - 1`, respectively, where `edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an edge between nodes `a<sub>i</sub>` and `b<sub>i</sub>` in the first tree and `edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]` indicates that there is an edge between nodes `u<sub>i</sub>` and `v<sub>i</sub>` in the second tree. You are also given an integer `k`.
///
/// Node `u` is **target** to node `v` if the number of edges on the path from `u` to `v` is less than or equal to `k`. **Note** that a node is *always* **target** to itself.
///
/// Return an array of `n` integers `answer`, where `answer[i]` is the **maximum** possible number of nodes **target** to node `i` of the first tree if you have to connect one node from the first tree to another node in the second tree.
///
/// **Note** that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.
///
/// **Example 1:**
///
/// **Input:** edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]], k = 2
///
/// **Output:** [9,7,9,8,8]
///
/// **Explanation:**
///
/// * For `i = 0`, connect node 0 from the first tree to node 0 from the second tree.
/// * For `i = 1`, connect node 1 from the first tree to node 0 from the second tree.
/// * For `i = 2`, connect node 2 from the first tree to node 4 from the second tree.
/// * For `i = 3`, connect node 3 from the first tree to node 4 from the second tree.
/// * For `i = 4`, connect node 4 from the first tree to node 4 from the second tree.
///
/// ![](https://assets.leetcode.com/uploads/2024/09/24/3982-1.png)
///
/// **Example 2:**
///
/// **Input:** edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]], k = 1
///
/// **Output:** [6,3,3,3,3]
///
/// **Explanation:**
///
/// For every `i`, connect node `i` of the first tree with any node of the second tree.
///
/// ![](https://assets.leetcode.com/uploads/2024/09/24/3928-2.png)
///
/// **Constraints:**
///
/// * `2 <= n, m <= 1000`
/// * `edges1.length == n - 1`
/// * `edges2.length == m - 1`
/// * `edges1[i].length == edges2[i].length == 2`
/// * `edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]`
/// * `0 <= a<sub>i</sub>, b<sub>i</sub> < n`
/// * `edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]`
/// * `0 <= u<sub>i</sub>, v<sub>i</sub> < m`
/// * The input is generated such that `edges1` and `edges2` represent valid trees.
/// * `0 <= k <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/
// discuss: https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        fn get_targets(cur: usize, prev: usize, k_remain: i32, graph: &Vec<Vec<usize>>) -> i32 {
            if k_remain == 0 {
                return 1;
            }

            let mut result = 1;

            for &next in graph[cur].iter().filter(|&&x| x != prev) {
                result += get_targets(next, cur, k_remain - 1, graph);
            }

            result
        }

        let [graph1, graph2] = [edges1, edges2].map(|edges| {
            let mut graph = vec![vec![]; edges.len() + 1];

            for edge in edges {
                let (a, b) = (edge[0] as usize, edge[1] as usize);
                graph[a].push(b);
                graph[b].push(a);
            }

            graph
        });

        let targets2_max = match k {
            0 => return vec![1; n],
            1 => 1,
            k => (0..m)
                .map(|i| get_targets(i, i, k - 1, &graph2))
                .max()
                .unwrap(),
        };

        (0..n)
            .map(|i| get_targets(i, i, k, &graph1) + targets2_max)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3372() {
        // let edges1 = nd_vec![[0, 1], [0, 2], [2, 3], [2, 4]];
        // let edges2 = nd_vec![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]];
        // let k = 2;
        // let expected = vec![9, 7, 9, 8, 8];
        // assert_eq!(Solution::max_target_nodes(edges1, edges2, k), expected);
        // let edges1 = nd_vec![[0, 1], [0, 2], [0, 3], [0, 4]];
        // let edges2 = nd_vec![[0, 1], [1, 2], [2, 3]];
        // let k = 1;
        // let expected = vec![6, 3, 3, 3, 3];
        // assert_eq!(Solution::max_target_nodes(edges1, edges2, k), expected);
        let edges1 = nd_vec![[3, 0], [2, 1], [5, 2], [6, 3], [5, 4], [5, 6]];
        let edges2 = nd_vec![
            [5, 0],
            [1, 5],
            [6, 1],
            [3, 6],
            [2, 3],
            [4, 2],
            [7, 4],
            [7, 8]
        ];
        let k = 4;
        let expected = vec![13, 13, 14, 14, 14, 14, 14];
        assert_eq!(Solution::max_target_nodes(edges1, edges2, k), expected);
    }
}
