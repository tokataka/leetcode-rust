///
/// # 221. Maximal Square
///
/// Given an `m x n` binary `matrix` filled with `0`'s and `1`'s, *find the largest square containing only* `1`'s *and return its area*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/11/26/max1grid.jpg)
///
/// ```
/// Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
/// Output: 4
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/11/26/max2grid.jpg)
///
/// ```
/// Input: matrix = [["0","1"],["1","0"]]
/// Output: 1
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: matrix = [["0"]]
/// Output: 0
///
/// ```
///
/// **Constraints:**
///
/// * `m == matrix.length`
/// * `n == matrix[i].length`
/// * `1 <= m, n <= 300`
/// * `matrix[i][j]` is `'0'` or `'1'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-square/
// discuss: https://leetcode.com/problems/maximal-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = vec![vec![0; m + 1]; n + 1];

        let mut result = 0;

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if matrix[i][j] == '1' {
                    dp[i][j] = 1 + dp[i + 1][j].min(dp[i][j + 1].min(dp[i + 1][j + 1]));
                }
                result = result.max(dp[i][j]);
            }
        }

        result * result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_221() {
        let matrix = nd_vec![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ];
        let expected = 4;
        assert_eq!(Solution::maximal_square(matrix), expected);
        let matrix = nd_vec![['0', '1'], ['1', '0']];
        let expected = 1;
        assert_eq!(Solution::maximal_square(matrix), expected);
        let matrix = nd_vec![['0']];
        let expected = 0;
        assert_eq!(Solution::maximal_square(matrix), expected);
    }
}
