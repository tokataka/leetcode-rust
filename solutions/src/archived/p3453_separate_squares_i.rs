///
/// # 3453. Separate Squares I
///
/// You are given a 2D integer array `squares`. Each `squares[i] = [x<sub>i</sub>, y<sub>i</sub>, l<sub>i</sub>]` represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
///
/// Find the **minimum** y-coordinate value of a horizontal line such that the total area of the squares above the line *equals* the total area of the squares below the line.
///
/// Answers within `10<sup>-5</sup>` of the actual answer will be accepted.
///
/// **Note**: Squares **may** overlap. Overlapping areas should be counted **multiple times**.
///
/// **Example 1:**
///
/// **Input:** squares = [[0,0,1],[2,2,1]]
///
/// **Output:** 1.00000
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/01/06/4062example1drawio.png)
///
/// Any horizontal line between `y = 1` and `y = 2` will have 1 square unit above it and 1 square unit below it. The lowest option is 1.
///
/// **Example 2:**
///
/// **Input:** squares = [[0,0,2],[1,1,1]]
///
/// **Output:** 1.16667
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/01/15/4062example2drawio.png)
///
/// The areas are:
///
/// * Below the line: `7/6 * 2 (Red) + 1/6 (Blue) = 15/6 = 2.5`.
/// * Above the line: `5/6 * 2 (Red) + 5/6 (Blue) = 15/6 = 2.5`.
///
/// Since the areas above and below the line are equal, the output is `7/6 = 1.16667`.
///
/// **Constraints:**
///
/// * `1 <= squares.length <= 5 * 10<sup>4</sup>`
/// * `squares[i] = [x<sub>i</sub>, y<sub>i</sub>, l<sub>i</sub>]`
/// * `squares[i].length == 3`
/// * `0 <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>9</sup>`
/// * `1 <= l<sub>i</sub> <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/separate-squares-i/
// discuss: https://leetcode.com/problems/separate-squares-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let (min, max) = squares.iter().fold((i32::MAX, 0), |(min, max), x| {
            (min.min(x[1]), max.max(x[1] + x[2]))
        });

        let mut result = (max + min) as f64 / 2.;
        let mut d = (max - min) as f64 / 4.;

        for _ in 0..50 {
            let mut lower = 0.;
            let mut upper = 0.;

            for square in &squares {
                let (_, y, l) = (square[0] as f64, square[1] as f64, square[2] as f64);

                if y + l <= result {
                    lower += l * l;
                } else if y >= result {
                    upper += l * l;
                } else {
                    lower += (result - y) * l;
                    upper += (y + l - result) * l;
                }
            }

            if upper > lower {
                result += d;
            } else {
                result -= d;
            }

            d /= 2.;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3453() {
        let squares = nd_vec![[0, 0, 1], [2, 2, 1]];
        let expected = 1.00000;
        assert_eq!(Solution::separate_squares(squares), expected);
        let squares = nd_vec![[0, 0, 2], [1, 1, 1]];
        let expected = 1.16667;
        assert_eq!(Solution::separate_squares(squares), expected);
    }
}
