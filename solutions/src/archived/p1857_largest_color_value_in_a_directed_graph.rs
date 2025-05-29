use std::collections::VecDeque;

///
/// # 1857. Largest Color Value in a Directed Graph
///
/// There is a **directed graph** of `n` colored nodes and `m` edges. The nodes are numbered from `0` to `n - 1`.
///
/// You are given a string `colors` where `colors[i]` is a lowercase English letter representing the **color** of the `i<sup>th</sup>` node in this graph (**0-indexed**). You are also given a 2D array `edges` where `edges[j] = [a<sub>j</sub>, b<sub>j</sub>]` indicates that there is a **directed edge** from node `a<sub>j</sub>` to node `b<sub>j</sub>`.
///
/// A valid **path** in the graph is a sequence of nodes `x<sub>1</sub> -> x<sub>2</sub> -> x<sub>3</sub> -> ... -> x<sub>k</sub>` such that there is a directed edge from `x<sub>i</sub>` to `x<sub>i+1</sub>` for every `1 <= i < k`. The **color value** of the path is the number of nodes that are colored the **most frequently** occurring color along that path.
///
/// Return *the **largest color value** of any valid path in the given graph, or* `-1` *if the graph contains a cycle*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/21/leet1.png)
///
/// ```
/// Input: colors = "abaca", edges = [[0,1],[0,2],[2,3],[3,4]]
/// Output: 3
/// Explanation: The path 0 -> 2 -> 3 -> 4 contains 3 nodes that are colored "a" (red in the above image).
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/21/leet2.png)
///
/// ```
/// Input: colors = "a", edges = [[0,0]]
/// Output: -1
/// Explanation: There is a cycle from 0 to 0.
///
/// ```
///
/// **Constraints:**
///
/// * `n == colors.length`
/// * `m == edges.length`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `0 <= m <= 10<sup>5</sup>`
/// * `colors` consists of lowercase English letters.
/// * `0 <= a<sub>j</sub>, b<sub>j</sub> < n`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-color-value-in-a-directed-graph/
// discuss: https://leetcode.com/problems/largest-color-value-in-a-directed-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();

        let colors = colors
            .bytes()
            .map(|x| (x - b'a') as usize)
            .collect::<Vec<_>>();

        let mut graph = vec![vec![]; n];
        let mut in_degree = vec![0; n];

        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            graph[a].push(b);

            in_degree[b] += 1;
        }

        let mut topologic = Vec::with_capacity(n);

        let mut q = in_degree
            .iter()
            .enumerate()
            .filter(|&(_, &x)| x == 0)
            .map(|(i, _)| i)
            .collect::<VecDeque<_>>();

        while let Some(cur) = q.pop_front() {
            topologic.push(cur);

            for &next in &graph[cur] {
                in_degree[next] -= 1;

                if in_degree[next] == 0 {
                    q.push_back(next);
                }
            }
        }

        if topologic.len() < n {
            return -1;
        }

        let mut counts = vec![[0; 26]; n];
        let mut result = 0;

        for cur in topologic {
            counts[cur][colors[cur]] += 1;
            result = result.max(counts[cur][colors[cur]]);

            for &next in &graph[cur] {
                for ch in 0..26 {
                    counts[next][ch] = counts[next][ch].max(counts[cur][ch])
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1857() {
        let colors = "abaca".to_owned();
        let edges = nd_vec![[0, 1], [0, 2], [2, 3], [3, 4]];
        let expected = 3;
        assert_eq!(Solution::largest_path_value(colors, edges), expected);
        let colors = "a".to_owned();
        let edges = nd_vec![[0, 0]];
        let expected = -1;
        assert_eq!(Solution::largest_path_value(colors, edges), expected);
    }
}
