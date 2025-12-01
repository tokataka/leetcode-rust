///
/// # 85. Maximal Rectangle
///
/// Given a `rows x cols` binary `matrix` filled with `0`'s and `1`'s, find the largest rectangle containing only `1`'s and return *its area*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/09/14/maximal.jpg)
///
/// ```
/// Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
/// Output: 6
/// Explanation: The maximal rectangle is shown in the above picture.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: matrix = [["0"]]
/// Output: 0
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: matrix = [["1"]]
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `rows == matrix.length`
/// * `cols == matrix[i].length`
/// * `1 <= rows, cols <= 200`
/// * `matrix[i][j]` is `'0'` or `'1'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-rectangle/
// discuss: https://leetcode.com/problems/maximal-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut widths = vec![vec![0; m]; n + 1];

        for (i, row) in matrix.iter().enumerate() {
            let mut cur = 0;

            for (j, &el) in row.iter().enumerate() {
                if el == '1' {
                    cur += 1;
                    widths[i][j] = cur;
                } else {
                    cur = 0;
                }
            }
        }

        let mut result = 0;

        for j in 0..m {
            let mut monotonic = vec![];

            for i in 0..=n {
                let width = widths[i][j];
                let mut height = 0;

                while let Some(&(last_width, last_height)) = monotonic.last() {
                    if width > last_width {
                        break;
                    }

                    monotonic.pop();
                    height += last_height;
                    result = result.max(last_width * height);
                }

                monotonic.push((width, height + 1));
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
    fn test_85() {
        let matrix = nd_vec![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ];
        let expected = 6;
        assert_eq!(Solution::maximal_rectangle(matrix), expected);
        let matrix = nd_vec![['0']];
        let expected = 0;
        assert_eq!(Solution::maximal_rectangle(matrix), expected);
        let matrix = nd_vec![['1']];
        let expected = 1;
        assert_eq!(Solution::maximal_rectangle(matrix), expected);
    }
}
