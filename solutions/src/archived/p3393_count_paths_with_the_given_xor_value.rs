///
/// # 3393. Count Paths With the Given XOR Value
///
/// You are given a 2D integer array `grid` with size `m x n`. You are also given an integer `k`.
///
/// Your task is to calculate the number of paths you can take from the top-left cell `(0, 0)` to the bottom-right cell `(m - 1, n - 1)` satisfying the following **constraints**:
///
/// * You can either move to the right or down. Formally, from the cell `(i, j)` you may move to the cell `(i, j + 1)` or to the cell `(i + 1, j)` if the target cell *exists*.
/// * The `XOR` of all the numbers on the path must be **equal** to `k`.
///
/// Return the total number of such paths.
///
/// Since the answer can be very large, return the result **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// **Input:** grid = [[2, 1, 5], [7, 10, 0], [12, 6, 4]], k = 11
///
/// **Output:** 3
///
/// **Explanation:**
///
/// The 3 paths are:
///
/// * `(0, 0) → (1, 0) → (2, 0) → (2, 1) → (2, 2)`
/// * `(0, 0) → (1, 0) → (1, 1) → (1, 2) → (2, 2)`
/// * `(0, 0) → (0, 1) → (1, 1) → (2, 1) → (2, 2)`
///
/// **Example 2:**
///
/// **Input:** grid = [[1, 3, 3, 3], [0, 3, 3, 2], [3, 0, 1, 1]], k = 2
///
/// **Output:** 5
///
/// **Explanation:**
///
/// The 5 paths are:
///
/// * `(0, 0) → (1, 0) → (2, 0) → (2, 1) → (2, 2) → (2, 3)`
/// * `(0, 0) → (1, 0) → (1, 1) → (2, 1) → (2, 2) → (2, 3)`
/// * `(0, 0) → (1, 0) → (1, 1) → (1, 2) → (1, 3) → (2, 3)`
/// * `(0, 0) → (0, 1) → (1, 1) → (1, 2) → (2, 2) → (2, 3)`
/// * `(0, 0) → (0, 1) → (0, 2) → (1, 2) → (2, 2) → (2, 3)`
///
/// **Example 3:**
///
/// **Input:** grid = [[1, 1, 1, 2], [3, 0, 3, 2], [3, 0, 2, 2]], k = 10
///
/// **Output:** 0
///
/// **Constraints:**
///
/// * `1 <= m == grid.length <= 300`
/// * `1 <= n == grid[r].length <= 300`
/// * `0 <= grid[r][c] < 16`
/// * `0 <= k < 16`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-paths-with-the-given-xor-value/
// discuss: https://leetcode.com/problems/count-paths-with-the-given-xor-value/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MOD: i32 = 10_i32.pow(9) + 7;
        let height = grid.len();
        let width = grid[0].len();

        let mut cache = vec![vec![vec![0; 16]; width + 1]; height + 1];
        cache[1][1][grid[0][0] as usize] = 1;

        for i in 1..=height {
            for j in 1..=width {
                let cell = grid[i - 1][j - 1] as usize;

                for prev_cell in 0..16 {
                    cache[i][j][prev_cell ^ cell] = (cache[i][j][prev_cell ^ cell]
                        + cache[i - 1][j][prev_cell]
                        + cache[i][j - 1][prev_cell])
                        % MOD;
                }
            }
        }

        cache[height][width][k as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3393() {
        let grid = nd_vec![[2, 1, 5], [7, 10, 0], [12, 6, 4]];
        let k = 11;
        let expected = 3;
        assert_eq!(Solution::count_paths_with_xor_value(grid, k), expected);
        let grid = nd_vec![[1, 3, 3, 3], [0, 3, 3, 2], [3, 0, 1, 1]];
        let k = 2;
        let expected = 5;
        assert_eq!(Solution::count_paths_with_xor_value(grid, k), expected);
        let grid = nd_vec![[1, 1, 1, 2], [3, 0, 3, 2], [3, 0, 2, 2]];
        let k = 10;
        let expected = 0;
        assert_eq!(Solution::count_paths_with_xor_value(grid, k), expected);
    }
}
