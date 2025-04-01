///
/// # 2685. Count the Number of Complete Components
///
/// You are given an integer `n`. There is an **undirected** graph with `n` vertices, numbered from `0` to `n - 1`. You are given a 2D integer array `edges` where `edges[i] = [a<sub>i</sub>, b<sub>i</sub>]` denotes that there exists an **undirected** edge connecting vertices `a<sub>i</sub>` and `b<sub>i</sub>`.
///
/// Return *the number of **complete connected components** of the graph*.
///
/// A **connected component** is a subgraph of a graph in which there exists a path between any two vertices, and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.
///
/// A connected component is said to be **complete** if there exists an edge between every pair of its vertices.
///
/// **Example 1:**
///
/// **![](https://assets.leetcode.com/uploads/2023/04/11/screenshot-from-2023-04-11-23-31-23.png)**
///
/// ```
/// Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4]]
/// Output: 3
/// Explanation: From the picture above, one can see that all of the components of this graph are complete.
///
/// ```
///
/// **Example 2:**
///
/// **![](https://assets.leetcode.com/uploads/2023/04/11/screenshot-from-2023-04-11-23-32-00.png)**
///
/// ```
/// Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4],[3,5]]
/// Output: 1
/// Explanation: The component containing vertices 0, 1, and 2 is complete since there is an edge between every pair of two vertices. On the other hand, the component containing vertices 3, 4, and 5 is not complete since there is no edge between vertices 4 and 5. Thus, the number of complete components in this graph is 1.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 50`
/// * `0 <= edges.length <= n * (n - 1) / 2`
/// * `edges[i].length == 2`
/// * `0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1`
/// * `a<sub>i</sub> != b<sub>i</sub>`
/// * There are no repeated edges.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-complete-components/
// discuss: https://leetcode.com/problems/count-the-number-of-complete-components/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct UnionFind {
    data: Vec<usize>,
    size: Vec<usize>,
    edge_count: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            data: (0..n).collect(),
            size: vec![1; n],
            edge_count: vec![0; n],
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

        if a != b {
            let (a, b) = match self.size[a] >= self.size[b] {
                true => (a, b),
                false => (b, a),
            };

            self.data[b] = a;
            self.size[a] += self.size[b];
            self.edge_count[a] += self.edge_count[b] + 1;
        } else {
            self.edge_count[a] += 1;
        }
    }

    fn complete_count(&mut self) -> i32 {
        let n = self.data.len();
        let mut visited = vec![false; n];
        let mut result = 0;

        for a in 0..n {
            let a = self.find(a);
            if visited[a] {
                continue;
            }

            visited[a] = true;

            if self.size[a] * (self.size[a] - 1) / 2 == self.edge_count[a] {
                result += 1;
            }
        }

        result
    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);

        for edge in edges {
            uf.union(edge[0] as usize, edge[1] as usize);
        }

        uf.complete_count()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2685() {
        let n = 6;
        let edges = nd_vec![[0, 1], [0, 2], [1, 2], [3, 4]];
        let expected = 3;
        assert_eq!(Solution::count_complete_components(n, edges), expected);
        let n = 6;
        let edges = nd_vec![[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]];
        let expected = 1;
        assert_eq!(Solution::count_complete_components(n, edges), expected);
    }
}
