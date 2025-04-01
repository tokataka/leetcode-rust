///
/// # 3108. Minimum Cost Walk in Weighted Graph
///
/// There is an undirected weighted graph with `n` vertices labeled from `0` to `n - 1`.
///
/// You are given the integer `n` and an array `edges`, where `edges[i] = [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]` indicates that there is an edge between vertices `u<sub>i</sub>` and `v<sub>i</sub>` with a weight of `w<sub>i</sub>`.
///
/// A walk on a graph is a sequence of vertices and edges. The walk starts and ends with a vertex, and each edge connects the vertex that comes before it and the vertex that comes after it. It's important to note that a walk may visit the same edge or vertex more than once.
///
/// The **cost** of a walk starting at node `u` and ending at node `v` is defined as the bitwise `AND` of the weights of the edges traversed during the walk. In other words, if the sequence of edge weights encountered during the walk is `w<sub>0</sub>, w<sub>1</sub>, w<sub>2</sub>, ..., w<sub>k</sub>`, then the cost is calculated as `w<sub>0</sub> & w<sub>1</sub> & w<sub>2</sub> & ... & w<sub>k</sub>`, where `&` denotes the bitwise `AND` operator.
///
/// You are also given a 2D array `query`, where `query[i] = [s<sub>i</sub>, t<sub>i</sub>]`. For each query, you need to find the minimum cost of the walk starting at vertex `s<sub>i</sub>` and ending at vertex `t<sub>i</sub>`. If there exists no such walk, the answer is `-1`.
///
/// Return *the array* `answer`*, where* `answer[i]` *denotes the **minimum** cost of a walk for query* `i`.
///
/// **Example 1:**
///
/// **Input:** n = 5, edges = [[0,1,7],[1,3,7],[1,2,1]], query = [[0,3],[3,4]]
///
/// **Output:** [1,-1]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/01/31/q4_example1-1.png)
///
/// To achieve the cost of 1 in the first query, we need to move on the following edges: `0->1` (weight 7), `1->2` (weight 1), `2->1` (weight 1), `1->3` (weight 7).
///
/// In the second query, there is no walk between nodes 3 and 4, so the answer is -1.
///
/// **Example 2:**
///
/// **Input:** n = 3, edges = [[0,2,7],[0,1,15],[1,2,6],[1,2,1]], query = [[1,2]]
///
/// **Output:** [0]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/01/31/q4_example2e.png)
///
/// To achieve the cost of 0 in the first query, we need to move on the following edges: `1->2` (weight 1), `2->1` (weight 6), `1->2` (weight 1).
///
/// **Constraints:**
///
/// * `2 <= n <= 10<sup>5</sup>`
/// * `0 <= edges.length <= 10<sup>5</sup>`
/// * `edges[i].length == 3`
/// * `0 <= u<sub>i</sub>, v<sub>i</sub> <= n - 1`
/// * `u<sub>i</sub> != v<sub>i</sub>`
/// * `0 <= w<sub>i</sub> <= 10<sup>5</sup>`
/// * `1 <= query.length <= 10<sup>5</sup>`
/// * `query[i].length == 2`
/// * `0 <= s<sub>i</sub>, t<sub>i</sub> <= n - 1`
/// * `s<sub>i</sub> != t<sub>i</sub>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/
// discuss: https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct UnionFind {
    root: Vec<usize>,
    weight: Vec<i32>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            root: (0..n).collect(),
            weight: vec![i32::MAX; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.root[a] == a {
            return self.root[a];
        }

        self.root[a] = self.find(self.root[a]);

        self.root[a]
    }

    fn union(&mut self, a: usize, b: usize, w: i32) {
        let a = self.find(a);
        let b = self.find(b);

        if a != b {
            let (a, b) = match self.size[a] >= self.size[b] {
                true => (a, b),
                false => (b, a),
            };

            let w = self.weight[a] & self.weight[b] & w;

            self.root[b] = a;
            self.weight[a] = w;
            self.size[a] += self.size[b];
        } else {
            self.weight[a] &= w;
        }
    }

    fn query(&mut self, a: usize, b: usize) -> i32 {
        match (self.find(a), self.find(b)) {
            (a, b) if a == b => self.weight[a],
            _ => -1,
        }
    }
}

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(n as usize);

        for edge in edges {
            uf.union(edge[0] as usize, edge[1] as usize, edge[2]);
        }

        query
            .into_iter()
            .map(|q| uf.query(q[0] as usize, q[1] as usize))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3108() {
        let n = 5;
        let edges = nd_vec![[0, 1, 7], [1, 3, 7], [1, 2, 1]];
        let query = nd_vec![[0, 3], [3, 4]];
        let expected = vec![1, -1];
        assert_eq!(Solution::minimum_cost(n, edges, query), expected);
        let n = 3;
        let edges = nd_vec![[0, 2, 7], [0, 1, 15], [1, 2, 6], [1, 2, 1]];
        let query = nd_vec![[1, 2]];
        let expected = vec![0];
        assert_eq!(Solution::minimum_cost(n, edges, query), expected);
    }
}
