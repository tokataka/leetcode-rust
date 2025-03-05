use std::collections::HashMap;

///
/// # 3241. Time Taken to Mark All Nodes
///
/// There exists an **undirected** tree with `n` nodes numbered `0` to `n - 1`. You are given a 2D integer array `edges` of length `n - 1`, where `edges[i] = [u<sub>i</sub>, v<sub>i</sub>]` indicates that there is an edge between nodes `u<sub>i</sub>` and `v<sub>i</sub>` in the tree.
///
/// Initially, **all** nodes are **unmarked**. For each node `i`:
///
/// * If `i` is odd, the node will get marked at time `x` if there is **at least** one node *adjacent* to it which was marked at time `x - 1`.
/// * If `i` is even, the node will get marked at time `x` if there is **at least** one node *adjacent* to it which was marked at time `x - 2`.
///
/// Return an array `times` where `times[i]` is the time when all nodes get marked in the tree, if you mark node `i` at time `t = 0`.
///
/// **Note** that the answer for each `times[i]` is **independent**, i.e. when you mark node `i` all other nodes are *unmarked*.
///
/// **Example 1:**
///
/// **Input:** edges = [[0,1],[0,2]]
///
/// **Output:** [2,4,3]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/06/01/screenshot-2024-06-02-122236.png)
///
/// * For `i = 0`:
///   * Node 1 is marked at `t = 1`, and Node 2 at `t = 2`.
///
/// * For `i = 1`:
///   * Node 0 is marked at `t = 2`, and Node 2 at `t = 4`.
///
/// * For `i = 2`:
///   * Node 0 is marked at `t = 2`, and Node 1 at `t = 3`.
///
/// **Example 2:**
///
/// **Input:** edges = [[0,1]]
///
/// **Output:** [1,2]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/06/01/screenshot-2024-06-02-122249.png)
///
/// * For `i = 0`:
///   * Node 1 is marked at `t = 1`.
///
/// * For `i = 1`:
///   * Node 0 is marked at `t = 2`.
///
/// **Example 3:**
///
/// **Input:** edges = [[2,4],[0,1],[2,3],[0,2]]
///
/// **Output:** [4,6,3,5,5]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/06/03/screenshot-2024-06-03-210550.png)
///
/// **Constraints:**
///
/// * `2 <= n <= 10<sup>5</sup>`
/// * `edges.length == n - 1`
/// * `edges[i].length == 2`
/// * `0 <= edges[i][0], edges[i][1] <= n - 1`
/// * The input is generated such that `edges` represents a valid tree.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/time-taken-to-mark-all-nodes/
// discuss: https://leetcode.com/problems/time-taken-to-mark-all-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn dp(
            graph: &HashMap<i32, Vec<i32>>,
            cache: &mut HashMap<(i32, i32), i32>,
            cache2: &mut HashMap<i32, ((i32, i32), (i32, i32))>,
            prev: i32,
            cur: i32,
        ) -> i32 {
            if let Some(&((a, a_idx), (b, _))) = cache2.get(&prev) {
                return if cur == a_idx { b } else { a };
            }

            if let Some(&x) = cache.get(&(prev, cur)) {
                return x;
            }

            let mut costs = vec![];

            for &next in graph[&cur].iter().filter(|&&x| x != prev) {
                costs.push((dp(graph, cache, cache2, cur, next), next));
            }

            costs.sort_by(|a, b| b.cmp(a));

            while costs.len() < 2 {
                costs.push((0, i32::MAX));
            }

            let mut cur_cost = 0;
            if prev != cur {
                cur_cost = if cur % 2 == 0 { 2 } else { 1 };
            }

            if prev == cur {
                cache2.insert(
                    prev,
                    (
                        (costs[0].0 + cur_cost, costs[0].1),
                        (costs[1].0 + cur_cost, costs[1].1),
                    ),
                );

                return costs[0].0;
            }

            let result = cur_cost
                + if cur == costs[0].1 {
                    costs[1].0
                } else {
                    costs[0].0
                };

            cache.insert((prev, cur), result);

            result
        }

        let n = edges.len() + 1;

        let graph: HashMap<i32, Vec<i32>> = edges.iter().fold(HashMap::new(), |mut acc, x| {
            let (a, b) = (x[0], x[1]);

            acc.entry(a).or_default().push(b);
            acc.entry(b).or_default().push(a);

            acc
        });

        let mut cache: HashMap<(i32, i32), i32> = HashMap::new();
        let mut cache2: HashMap<i32, ((i32, i32), (i32, i32))> = HashMap::new();
        let mut result = Vec::with_capacity(n);

        for i in 0..n as i32 {
            result.push(dp(&graph, &mut cache, &mut cache2, i, i));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3241() {
        let edges = nd_vec![[0, 1], [0, 2]];
        let expected = vec![2, 4, 3];
        assert_eq!(Solution::time_taken(edges), expected);
        let edges = nd_vec![[0, 1]];
        let expected = vec![1, 2];
        assert_eq!(Solution::time_taken(edges), expected);
        let edges = nd_vec![[2, 4], [0, 1], [2, 3], [0, 2]];
        let expected = vec![4, 6, 3, 5, 5];
        assert_eq!(Solution::time_taken(edges), expected);
    }
}
