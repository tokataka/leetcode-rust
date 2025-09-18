///
/// # 1277. Count Square Submatrices with All Ones
///
/// Given a `m * n` matrix of ones and zeros, return how many **square** submatrices have all ones.
///
/// **Example 1:**
///
/// ```
/// Input: matrix =
/// [
///   [0,1,1,1],
///   [1,1,1,1],
///   [0,1,1,1]
/// ]
/// Output: 15
/// Explanation:
/// There are 10 squares of side 1.
/// There are 4 squares of side 2.
/// There is  1 square of side 3.
/// Total number of squares = 10 + 4 + 1 = 15.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: matrix =
/// [
///   [1,0,1],
///   [1,1,0],
///   [1,1,0]
/// ]
/// Output: 7
/// Explanation:
/// There are 6 squares of side 1.
/// There is 1 square of side 2.
/// Total number of squares = 6 + 1 = 7.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= arr.length <= 300`
/// * `1 <= arr[0].length <= 300`
/// * `0 <= arr[i][j] <= 1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-square-submatrices-with-all-ones/
// discuss: https://leetcode.com/problems/count-square-submatrices-with-all-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut result = 0;

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 1 && i > 0 && j > 0 {
                    matrix[i][j] += matrix[i - 1][j]
                        .min(matrix[i - 1][j - 1])
                        .min(matrix[i][j - 1]);
                }

                result += matrix[i][j];
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
    fn test_1277() {
        let matrix = nd_vec![[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]];
        let expected = 15;
        assert_eq!(Solution::count_squares(matrix), expected);
        let matrix = nd_vec![[1, 0, 1], [1, 1, 0], [1, 1, 0]];
        let expected = 7;
        assert_eq!(Solution::count_squares(matrix), expected);
    }
}
