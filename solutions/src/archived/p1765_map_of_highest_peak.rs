///
/// # 1765. Map of Highest Peak
///
/// You are given an integer matrix `isWater` of size `m x n` that represents a map of **land** and **water** cells.
///
/// * If `isWater[i][j] == 0`, cell `(i, j)` is a **land** cell.
/// * If `isWater[i][j] == 1`, cell `(i, j)` is a **water** cell.
///
/// You must assign each cell a height in a way that follows these rules:
///
/// * The height of each cell must be non-negative.
/// * If the cell is a **water** cell, its height must be `0`.
/// * Any two adjacent cells must have an absolute height difference of **at most** `1`. A cell is adjacent to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).
///
/// Find an assignment of heights such that the maximum height in the matrix is **maximized**.
///
/// Return *an integer matrix* `height` *of size* `m x n` *where* `height[i][j]` *is cell* `(i, j)`*'s height. If there are multiple solutions, return **any** of them*.
///
/// **Example 1:**
///
/// **![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-82045-am.png)**
///
/// ```
/// Input: isWater = [[0,1],[0,0]]
/// Output: [[1,0],[2,1]]
/// Explanation: The image shows the assigned heights of each cell.
/// The blue cell is the water cell, and the green cells are the land cells.
///
/// ```
///
/// **Example 2:**
///
/// **![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-82050-am.png)**
///
/// ```
/// Input: isWater = [[0,0,1],[1,0,0],[0,0,0]]
/// Output: [[1,1,0],[0,1,1],[1,2,2]]
/// Explanation: A height of 2 is the maximum possible height of any assignment.
/// Any height assignment that has a maximum height of 2 while still meeting the rules will also be accepted.
///
/// ```
///
/// **Constraints:**
///
/// * `m == isWater.length`
/// * `n == isWater[i].length`
/// * `1 <= m, n <= 1000`
/// * `isWater[i][j]` is `0` or `1`.
/// * There is at least **one** water cell.
///
/// **Note:** This question is the same as 542: [https://leetcode.com/problems/01-matrix/](https://leetcode.com/problems/01-matrix/description/)
///
pub struct Solution {}

// problem: https://leetcode.com/problems/map-of-highest-peak/
// discuss: https://leetcode.com/problems/map-of-highest-peak/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let i_size = is_water.len();
        let j_size = is_water[0].len();

        let mut result = vec![vec![0; j_size]; i_size];

        let mut st = vec![];
        let mut visited = vec![vec![false; j_size]; i_size];

        for (i, row) in is_water.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x == 1 {
                    st.push((i, j));
                    visited[i][j] = true;
                }
            }
        }

        const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for cur_height in 0.. {
            if st.is_empty() {
                break;
            }

            let mut next_st = vec![];

            for (i, j) in st {
                result[i][j] = cur_height;

                for (di, dj) in DIRECTIONS {
                    let (ci, cj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                        (Some(ci), Some(cj)) if ci < i_size && cj < j_size => (ci, cj),
                        _ => continue,
                    };

                    if !visited[ci][cj] {
                        next_st.push((ci, cj));
                        visited[ci][cj] = true;
                    }
                }
            }

            st = next_st;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1765() {
        let is_water = nd_vec![[0, 1], [0, 0]];
        let expected = nd_vec![[1, 0], [2, 1]];
        assert_eq!(Solution::highest_peak(is_water), expected);
        let is_water = nd_vec![[0, 0, 1], [1, 0, 0], [0, 0, 0]];
        let expected = nd_vec![[1, 1, 0], [0, 1, 1], [1, 2, 2]];
        assert_eq!(Solution::highest_peak(is_water), expected);
    }
}
