///
/// # 1504. Count Submatrices With All Ones
///
/// Given an `m x n` binary matrix `mat`, *return the number of **submatrices** that have all ones*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/10/27/ones1-grid.jpg)
///
/// ```
/// Input: mat = [[1,0,1],[1,1,0],[1,1,0]]
/// Output: 13
/// Explanation:
/// There are 6 rectangles of side 1x1.
/// There are 2 rectangles of side 1x2.
/// There are 3 rectangles of side 2x1.
/// There is 1 rectangle of side 2x2.
/// There is 1 rectangle of side 3x1.
/// Total number of rectangles = 6 + 2 + 3 + 1 + 1 = 13.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/10/27/ones2-grid.jpg)
///
/// ```
/// Input: mat = [[0,1,1,0],[0,1,1,1],[1,1,1,0]]
/// Output: 24
/// Explanation:
/// There are 8 rectangles of side 1x1.
/// There are 5 rectangles of side 1x2.
/// There are 2 rectangles of side 1x3.
/// There are 4 rectangles of side 2x1.
/// There are 2 rectangles of side 2x2.
/// There are 2 rectangles of side 3x1.
/// There is 1 rectangle of side 3x2.
/// Total number of rectangles = 8 + 5 + 2 + 4 + 2 + 2 + 1 = 24.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= m, n <= 150`
/// * `mat[i][j]` is either `0` or `1`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-submatrices-with-all-ones/
// discuss: https://leetcode.com/problems/count-submatrices-with-all-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn num_submat(mut mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();

        for i in 0..n {
            for j in 1..m {
                if mat[i][j] == 1 {
                    mat[i][j] += mat[i][j - 1];
                }
            }
        }

        let mut result = 0;
        let mut monotonic = Vec::with_capacity(n);

        for j in 0..m {
            monotonic.clear();

            for i in 0..n {
                if mat[i][j] == 0 {
                    monotonic.clear();
                    continue;
                }

                let cur_width = mat[i][j];
                let mut cur_i = i;
                let mut cur_count = 0;

                while let Some(&(last_width, last_i, last_count)) = monotonic.last() {
                    if last_width < cur_width {
                        cur_count += last_count;
                        break;
                    }

                    cur_i = last_i;
                    monotonic.pop();
                }

                cur_count += cur_width * (i + 1 - cur_i) as i32;
                monotonic.push((cur_width, cur_i, cur_count));
                result += cur_count;
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
    fn test_1504() {
        // let mat = nd_vec![[1, 0, 1], [1, 1, 0], [1, 1, 0]];
        // let expected = 13;
        // assert_eq!(Solution::num_submat(mat), expected);
        // let mat = nd_vec![[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]];
        // let expected = 24;
        // assert_eq!(Solution::num_submat(mat), expected);
        let mat = nd_vec![[1, 0, 1], [0, 1, 0], [1, 0, 1]];
        let expected = 5;
        assert_eq!(Solution::num_submat(mat), expected);
    }
}
