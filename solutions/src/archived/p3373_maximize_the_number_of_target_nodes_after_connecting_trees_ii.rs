///
/// # 3373. Maximize the Number of Target Nodes After Connecting Trees II
///
/// There exist two **undirected** trees with `n` and `m` nodes, labeled from `[0, n - 1]` and `[0, m - 1]`, respectively.
///
/// You are given two 2D integer arrays `edges1` and `edges2` of lengths `n - 1` and `m - 1`, respectively, where `edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an edge between nodes `a<sub>i</sub>` and `b<sub>i</sub>` in the first tree and `edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]` indicates that there is an edge between nodes `u<sub>i</sub>` and `v<sub>i</sub>` in the second tree.
///
/// Node `u` is **target** to node `v` if the number of edges on the path from `u` to `v` is even. **Note** that a node is *always* **target** to itself.
///
/// Return an array of `n` integers `answer`, where `answer[i]` is the **maximum** possible number of nodes that are **target** to node `i` of the first tree if you had to connect one node from the first tree to another node in the second tree.
///
/// **Note** that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.
///
/// **Example 1:**
///
/// **Input:** edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]]
///
/// **Output:** [8,7,7,8,8]
///
/// **Explanation:**
///
/// * For `i = 0`, connect node 0 from the first tree to node 0 from the second tree.
/// * For `i = 1`, connect node 1 from the first tree to node 4 from the second tree.
/// * For `i = 2`, connect node 2 from the first tree to node 7 from the second tree.
/// * For `i = 3`, connect node 3 from the first tree to node 0 from the second tree.
/// * For `i = 4`, connect node 4 from the first tree to node 4 from the second tree.
///
/// ![](https://assets.leetcode.com/uploads/2024/09/24/3982-1.png)
///
/// **Example 2:**
///
/// **Input:** edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]]
///
/// **Output:** [3,6,6,6,6]
///
/// **Explanation:**
///
/// For every `i`, connect node `i` of the first tree with any node of the second tree.
///
/// ![](https://assets.leetcode.com/uploads/2024/09/24/3928-2.png)
///
/// **Constraints:**
///
/// * `2 <= n, m <= 10<sup>5</sup>`
/// * `edges1.length == n - 1`
/// * `edges2.length == m - 1`
/// * `edges1[i].length == edges2[i].length == 2`
/// * `edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]`
/// * `0 <= a<sub>i</sub>, b<sub>i</sub> < n`
/// * `edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]`
/// * `0 <= u<sub>i</sub>, v<sub>i</sub> < m`
/// * The input is generated such that `edges1` and `edges2` represent valid trees.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/
// discuss: https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        fn get_evens_odds(graph: &Vec<Vec<usize>>) -> [Vec<usize>; 2] {
            let mut result = [vec![], vec![]];

            let mut visited = vec![false; graph.len()];
            visited[0] = true;

            let mut st = vec![(0, 0)];

            while let Some((cur, flag)) = st.pop() {
                result[flag].push(cur);

                let next_flag = (flag + 1) % 2;

                for &next in &graph[cur] {
                    if visited[next] {
                        continue;
                    }

                    visited[next] = true;
                    st.push((next, next_flag));
                }
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

        let target2_max = get_evens_odds(&graph2)
            .into_iter()
            .map(|x| x.len())
            .max()
            .unwrap();

        get_evens_odds(&graph1)
            .into_iter()
            .flat_map(|x| {
                let max = (x.len() + target2_max) as i32;
                x.into_iter().map(move |y| (y, max))
            })
            .fold(vec![0; graph1.len()], |mut acc, (i, x)| {
                acc[i] = x;
                acc
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3373() {
        let edges1 = nd_vec![[0, 1], [0, 2], [2, 3], [2, 4]];
        let edges2 = nd_vec![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]];
        let expected = vec![8, 7, 7, 8, 8];
        assert_eq!(Solution::max_target_nodes(edges1, edges2), expected);
        let edges1 = nd_vec![[0, 1], [0, 2], [0, 3], [0, 4]];
        let edges2 = nd_vec![[0, 1], [1, 2], [2, 3]];
        let expected = vec![3, 6, 6, 6, 6];
        assert_eq!(Solution::max_target_nodes(edges1, edges2), expected);
    }
}
