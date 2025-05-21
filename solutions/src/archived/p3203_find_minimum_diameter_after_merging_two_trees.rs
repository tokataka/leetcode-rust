///
/// # 3203. Find Minimum Diameter After Merging Two Trees
///
/// There exist two **undirected** trees with `n` and `m` nodes, numbered from `0` to `n - 1` and from `0` to `m - 1`, respectively. You are given two 2D integer arrays `edges1` and `edges2` of lengths `n - 1` and `m - 1`, respectively, where `edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an edge between nodes `a<sub>i</sub>` and `b<sub>i</sub>` in the first tree and `edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]` indicates that there is an edge between nodes `u<sub>i</sub>` and `v<sub>i</sub>` in the second tree.
///
/// You must connect one node from the first tree with another node from the second tree with an edge.
///
/// Return the **minimum** possible **diameter** of the resulting tree.
///
/// The **diameter** of a tree is the length of the *longest* path between any two nodes in the tree.
///
/// **Example 1:**![](https://assets.leetcode.com/uploads/2024/04/22/example11-transformed.png)
///
/// **Input:** edges1 = [[0,1],[0,2],[0,3]], edges2 = [[0,1]]
///
/// **Output:** 3
///
/// **Explanation:**
///
/// We can obtain a tree of diameter 3 by connecting node 0 from the first tree with any node from the second tree.
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2024/04/22/example211.png)
///
/// **Input:** edges1 = [[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]], edges2 = [[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]]
///
/// **Output:** 5
///
/// **Explanation:**
///
/// We can obtain a tree of diameter 5 by connecting node 0 from the first tree with node 0 from the second tree.
///
/// **Constraints:**
///
/// * `1 <= n, m <= 10<sup>5</sup>`
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

// problem: https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/
// discuss: https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        fn dfs(cur: usize, prev: usize, graph: &Vec<Vec<usize>>) -> (i32, i32) {
            let mut max_diameter = 0;
            let mut lengths = vec![0, 0];

            for &next in &graph[cur] {
                if next == prev {
                    continue;
                }

                let (diameter, length) = dfs(next, cur, graph);

                max_diameter = max_diameter.max(diameter);
                lengths.push(length);
            }

            lengths.sort_unstable_by(|a, b| b.cmp(a));

            (max_diameter.max(lengths[0] + lengths[1]), lengths[0] + 1)
        }

        fn find_diameter(edges: &Vec<Vec<i32>>) -> i32 {
            let mut graph = vec![vec![]; edges.len() + 1];

            for edge in edges {
                let (a, b) = (edge[0] as usize, edge[1] as usize);

                graph[a].push(b);
                graph[b].push(a);
            }

            dfs(0, 0, &graph).0
        }

        let diameter1 = find_diameter(&edges1);
        let diameter2 = find_diameter(&edges2);

        diameter1
            .max(diameter2)
            .max((diameter1 + 1) / 2 + (diameter2 + 1) / 2 + 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3203() {
        let edges1 = nd_vec![[0, 1], [0, 2], [0, 3]];
        let edges2 = nd_vec![[0, 1]];
        let expected = 3;
        assert_eq!(
            Solution::minimum_diameter_after_merge(edges1, edges2),
            expected
        );
        let edges1 = nd_vec![[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]];
        let edges2 = nd_vec![[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]];
        let expected = 5;
        assert_eq!(
            Solution::minimum_diameter_after_merge(edges1, edges2),
            expected
        );
    }
}
