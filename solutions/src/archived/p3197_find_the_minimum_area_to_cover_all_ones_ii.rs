///
/// # 3197. Find the Minimum Area to Cover All Ones II
///
/// You are given a 2D **binary** array `grid`. You need to find 3 **non-overlapping** rectangles having **non-zero** areas with horizontal and vertical sides such that all the 1's in `grid` lie inside these rectangles.
///
/// Return the **minimum** possible sum of the area of these rectangles.
///
/// **Note** that the rectangles are allowed to touch.
///
/// **Example 1:**
///
/// **Input:** grid = [[1,0,1],[1,1,1]]
///
/// **Output:** 5
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/05/14/example0rect21.png)
///
/// * The 1's at `(0, 0)` and `(1, 0)` are covered by a rectangle of area 2.
/// * The 1's at `(0, 2)` and `(1, 2)` are covered by a rectangle of area 2.
/// * The 1 at `(1, 1)` is covered by a rectangle of area 1.
///
/// **Example 2:**
///
/// **Input:** grid = [[1,0,1,0],[0,1,0,1]]
///
/// **Output:** 5
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/05/14/example1rect2.png)
///
/// * The 1's at `(0, 0)` and `(0, 2)` are covered by a rectangle of area 3.
/// * The 1 at `(1, 1)` is covered by a rectangle of area 1.
/// * The 1 at `(1, 3)` is covered by a rectangle of area 1.
///
/// **Constraints:**
///
/// * `1 <= grid.length, grid[i].length <= 30`
/// * `grid[i][j]` is either 0 or 1.
/// * The input is generated such that there are at least three 1's in `grid`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii/
// discuss: https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        fn _minimum_sum(
            i1: usize,
            i2: usize,
            j1: usize,
            j2: usize,
            split_remain: i32,
            grid: &Vec<Vec<i32>>,
            cache: &mut HashMap<(usize, usize, usize, usize, i32), usize>,
        ) -> usize {
            let n = grid.len();
            let m = grid[0].len();

            if let Some(&x) = cache.get(&(i1, i2, j1, j2, split_remain)) {
                return x;
            }

            if split_remain == 0 {
                let (mut i_min, mut i_max, mut j_min, mut j_max) = (n - 1, 0, m - 1, 0);
                let mut has_one = false;

                for i in i1..i2 {
                    for j in j1..j2 {
                        if grid[i][j] == 1 {
                            i_min = i_min.min(i);
                            i_max = i_max.max(i);
                            j_min = j_min.min(j);
                            j_max = j_max.max(j);
                            has_one = true;
                        }
                    }
                }

                let result = match has_one {
                    false => n * m,
                    true => (i_max + 1 - i_min) * (j_max + 1 - j_min),
                };

                cache.insert((i1, i2, j1, j2, split_remain), result);

                return result;
            }

            let mut result = n * m;

            for split_i in i1 + 1..i2 {
                result = result.min(
                    _minimum_sum(i1, split_i, j1, j2, 0, grid, cache)
                        + _minimum_sum(split_i, i2, j1, j2, split_remain - 1, grid, cache),
                );
                result = result.min(
                    _minimum_sum(i1, split_i, j1, j2, split_remain - 1, grid, cache)
                        + _minimum_sum(split_i, i2, j1, j2, 0, grid, cache),
                );
            }

            for split_j in j1 + 1..j2 {
                result = result.min(
                    _minimum_sum(i1, i2, j1, split_j, 0, grid, cache)
                        + _minimum_sum(i1, i2, split_j, j2, split_remain - 1, grid, cache),
                );
                result = result.min(
                    _minimum_sum(i1, i2, j1, split_j, split_remain - 1, grid, cache)
                        + _minimum_sum(i1, i2, split_j, j2, 0, grid, cache),
                );
            }

            cache.insert((i1, i2, j1, j2, split_remain), result);

            result
        }

        _minimum_sum(
            0,
            grid.len(),
            0,
            grid[0].len(),
            2,
            &grid,
            &mut HashMap::new(),
        ) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3197() {
        let grid = nd_vec![[1, 0, 1], [1, 1, 1]];
        let expected = 5;
        assert_eq!(Solution::minimum_sum(grid), expected);
        let grid = nd_vec![[1, 0, 1, 0], [0, 1, 0, 1]];
        let expected = 5;
        assert_eq!(Solution::minimum_sum(grid), expected);
    }
}
