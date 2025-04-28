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

enum Time {
    Complete(i32, i32, i32),
    Incomplete(i32, i32, i32, i32),
}

use Time::*;

impl Solution {
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn get_time(
            cache: &mut HashMap<i32, Time>,
            graph: &HashMap<i32, Vec<i32>>,
            from: i32,
            to: i32,
        ) -> i32 {
            if let Some(x) = cache.get(&to) {
                let (top, second, top_idx) = match *x {
                    Complete(top, second, top_idx) => (top, second, top_idx),
                    Incomplete(mut top, mut second, mut top_idx, incomplete_idx) => {
                        let mut incomplete_time = get_time(cache, graph, to, incomplete_idx);

                        incomplete_time += if to % 2 == 0 { 2 } else { 1 };

                        if incomplete_time > top {
                            second = top;
                            top = incomplete_time;
                            top_idx = incomplete_idx;
                        } else if incomplete_time > second {
                            second = incomplete_time;
                        }

                        cache.insert(to, Complete(top, second, top_idx));

                        (top, second, top_idx)
                    }
                };

                return if top_idx == from { second } else { top };
            }

            let mut times = graph
                .get(&to)
                .unwrap()
                .iter()
                .filter(|&&next_to| next_to != from)
                .map(|&next_to| (get_time(cache, graph, to, next_to), next_to))
                .collect::<Vec<_>>();

            times.sort_unstable_by(|a, b| b.cmp(a));

            let (mut top, mut second, top_idx) = match times.len() {
                0 => (0, 0, -1),
                1 => (times[0].0, 0, times[0].1),
                _ => (times[0].0, times[1].0, times[0].1),
            };

            let time_add = if to % 2 == 0 { 2 } else { 1 };

            top += time_add;
            second += time_add;

            match from {
                -1 => cache.insert(to, Complete(top, second, top_idx)),
                from => cache.insert(to, Incomplete(top, second, top_idx, from)),
            };

            if top_idx == from {
                second
            } else {
                top
            }
        }

        let graph: HashMap<i32, Vec<i32>> = edges.iter().fold(HashMap::new(), |mut acc, x| {
            let (a, b) = (x[0], x[1]);

            acc.entry(a).or_default().push(b);
            acc.entry(b).or_default().push(a);

            acc
        });

        let mut cache: HashMap<i32, Time> = HashMap::new();

        (0..=edges.len() as i32)
            .map(|from| get_time(&mut cache, &graph, -1, from) - if from % 2 == 0 { 2 } else { 1 })
            .collect()
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
