///
/// # 417. Pacific Atlantic Water Flow
///
/// There is an `m x n` rectangular island that borders both the **Pacific Ocean** and **Atlantic Ocean**. The **Pacific Ocean** touches the island's left and top edges, and the **Atlantic Ocean** touches the island's right and bottom edges.
///
/// The island is partitioned into a grid of square cells. You are given an `m x n` integer matrix `heights` where `heights[r][c]` represents the **height above sea level** of the cell at coordinate `(r, c)`.
///
/// The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is **less than or equal to** the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.
///
/// Return *a **2D list** of grid coordinates* `result` *where* `result[i] = [r<sub>i</sub>, c<sub>i</sub>]` *denotes that rain water can flow from cell* `(r<sub>i</sub>, c<sub>i</sub>)` *to **both** the Pacific and Atlantic oceans*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/06/08/waterflow-grid.jpg)
///
/// ```
/// Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
/// Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
/// Explanation: The following cells can flow to the Pacific and Atlantic oceans, as shown below:
/// [0,4]: [0,4] -> Pacific Ocean
///        [0,4] -> Atlantic Ocean
/// [1,3]: [1,3] -> [0,3] -> Pacific Ocean
///        [1,3] -> [1,4] -> Atlantic Ocean
/// [1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean
///        [1,4] -> Atlantic Ocean
/// [2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean
///        [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
/// [3,0]: [3,0] -> Pacific Ocean
///        [3,0] -> [4,0] -> Atlantic Ocean
/// [3,1]: [3,1] -> [3,0] -> Pacific Ocean
///        [3,1] -> [4,1] -> Atlantic Ocean
/// [4,0]: [4,0] -> Pacific Ocean
///        [4,0] -> Atlantic Ocean
/// Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: heights = [[1]]
/// Output: [[0,0]]
/// Explanation: The water can flow from the only cell to the Pacific and Atlantic oceans.
///
/// ```
///
/// **Constraints:**
///
/// * `m == heights.length`
/// * `n == heights[r].length`
/// * `1 <= m, n <= 200`
/// * `0 <= heights[r][c] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/pacific-atlantic-water-flow/
// discuss: https://leetcode.com/problems/pacific-atlantic-water-flow/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        let m = heights[0].len();

        let mut data = vec![vec![[false; 2]; m]; n];

        let mut st = vec![];

        for i in 0..n {
            data[i][0][0] = true;
            st.push((i, 0, 0));
            data[i][m - 1][1] = true;
            st.push((i, m - 1, 1));
        }

        for j in 0..m {
            data[0][j][0] = true;
            st.push((0, j, 0));
            data[n - 1][j][1] = true;
            st.push((n - 1, j, 1));
        }

        while let Some((i, j, ocean)) = st.pop() {
            for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ci, cj) = match (i.checked_add_signed(di), (j.checked_add_signed(dj))) {
                    (Some(ci), Some(cj)) if ci < n && cj < m => (ci, cj),
                    _ => continue,
                };

                if data[ci][cj][ocean] {
                    continue;
                }

                if heights[i][j] > heights[ci][cj] {
                    continue;
                }

                data[ci][cj][ocean] = true;
                st.push((ci, cj, ocean));
            }
        }

        let mut result = vec![];

        for i in 0..n {
            for j in 0..m {
                if data[i][j][0] && data[i][j][1] {
                    result.push(vec![i as i32, j as i32]);
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
    fn test_417() {
        let heights = nd_vec![
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4]
        ];
        let expected = nd_vec![[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]];
        assert_eq!(Solution::pacific_atlantic(heights), expected);
        let heights = nd_vec![[1]];
        let expected = nd_vec![[0, 0]];
        assert_eq!(Solution::pacific_atlantic(heights), expected);
    }
}
