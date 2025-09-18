///
/// # 120. Triangle
///
/// Given a `triangle` array, return *the minimum path sum from top to bottom*.
///
/// For each step, you may move to an adjacent number of the row below. More formally, if you are on index `i` on the current row, you may move to either index `i` or index `i + 1` on the next row.
///
/// **Example 1:**
///
/// ```
/// Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
/// Output: 11
/// Explanation: The triangle looks like:
///    2
///   3 4
///  6 5 7
/// 4 1 8 3
/// The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: triangle = [[-10]]
/// Output: -10
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= triangle.length <= 200`
/// * `triangle[0].length == 1`
/// * `triangle[i].length == triangle[i - 1].length + 1`
/// * `-10<sup>4</sup> <= triangle[i][j] <= 10<sup>4</sup>`
///
/// **Follow up:** Could you do this using only `O(n)` extra space, where `n` is the total number of rows in the triangle?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/triangle/
// discuss: https://leetcode.com/problems/triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();

        for r in (1..n).rev() {
            for i in 0..r {
                triangle[r - 1][i] += triangle[r][i].min(triangle[r][i + 1]);
            }
        }

        triangle[0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_120() {
        let triangle = nd_vec![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]];
        let expected = 11;
        assert_eq!(Solution::minimum_total(triangle), expected);
        let triangle = nd_vec![[-10]];
        let expected = -10;
        assert_eq!(Solution::minimum_total(triangle), expected);
    }
}
