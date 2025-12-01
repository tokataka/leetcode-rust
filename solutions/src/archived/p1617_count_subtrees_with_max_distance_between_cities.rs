///
/// # 1617. Count Subtrees With Max Distance Between Cities
///
/// There are `n` cities numbered from `1` to `n`. You are given an array `edges` of size `n-1`, where `edges[i] = [u<sub>i</sub>, v<sub>i</sub>]` represents a bidirectional edge between cities `u<sub>i</sub>` and `v<sub>i</sub>`. There exists a unique path between each pair of cities. In other words, the cities form a **tree**.
///
/// A **subtree** is a subset of cities where every city is reachable from every other city in the subset, where the path between each pair passes through only the cities from the subset. Two subtrees are different if there is a city in one subtree that is not present in the other.
///
/// For each `d` from `1` to `n-1`, find the number of subtrees in which the **maximum distance** between any two cities in the subtree is equal to `d`.
///
/// Return *an array of size* `n-1` *where the* `d<sup>th</sup>` *element **(1-indexed)** is the number of subtrees in which the **maximum distance** between any two cities is equal to* `d`.
///
/// **Notice** that the **distance** between the two cities is the number of edges in the path between them.
///
/// **Example 1:**
///
/// **![](https://assets.leetcode.com/uploads/2020/09/21/p1.png)**
///
/// ```
/// Input: n = 4, edges = [[1,2],[2,3],[2,4]]
/// Output: [3,4,0]
/// Explanation:
/// The subtrees with subsets {1,2}, {2,3} and {2,4} have a max distance of 1.
/// The subtrees with subsets {1,2,3}, {1,2,4}, {2,3,4} and {1,2,3,4} have a max distance of 2.
/// No subtree has two nodes where the max distance between them is 3.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 2, edges = [[1,2]]
/// Output: [1]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 3, edges = [[1,2],[2,3]]
/// Output: [2,1]
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 15`
/// * `edges.length == n-1`
/// * `edges[i].length == 2`
/// * `1 <= u<sub>i</sub>, v<sub>i</sub> <= n`
/// * All pairs `(u<sub>i</sub>, v<sub>i</sub>)` are distinct.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-subtrees-with-max-distance-between-cities/
// discuss: https://leetcode.com/problems/count-subtrees-with-max-distance-between-cities/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn _count_subgraphs_for_each_diameter(
            cur: usize,
            prev: usize,
            n: usize,
            graph: &Vec<Vec<usize>>,
        ) -> (Vec<i32>, Vec<i32>) {
            let mut diameters = vec![0; n];
            let mut distances = vec![0; n];
            distances[0] = 1;

            let mut children_distances = vec![];

            for &next in graph[cur].iter().filter(|&&x| x != prev) {
                let (next_diameters, next_distances) =
                    _count_subgraphs_for_each_diameter(next, cur, n, graph);

                for (i, &d) in next_diameters.iter().enumerate() {
                    diameters[i] += d;
                }

                for (i, &d) in next_distances.iter().enumerate().take(n - 1) {
                    distances[i + 1] += d;
                    diameters[i + 1] += d;
                }

                children_distances.push(next_distances);
            }

            for (i, di) in children_distances.iter().enumerate() {
                for dj in children_distances.iter().skip(i + 1) {
                    for dist_i in 0..n {
                        for dist_j in 0..n {
                            if dist_i + dist_j + 2 < n {
                                diameters[dist_i + dist_j + 2] += di[dist_i] * dj[dist_j];
                            }
                        }
                    }
                }
            }

            (diameters, distances)
        }

        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (a, b) = (edge[0] as usize - 1, edge[1] as usize - 1);
            graph[a].push(b);
            graph[b].push(a);
        }

        _count_subgraphs_for_each_diameter(0, 0, n, &graph).0[1..].to_vec()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1617() {
        assert_eq!(
            Solution::count_subgraphs_for_each_diameter(4, nd_vec![[1, 2], [2, 3], [2, 4]]),
            vec![3, 4, 0]
        );
        assert_eq!(
            Solution::count_subgraphs_for_each_diameter(2, nd_vec![[1, 2]]),
            vec![1]
        );
        assert_eq!(
            Solution::count_subgraphs_for_each_diameter(3, nd_vec![[1, 2], [2, 3]]),
            vec![2, 1]
        );
    }
}
