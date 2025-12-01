///
/// # 2065. Maximum Path Quality of a Graph
///
/// There is an **undirected** graph with `n` nodes numbered from `0` to `n - 1` (**inclusive**). You are given a **0-indexed** integer array `values` where `values[i]` is the **value** of the `i<sup>th</sup>` node. You are also given a **0-indexed** 2D integer array `edges`, where each `edges[j] = [u<sub>j</sub>, v<sub>j</sub>, time<sub>j</sub>]` indicates that there is an undirected edge between the nodes `u<sub>j</sub>` and `v<sub>j</sub>`,<sub> </sub>and it takes `time<sub>j</sub>` seconds to travel between the two nodes. Finally, you are given an integer `maxTime`.
///
/// A **valid** **path** in the graph is any path that starts at node `0`, ends at node `0`, and takes **at most** `maxTime` seconds to complete. You may visit the same node multiple times. The **quality** of a valid path is the **sum** of the values of the **unique nodes** visited in the path (each node's value is added **at most once** to the sum).
///
/// Return *the **maximum** quality of a valid path*.
///
/// **Note:** There are **at most four** edges connected to each node.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/10/19/ex1drawio.png)
///
/// ```
/// Input: values = [0,32,10,43], edges = [[0,1,10],[1,2,15],[0,3,10]], maxTime = 49
/// Output: 75
/// Explanation:
/// One possible path is 0 -> 1 -> 0 -> 3 -> 0. The total time taken is 10 + 10 + 10 + 10 = 40 <= 49.
/// The nodes visited are 0, 1, and 3, giving a maximal path quality of 0 + 32 + 43 = 75.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/10/19/ex2drawio.png)
///
/// ```
/// Input: values = [5,10,15,20], edges = [[0,1,10],[1,2,10],[0,3,10]], maxTime = 30
/// Output: 25
/// Explanation:
/// One possible path is 0 -> 3 -> 0. The total time taken is 10 + 10 = 20 <= 30.
/// The nodes visited are 0 and 3, giving a maximal path quality of 5 + 20 = 25.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/10/19/ex31drawio.png)
///
/// ```
/// Input: values = [1,2,3,4], edges = [[0,1,10],[1,2,11],[2,3,12],[1,3,13]], maxTime = 50
/// Output: 7
/// Explanation:
/// One possible path is 0 -> 1 -> 3 -> 1 -> 0. The total time taken is 10 + 13 + 13 + 10 = 46 <= 50.
/// The nodes visited are 0, 1, and 3, giving a maximal path quality of 1 + 2 + 4 = 7.
///
/// ```
///
/// **Constraints:**
///
/// * `n == values.length`
/// * `1 <= n <= 1000`
/// * `0 <= values[i] <= 10<sup>8</sup>`
/// * `0 <= edges.length <= 2000`
/// * `edges[j].length == 3 `
/// * `0 <= u<sub>j </sub>< v<sub>j</sub> <= n - 1`
/// * `10 <= time<sub>j</sub>, maxTime <= 100`
/// * All the pairs `[u<sub>j</sub>, v<sub>j</sub>]` are **unique**.
/// * There are **at most four** edges connected to each node.
/// * The graph may not be connected.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-path-quality-of-a-graph/
// discuss: https://leetcode.com/problems/maximum-path-quality-of-a-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        fn _maximal_path_quality(
            cur: usize,
            remain_time: i32,
            quality: i32,
            visit_count: &mut [i32],
            values: &[i32],
            graph: &Vec<Vec<(usize, i32)>>,
        ) -> i32 {
            let mut result = if cur == 0 { quality } else { 0 };

            for &(next, time) in &graph[cur] {
                if time > remain_time {
                    continue;
                }

                let mut next_quality = quality;

                if visit_count[next] == 0 {
                    next_quality += values[next];
                }

                visit_count[next] += 1;

                result = result.max(_maximal_path_quality(
                    next,
                    remain_time - time,
                    next_quality,
                    visit_count,
                    values,
                    graph,
                ));

                visit_count[next] -= 1;
            }

            result
        }

        let n = values.len();

        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (a, b, t) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[a].push((b, t));
            graph[b].push((a, t));
        }

        let mut visit_count = vec![0; n];
        visit_count[0] = 1;

        _maximal_path_quality(0, max_time, values[0], &mut visit_count, &values, &graph)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2065() {
        assert_eq!(
            Solution::maximal_path_quality(
                vec![0, 32, 10, 43],
                nd_vec![[0, 1, 10], [1, 2, 15], [0, 3, 10]],
                49
            ),
            75
        );
        assert_eq!(
            Solution::maximal_path_quality(
                vec![5, 10, 15, 20],
                nd_vec![[0, 1, 10], [1, 2, 10], [0, 3, 10]],
                30
            ),
            25
        );
        assert_eq!(
            Solution::maximal_path_quality(
                vec![1, 2, 3, 4],
                nd_vec![[0, 1, 10], [1, 2, 11], [2, 3, 12], [1, 3, 13]],
                50
            ),
            7
        );
    }
}
