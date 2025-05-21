///
/// # 3446. Sort Matrix by Diagonals
///
/// You are given an `n x n` square matrix of integers `grid`. Return the matrix such that:
///
/// * The diagonals in the **bottom-left triangle** (including the middle diagonal) are sorted in **non-increasing order**.
/// * The diagonals in the **top-right triangle** are sorted in **non-decreasing order**.
///
/// **Example 1:**
///
/// **Input:** grid = [[1,7,3],[9,8,2],[4,5,6]]
///
/// **Output:** [[8,2,3],[9,6,7],[4,5,1]]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/12/29/4052example1drawio.png)
///
/// The diagonals with a black arrow (bottom-left triangle) should be sorted in non-increasing order:
///
/// * `[1, 8, 6]` becomes `[8, 6, 1]`.
/// * `[9, 5]` and `[4]` remain unchanged.
///
/// The diagonals with a blue arrow (top-right triangle) should be sorted in non-decreasing order:
///
/// * `[7, 2]` becomes `[2, 7]`.
/// * `[3]` remains unchanged.
///
/// **Example 2:**
///
/// **Input:** grid = [[0,1],[1,2]]
///
/// **Output:** [[2,1],[1,0]]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/12/29/4052example2adrawio.png)
///
/// The diagonals with a black arrow must be non-increasing, so `[0, 2]` is changed to `[2, 0]`. The other diagonals are already in the correct order.
///
/// **Example 3:**
///
/// **Input:** grid = [[1]]
///
/// **Output:** [[1]]
///
/// **Explanation:**
///
/// Diagonals with exactly one element are already in order, so no changes are needed.
///
/// **Constraints:**
///
/// * `grid.length == grid[i].length == n`
/// * `1 <= n <= 10`
/// * `-10<sup>5</sup> <= grid[i][j] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-matrix-by-diagonals/
// discuss: https://leetcode.com/problems/sort-matrix-by-diagonals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();

        for di in 0..n {
            let mut diagonal = vec![];

            for d in 0..n - di {
                diagonal.push(grid[di + d][d]);
            }

            diagonal.sort_by(|x, y| y.cmp(x));

            for d in 0..n - di {
                grid[di + d][d] = diagonal[d];
            }
        }

        for dj in 1..n {
            let mut diagonal = vec![];

            for d in 0..n - dj {
                diagonal.push(grid[d][dj + d]);
            }

            diagonal.sort();

            for d in 0..n - dj {
                grid[d][dj + d] = diagonal[d];
            }
        }

        grid
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3446() {
        let grid = nd_vec![[1, 7, 3], [9, 8, 2], [4, 5, 6]];
        let expected = nd_vec![[8, 2, 3], [9, 6, 7], [4, 5, 1]];
        assert_eq!(Solution::sort_matrix(grid), expected);
        let grid = nd_vec![[0, 1], [1, 2]];
        let expected = nd_vec![[2, 1], [1, 0]];
        assert_eq!(Solution::sort_matrix(grid), expected);
        let grid = nd_vec![[1]];
        let expected = nd_vec![[1]];
        assert_eq!(Solution::sort_matrix(grid), expected);
    }
}
