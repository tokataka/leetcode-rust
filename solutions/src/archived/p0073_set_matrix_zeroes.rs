///
/// # 73. Set Matrix Zeroes
///
/// Given an `m x n` integer matrix `matrix`, if an element is `0`, set its entire row and column to `0`'s.
///
/// You must do it [in place](https://en.wikipedia.org/wiki/In-place_algorithm).
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/08/17/mat1.jpg)
///
/// ```
/// Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
/// Output: [[1,0,1],[0,0,0],[1,0,1]]
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/08/17/mat2.jpg)
///
/// ```
/// Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
/// Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
///
/// ```
///
/// **Constraints:**
///
/// * `m == matrix.length`
/// * `n == matrix[0].length`
/// * `1 <= m, n <= 200`
/// * `-2<sup>31</sup> <= matrix[i][j] <= 2<sup>31</sup> - 1`
///
/// **Follow up:**
///
/// * A straightforward solution using `O(mn)` space is probably a bad idea.
/// * A simple improvement uses `O(m + n)` space, but still not the best solution.
/// * Could you devise a constant space solution?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/set-matrix-zeroes/
// discuss: https://leetcode.com/problems/set-matrix-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();

        let first_row_has_zero = matrix[0].iter().any(|&cell| cell == 0);
        let first_col_has_zero = matrix.iter().any(|row| row[0] == 0);

        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..n {
            if matrix[i][0] == 0 {
                for j in 1..m {
                    matrix[i][j] = 0;
                }
            }
        }

        for j in 1..m {
            if matrix[0][j] == 0 {
                for i in 1..n {
                    matrix[i][j] = 0;
                }
            }
        }

        if first_row_has_zero {
            for j in 0..m {
                matrix[0][j] = 0;
            }
        }

        if first_col_has_zero {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_73() {
        let mut matrix = nd_vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        let expected = nd_vec![[1, 0, 1], [0, 0, 0], [1, 0, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
        let mut matrix = nd_vec![[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]];
        let expected = nd_vec![[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
