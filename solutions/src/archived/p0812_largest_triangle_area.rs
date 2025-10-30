///
/// # 812. Largest Triangle Area
///
/// Given an array of points on the **X-Y** plane `points` where `points[i] = [x<sub>i</sub>, y<sub>i</sub>]`, return *the area of the largest triangle that can be formed by any three different points*. Answers within `10<sup>-5</sup>` of the actual answer will be accepted.
///
/// **Example 1:**
///
/// ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/04/1027.png)
///
/// ```
/// Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
/// Output: 2.00000
/// Explanation: The five points are shown in the above figure. The red triangle is the largest.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: points = [[1,0],[0,0],[0,1]]
/// Output: 0.50000
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= points.length <= 50`
/// * `-50 <= x<sub>i</sub>, y<sub>i</sub> <= 50`
/// * All the given points are **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-triangle-area/
// discuss: https://leetcode.com/problems/largest-triangle-area/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut result = 0f64;

        for p in points.iter().combinations(3) {
            let (xi, yi) = (p[0][0], p[0][1]);
            let (xj, yj) = (p[1][0], p[1][1]);
            let (xk, yk) = (p[2][0], p[2][1]);

            result = result.max(
                (xi * yj + xj * yk + xk * yi - yi * xj - yj * xk - yk * xi).abs() as f64 / 2.0,
            );
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_812() {
        let points = nd_vec![[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]];
        let expected = 2.00000;
        assert_eq!(Solution::largest_triangle_area(points), expected);
        let points = nd_vec![[1, 0], [0, 0], [0, 1]];
        let expected = 0.50000;
        assert_eq!(Solution::largest_triangle_area(points), expected);
    }
}
