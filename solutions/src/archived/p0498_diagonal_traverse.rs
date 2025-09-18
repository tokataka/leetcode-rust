///
/// # 498. Diagonal Traverse
///
/// Given an `m x n` matrix `mat`, return *an array of all the elements of the array in a diagonal order*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/10/diag1-grid.jpg)
///
/// ```
/// Input: mat = [[1,2,3],[4,5,6],[7,8,9]]
/// Output: [1,2,4,7,5,3,6,8,9]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: mat = [[1,2],[3,4]]
/// Output: [1,2,3,4]
///
/// ```
///
/// **Constraints:**
///
/// * `m == mat.length`
/// * `n == mat[i].length`
/// * `1 <= m, n <= 10<sup>4</sup>`
/// * `1 <= m * n <= 10<sup>4</sup>`
/// * `-10<sup>5</sup> <= mat[i][j] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/diagonal-traverse/
// discuss: https://leetcode.com/problems/diagonal-traverse/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len();
        let m = mat[0].len();

        let mut result = Vec::with_capacity(n * m);

        for d in 0..n + m - 1 {
            let skip_i = d.saturating_sub(n - 1);
            let skip_j = d.saturating_sub(m - 1);

            if d % 2 == 0 {
                for j in skip_i..=d - skip_j {
                    result.push(mat[d - j][j]);
                }
            } else {
                for i in skip_j..=d - skip_i {
                    result.push(mat[i][d - i]);
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
    fn test_498() {
        let mat = nd_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let expected = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];
        assert_eq!(Solution::find_diagonal_order(mat), expected);
        let mat = nd_vec![[1, 2], [3, 4]];
        let expected = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_diagonal_order(mat), expected);
    }
}
