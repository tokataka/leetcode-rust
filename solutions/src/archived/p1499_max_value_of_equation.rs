///
/// # 1499. Max Value of Equation
///
/// You are given an array `points` containing the coordinates of points on a 2D plane, sorted by the x-values, where `points[i] = [x<sub>i</sub>, y<sub>i</sub>]` such that `x<sub>i</sub> < x<sub>j</sub>` for all `1 <= i < j <= points.length`. You are also given an integer `k`.
///
/// Return *the maximum value of the equation* `y<sub>i</sub> + y<sub>j</sub> + |x<sub>i</sub> - x<sub>j</sub>|` where `|x<sub>i</sub> - x<sub>j</sub>| <= k` and `1 <= i < j <= points.length`.
///
/// It is guaranteed that there exists at least one pair of points that satisfy the constraint `|x<sub>i</sub> - x<sub>j</sub>| <= k`.
///
/// **Example 1:**
///
/// ```
/// Input: points = [[1,3],[2,0],[5,10],[6,-10]], k = 1
/// Output: 4
/// Explanation: The first two points satisfy the condition |xi - xj| <= 1 and if we calculate the equation we get 3 + 0 + |1 - 2| = 4. Third and fourth points also satisfy the condition and give a value of 10 + -10 + |5 - 6| = 1.
/// No other pairs satisfy the condition, so we return the max of 4 and 1.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: points = [[0,0],[3,0],[9,2]], k = 3
/// Output: 3
/// Explanation: Only the first two points have an absolute difference of 3 or less in the x-values, and give the value of 0 + 0 + |0 - 3| = 3.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= points.length <= 10<sup>5</sup>`
/// * `points[i].length == 2`
/// * `-10<sup>8</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>8</sup>`
/// * `0 <= k <= 2 * 10<sup>8</sup>`
/// * `x<sub>i</sub> < x<sub>j</sub>` for all `1 <= i < j <= points.length`
/// * `x<sub>i</sub>` form a strictly increasing sequence.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/max-value-of-equation/
// discuss: https://leetcode.com/problems/max-value-of-equation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut result = i32::MIN;

        let mut q: VecDeque<(i32, i32)> = VecDeque::with_capacity(points.len());

        for right in &points {
            let (x, y) = (right[0], right[1]);

            while let Some(&(q_x, q_val)) = q.front() {
                if q_x >= x - k {
                    result = result.max(y + x + q_val);
                    break;
                }

                q.pop_front();
            }

            while let Some(&(_, q_val)) = q.back() {
                if q_val > y - x {
                    break;
                }

                q.pop_back();
            }

            q.push_back((x, y - x));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1499() {
        let points = nd_vec![[1, 3], [2, 0], [5, 10], [6, -10]];
        let k = 1;
        let expected = 4;
        assert_eq!(Solution::find_max_value_of_equation(points, k), expected);
        let points = nd_vec![[0, 0], [3, 0], [9, 2]];
        let k = 3;
        let expected = 3;
        assert_eq!(Solution::find_max_value_of_equation(points, k), expected);
    }
}
