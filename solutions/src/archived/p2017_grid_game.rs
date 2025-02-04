///
/// # 2017. Grid Game
///
/// You are given a **0-indexed** 2D array `grid` of size `2 x n`, where `grid[r][c]` represents the number of points at position `(r, c)` on the matrix. Two robots are playing a game on this matrix.
///
/// Both robots initially start at `(0, 0)` and want to reach `(1, n-1)`. Each robot may only move to the **right** (`(r, c)` to `(r, c + 1)`) or **down** (`(r, c)` to `(r + 1, c)`).
///
/// At the start of the game, the **first** robot moves from `(0, 0)` to `(1, n-1)`, collecting all the points from the cells on its path. For all cells `(r, c)` traversed on the path, `grid[r][c]` is set to `0`. Then, the **second** robot moves from `(0, 0)` to `(1, n-1)`, collecting the points on its path. Note that their paths may intersect with one another.
///
/// The **first** robot wants to **minimize** the number of points collected by the **second** robot. In contrast, the **second** robot wants to **maximize** the number of points it collects. If both robots play **optimally**, return *the **number of points** collected by the **second** robot.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/09/08/a1.png)
///
/// ```
/// Input: grid = [[2,5,4],[1,5,1]]
/// Output: 4
/// Explanation: The optimal path taken by the first robot is shown in red, and the optimal path taken by the second robot is shown in blue.
/// The cells visited by the first robot are set to 0.
/// The second robot will collect 0 + 0 + 4 + 0 = 4 points.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/09/08/a2.png)
///
/// ```
/// Input: grid = [[3,3,1],[8,5,2]]
/// Output: 4
/// Explanation: The optimal path taken by the first robot is shown in red, and the optimal path taken by the second robot is shown in blue.
/// The cells visited by the first robot are set to 0.
/// The second robot will collect 0 + 3 + 1 + 0 = 4 points.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/09/08/a3.png)
///
/// ```
/// Input: grid = [[1,3,1,15],[1,3,3,1]]
/// Output: 7
/// Explanation: The optimal path taken by the first robot is shown in red, and the optimal path taken by the second robot is shown in blue.
/// The cells visited by the first robot are set to 0.
/// The second robot will collect 0 + 1 + 3 + 3 + 0 = 7 points.
///
/// ```
///
/// **Constraints:**
///
/// * `grid.length == 2`
/// * `n == grid[r].length`
/// * `1 <= n <= 5 * 10<sup>4</sup>`
/// * `1 <= grid[r][c] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/grid-game/
// discuss: https://leetcode.com/problems/grid-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut right = grid[0]
            .get(1..)
            .unwrap_or_default()
            .iter()
            .map(|&x| x as i64)
            .sum();
        let mut left = 0;

        let mut result = left.max(right);

        for j in 1..grid[0].len() {
            right -= grid[0][j] as i64;
            left += grid[1][j - 1] as i64;

            let maybe_result = left.max(right);

            if maybe_result > result {
                return result;
            }

            result = maybe_result;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2017() {
        let grid = nd_vec![[2, 5, 4], [1, 5, 1]];
        let expected = 4;
        assert_eq!(Solution::grid_game(grid), expected);
        let grid = nd_vec![[3, 3, 1], [8, 5, 2]];
        let expected = 4;
        assert_eq!(Solution::grid_game(grid), expected);
        let grid = nd_vec![[1, 3, 1, 15], [1, 3, 3, 1]];
        let expected = 7;
        assert_eq!(Solution::grid_game(grid), expected);
    }
}
