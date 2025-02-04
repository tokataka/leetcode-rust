///
/// # 802. Find Eventual Safe States
///
/// There is a directed graph of `n` nodes with each node labeled from `0` to `n - 1`. The graph is represented by a **0-indexed** 2D integer array `graph` where `graph[i]` is an integer array of nodes adjacent to node `i`, meaning there is an edge from node `i` to each node in `graph[i]`.
///
/// A node is a **terminal node** if there are no outgoing edges. A node is a **safe node** if every possible path starting from that node leads to a **terminal node** (or another safe node).
///
/// Return *an array containing all the **safe nodes** of the graph*. The answer should be sorted in **ascending** order.
///
/// **Example 1:**
///
/// ![Illustration of graph](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/03/17/picture1.png)
///
/// ```
/// Input: graph = [[1,2],[2,3],[5],[0],[5],[],[]]
/// Output: [2,4,5,6]
/// Explanation: The given graph is shown above.
/// Nodes 5 and 6 are terminal nodes as there are no outgoing edges from either of them.
/// Every path starting at nodes 2, 4, 5, and 6 all lead to either node 5 or 6.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: graph = [[1,2,3,4],[1,2],[3,4],[0,4],[]]
/// Output: [4]
/// Explanation:
/// Only node 4 is a terminal node, and every path starting at node 4 leads to node 4.
///
/// ```
///
/// **Constraints:**
///
/// * `n == graph.length`
/// * `1 <= n <= 10<sup>4</sup>`
/// * `0 <= graph[i].length <= n`
/// * `0 <= graph[i][j] <= n - 1`
/// * `graph[i]` is sorted in a strictly increasing order.
/// * The graph may contain self-loops.
/// * The number of edges in the graph will be in the range `[1, 4 * 10<sup>4</sup>]`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-eventual-safe-states/
// discuss: https://leetcode.com/problems/find-eventual-safe-states/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn is_safe(
            cache: &mut Vec<Option<bool>>,
            route: &mut Vec<bool>,
            graph: &Vec<Vec<i32>>,
            cur: usize,
        ) -> bool {
            if let Some(x) = cache[cur] {
                return x;
            }

            // track route
            route[cur] = true;

            // result is true if every next node is true otherwise false
            // result is also true if graph[cur] is empty which is terminal node.
            let result = graph[cur].iter().all(|&next| {
                let next = next as usize;

                // cycle found
                if route[next] {
                    return false;
                }

                is_safe(cache, route, graph, next)
            });

            // update cache and revert route
            cache[cur] = Some(result);
            route[cur] = false;

            result
        }

        let mut cache = vec![None; graph.len()];
        let mut route = vec![false; graph.len()];

        (0..graph.len())
            .filter(|&cur| is_safe(&mut cache, &mut route, &graph, cur))
            .map(|x| x as i32)
            .collect()
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_802() {
        // let graph = nd_vec![[1, 2], [2, 3], [5], [0], [5], [], []];
        // let expected = vec![2, 4, 5, 6];
        // assert_eq!(Solution::eventual_safe_nodes(graph), expected);
        // let graph = nd_vec![[1, 2, 3, 4], [1, 2], [3, 4], [0, 4], []];
        // let expected = vec![4];
        // assert_eq!(Solution::eventual_safe_nodes(graph), expected);
        let graph = nd_vec![[2, 3], [2, 3, 4], [3, 4], [], [1]];
        let expected = vec![3];
        assert_eq!(Solution::eventual_safe_nodes(graph), expected);
    }
}
