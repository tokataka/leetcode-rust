///
/// # 3311. Construct 2D Grid Matching Graph Layout
///
/// You are given a 2D integer array `edges` representing an **undirected** graph having `n` nodes, where `edges[i] = [u<sub>i</sub>, v<sub>i</sub>]` denotes an edge between nodes `u<sub>i</sub>` and `v<sub>i</sub>`.
///
/// Construct a 2D grid that satisfies these conditions:
///
/// * The grid contains **all nodes** from `0` to `n - 1` in its cells, with each node appearing exactly **once**.
/// * Two nodes should be in adjacent grid cells (**horizontally** or **vertically**) **if and only if** there is an edge between them in `edges`.
///
/// It is guaranteed that `edges` can form a 2D grid that satisfies the conditions.
///
/// Return a 2D integer array satisfying the conditions above. If there are multiple solutions, return *any* of them.
///
/// **Example 1:**
///
/// **Input:** n = 4, edges = [[0,1],[0,2],[1,3],[2,3]]
///
/// **Output:** [[3,1],[2,0]]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/08/11/screenshot-from-2024-08-11-14-07-59.png)
///
/// **Example 2:**
///
/// **Input:** n = 5, edges = [[0,1],[1,3],[2,3],[2,4]]
///
/// **Output:** [[4,2,3,1,0]]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/08/11/screenshot-from-2024-08-11-14-06-02.png)
///
/// **Example 3:**
///
/// **Input:** n = 9, edges = [[0,1],[0,4],[0,5],[1,7],[2,3],[2,4],[2,5],[3,6],[4,6],[4,7],[6,8],[7,8]]
///
/// **Output:** [[8,6,3],[7,4,2],[1,0,5]]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/08/11/screenshot-from-2024-08-11-14-06-38.png)
///
/// **Constraints:**
///
/// * `2 <= n <= 5 * 10<sup>4</sup>`
/// * `1 <= edges.length <= 10<sup>5</sup>`
/// * `edges[i] = [u<sub>i</sub>, v<sub>i</sub>]`
/// * `0 <= u<sub>i</sub> < v<sub>i</sub> < n`
/// * All the edges are distinct.
/// * The input is generated such that `edges` can form a 2D grid that satisfies the conditions.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/construct-2d-grid-matching-graph-layout/
// discuss: https://leetcode.com/problems/construct-2d-grid-matching-graph-layout/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            graph[a].push(b);
            graph[b].push(a);
        }

        let (corner, min_len) = graph
            .iter()
            .enumerate()
            .map(|(i, x)| (i, x.len()))
            .min_by_key(|&(_, x)| x)
            .unwrap();

        let mut used = vec![false; n];
        used[corner] = true;

        let mut cur_row = vec![corner];
        let mut cur = corner;

        while let Some(&next) = graph[cur]
            .iter()
            .filter(|&&next| !used[next])
            .min_by_key(|&&next| graph[next].len())
        {
            cur_row.push(next);
            used[next] = true;

            if graph[next].len() == min_len {
                break;
            }

            cur = next;
        }

        let width = cur_row.len();
        let height = n / width;

        let mut result = vec![vec![0; width]; height];

        let mut next_row = vec![];

        for i in 0..height {
            for (j, el) in cur_row.drain(..).enumerate() {
                result[i][j] = el as i32;

                if let Some(&next_el) = graph[el].iter().find(|&&x| !used[x]) {
                    used[next_el] = true;
                    next_row.push(next_el);
                }
            }

            std::mem::swap(&mut cur_row, &mut next_row);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3311() {
        let n = 4;
        let edges = nd_vec![[0, 1], [0, 2], [1, 3], [2, 3]];
        let expected = nd_vec![[3, 1], [2, 0]];
        assert_eq!(Solution::construct_grid_layout(n, edges), expected);
        let n = 5;
        let edges = nd_vec![[0, 1], [1, 3], [2, 3], [2, 4]];
        let expected = nd_vec![[4, 2, 3, 1, 0]];
        assert_eq!(Solution::construct_grid_layout(n, edges), expected);
        let n = 9;
        let edges = nd_vec![
            [0, 1],
            [0, 4],
            [0, 5],
            [1, 7],
            [2, 3],
            [2, 4],
            [2, 5],
            [3, 6],
            [4, 6],
            [4, 7],
            [6, 8],
            [7, 8]
        ];
        let expected = nd_vec![[8, 6, 3], [7, 4, 2], [1, 0, 5]];
        assert_eq!(Solution::construct_grid_layout(n, edges), expected);
    }
}
