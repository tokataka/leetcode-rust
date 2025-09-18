///
/// # 3459. Length of Longest V-Shaped Diagonal Segment
///
/// You are given a 2D integer matrix `grid` of size `n x m`, where each element is either `0`, `1`, or `2`.
///
/// A **V-shaped diagonal segment** is defined as:
///
/// * The segment starts with `1`.
/// * The subsequent elements follow this infinite sequence: `2, 0, 2, 0, ...`.
/// * The segment:
///   * Starts **along** a diagonal direction (top-left to bottom-right, bottom-right to top-left, top-right to bottom-left, or bottom-left to top-right).
///   * Continues the **sequence** in the same diagonal direction.
///   * Makes **at most one clockwise 90-degree** **turn** to another diagonal direction while **maintaining** the sequence.
///
/// ![](https://assets.leetcode.com/uploads/2025/01/11/length_of_longest3.jpg)
///
/// Return the **length** of the **longest** **V-shaped diagonal segment**. If no valid segment *exists*, return 0.
///
/// **Example 1:**
///
/// **Input:** grid = [[2,2,1,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]
///
/// **Output:** 5
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/12/09/matrix_1-2.jpg)
///
/// The longest V-shaped diagonal segment has a length of 5 and follows these coordinates: `(0,2) → (1,3) → (2,4)`, takes a **90-degree clockwise turn** at `(2,4)`, and continues as `(3,3) → (4,2)`.
///
/// **Example 2:**
///
/// **Input:** grid = [[2,2,2,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]
///
/// **Output:** 4
///
/// **Explanation:**
///
/// **![](https://assets.leetcode.com/uploads/2024/12/09/matrix_2.jpg)**
///
/// The longest V-shaped diagonal segment has a length of 4 and follows these coordinates: `(2,3) → (3,2)`, takes a **90-degree clockwise turn** at `(3,2)`, and continues as `(2,1) → (1,0)`.
///
/// **Example 3:**
///
/// **Input:** grid = [[1,2,2,2,2],[2,2,2,2,0],[2,0,0,0,0],[0,0,2,2,2],[2,0,0,2,0]]
///
/// **Output:** 5
///
/// **Explanation:**
///
/// **![](https://assets.leetcode.com/uploads/2024/12/09/matrix_3.jpg)**
///
/// The longest V-shaped diagonal segment has a length of 5 and follows these coordinates: `(0,0) → (1,1) → (2,2) → (3,3) → (4,4)`.
///
/// **Example 4:**
///
/// **Input:** grid = [[1]]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// The longest V-shaped diagonal segment has a length of 1 and follows these coordinates: `(0,0)`.
///
/// **Constraints:**
///
/// * `n == grid.length`
/// * `m == grid[i].length`
/// * `1 <= n, m <= 500`
/// * `grid[i][j]` is either `0`, `1` or `2`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/length-of-longest-v-shaped-diagonal-segment/
// discuss: https://leetcode.com/problems/length-of-longest-v-shaped-diagonal-segment/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTIONS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

        let n = grid.len();
        let m = grid[0].len();

        fn _let_of_v_diagonal(
            i: usize,
            j: usize,
            d: usize,
            turned: bool,
            grid: &Vec<Vec<i32>>,
            cache: &mut Vec<Vec<Vec<i32>>>,
        ) -> (i32, i32) {
            if turned && cache[i][j][d] != i32::MAX {
                return (cache[i][j][d], cache[i][j][d]);
            }

            let n = grid.len();
            let m = grid[0].len();

            let mut max_v_shape = 1;
            let mut max_diagonal = 1;

            let next_needed = match grid[i][j] {
                2 => 0,
                _ => 2,
            };

            let (di, dj) = DIRECTIONS[d];

            if let (Some(next_i), Some(next_j)) =
                (i.checked_add_signed(di), j.checked_add_signed(dj))
            {
                if next_i < n && next_j < m && grid[next_i][next_j] == next_needed {
                    let (v_shape, diagonal) =
                        _let_of_v_diagonal(next_i, next_j, d, turned, grid, cache);

                    max_v_shape += v_shape;
                    max_diagonal += diagonal;
                }
            }

            cache[i][j][d] = max_diagonal;

            if !turned {
                let turn_d = (d + 1) % 4;

                max_v_shape = max_v_shape.max(match cache[i][j][turn_d] {
                    i32::MAX => _let_of_v_diagonal(i, j, turn_d, true, grid, cache).1,
                    x => x,
                });
            }

            (max_v_shape, max_diagonal)
        }

        let mut result = 0;
        let mut cache = vec![vec![vec![i32::MAX; 4]; m]; n];

        for (i, row) in grid.iter().enumerate() {
            for (j, &el) in row.iter().enumerate() {
                if el != 1 {
                    continue;
                }

                for d in 0..4 {
                    result = result.max(_let_of_v_diagonal(i, j, d, false, &grid, &mut cache).0);
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
    fn test_3459() {
        let grid = nd_vec![
            [2, 2, 1, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2]
        ];
        let expected = 5;
        assert_eq!(Solution::len_of_v_diagonal(grid), expected);
        let grid = nd_vec![
            [2, 2, 2, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2]
        ];
        let expected = 4;
        assert_eq!(Solution::len_of_v_diagonal(grid), expected);
        let grid = nd_vec![
            [1, 2, 2, 2, 2],
            [2, 2, 2, 2, 0],
            [2, 0, 0, 0, 0],
            [0, 0, 2, 2, 2],
            [2, 0, 0, 2, 0]
        ];
        let expected = 5;
        assert_eq!(Solution::len_of_v_diagonal(grid), expected);
        let grid = nd_vec![[1]];
        let expected = 1;
        assert_eq!(Solution::len_of_v_diagonal(grid), expected);
    }
}
