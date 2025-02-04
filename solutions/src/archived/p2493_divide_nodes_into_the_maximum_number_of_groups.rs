///
/// # 2493. Divide Nodes Into the Maximum Number of Groups
///
/// You are given a positive integer `n` representing the number of nodes in an **undirected** graph. The nodes are labeled from `1` to `n`.
///
/// You are also given a 2D integer array `edges`, where `edges[i] = [a<sub>i, </sub>b<sub>i</sub>]` indicates that there is a **bidirectional** edge between nodes `a<sub>i</sub>` and `b<sub>i</sub>`. **Notice** that the given graph may be disconnected.
///
/// Divide the nodes of the graph into `m` groups (**1-indexed**) such that:
///
/// * Each node in the graph belongs to exactly one group.
/// * For every pair of nodes in the graph that are connected by an edge `[a<sub>i, </sub>b<sub>i</sub>]`, if `a<sub>i</sub>` belongs to the group with index `x`, and `b<sub>i</sub>` belongs to the group with index `y`, then `|y - x| = 1`.
///
/// Return *the maximum number of groups (i.e., maximum* `m`*) into which you can divide the nodes*. Return `-1` *if it is impossible to group the nodes with the given conditions*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/10/13/example1.png)
///
/// ```
/// Input: n = 6, edges = [[1,2],[1,4],[1,5],[2,6],[2,3],[4,6]]
/// Output: 4
/// Explanation: As shown in the image we:
/// - Add node 5 to the first group.
/// - Add node 1 to the second group.
/// - Add nodes 2 and 4 to the third group.
/// - Add nodes 3 and 6 to the fourth group.
/// We can see that every edge is satisfied.
/// It can be shown that that if we create a fifth group and move any node from the third or fourth group to it, at least on of the edges will not be satisfied.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 3, edges = [[1,2],[2,3],[3,1]]
/// Output: -1
/// Explanation: If we add node 1 to the first group, node 2 to the second group, and node 3 to the third group to satisfy the first two edges, we can see that the third edge will not be satisfied.
/// It can be shown that no grouping is possible.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 500`
/// * `1 <= edges.length <= 10<sup>4</sup>`
/// * `edges[i].length == 2`
/// * `1 <= a<sub>i</sub>, b<sub>i</sub> <= n`
/// * `a<sub>i</sub> != b<sub>i</sub>`
/// * There is at most one edge between any pair of vertices.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/
// discuss: https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{HashMap, VecDeque};

struct UnionFind {
    data: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            data: (0..size).collect(),
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.data[a] == a {
            return a;
        }

        self.data[a] = self.find(self.data[a]);

        self.data[a]
    }

    fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);

        self.data[b] = a;
    }

    fn groups(&mut self) -> Vec<usize> {
        let mut result = self.data.clone();

        result.sort();
        result.dedup();

        result
    }
}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut graph = vec![vec![]; n + 1];
        let mut uf = UnionFind::new(n + 1);

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            graph[a].push(b);
            graph[b].push(a);

            uf.union(a, b);
        }

        for idx in uf.groups() {
            let mut st = vec![idx];
            let mut visited = vec![None; n + 1];
            visited[idx] = Some(false);

            while let Some(cur) = st.pop() {
                let cur_color = visited[cur].unwrap();

                for &next in &graph[cur] {
                    if let Some(next_color) = visited[next] {
                        if cur_color == next_color {
                            return -1;
                        }

                        continue;
                    }

                    visited[next] = Some(!cur_color);
                    st.push(next);
                }
            }
        }

        let mut group_diameters: HashMap<usize, i32> = HashMap::new();

        for idx in 1..=n {
            let mut q = VecDeque::from([idx]);
            let mut visited = vec![false; n + 1];
            visited[idx] = true;

            for diameter in 1.. {
                let q_size = q.len();

                for _ in 0..q_size {
                    let cur = q.pop_front().unwrap();

                    for &next in &graph[cur] {
                        if !visited[next] {
                            visited[next] = true;
                            q.push_back(next);
                        }
                    }
                }

                if q.is_empty() {
                    group_diameters
                        .entry(uf.find(idx))
                        .and_modify(|x| *x = (*x).max(diameter))
                        .or_insert(diameter);

                    break;
                }
            }
        }

        group_diameters.values().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2493() {
        let n = 6;
        let edges = nd_vec![[1, 2], [1, 4], [1, 5], [2, 6], [2, 3], [4, 6]];
        let expected = 4;
        assert_eq!(Solution::magnificent_sets(n, edges), expected);
        let n = 3;
        let edges = nd_vec![[1, 2], [2, 3], [3, 1]];
        let expected = -1;
        assert_eq!(Solution::magnificent_sets(n, edges), expected);
    }
}
