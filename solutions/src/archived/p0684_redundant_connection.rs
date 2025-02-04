///
/// # 684. Redundant Connection
///
/// In this problem, a tree is an **undirected graph** that is connected and has no cycles.
///
/// You are given a graph that started as a tree with `n` nodes labeled from `1` to `n`, with one additional edge added. The added edge has two **different** vertices chosen from `1` to `n`, and was not an edge that already existed. The graph is represented as an array `edges` of length `n` where `edges[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an edge between nodes `a<sub>i</sub>` and `b<sub>i</sub>` in the graph.
///
/// Return *an edge that can be removed so that the resulting graph is a tree of* `n` *nodes*. If there are multiple answers, return the answer that occurs last in the input.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/05/02/reduntant1-1-graph.jpg)
///
/// ```
/// Input: edges = [[1,2],[1,3],[2,3]]
/// Output: [2,3]
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/05/02/reduntant1-2-graph.jpg)
///
/// ```
/// Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
/// Output: [1,4]
///
/// ```
///
/// **Constraints:**
///
/// * `n == edges.length`
/// * `3 <= n <= 1000`
/// * `edges[i].length == 2`
/// * `1 <= a<sub>i</sub> < b<sub>i</sub> <= edges.length`
/// * `a<sub>i</sub> != b<sub>i</sub>`
/// * There are no repeated edges.
/// * The given graph is connected.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/redundant-connection/
// discuss: https://leetcode.com/problems/redundant-connection/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct UnionFind {
    pub data: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            data: (0..size).collect(),
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if a == self.data[a] {
            return a;
        }

        self.data[a] = self.find(self.data[a]);

        self.data[a]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return false;
        }

        self.data[b] = a;

        true
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(1001);

        for edge in edges {
            if !uf.union(edge[0] as usize, edge[1] as usize) {
                return edge;
            }
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_684() {
        let edges = nd_vec![[1, 2], [1, 3], [2, 3]];
        let expected = vec![2, 3];
        assert_eq!(Solution::find_redundant_connection(edges), expected);
        let edges = nd_vec![[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]];
        let expected = vec![1, 4];
        assert_eq!(Solution::find_redundant_connection(edges), expected);
    }
}
