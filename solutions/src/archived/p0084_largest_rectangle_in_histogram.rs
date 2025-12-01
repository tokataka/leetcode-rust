///
/// # 84. Largest Rectangle in Histogram
///
/// Given an array of integers `heights` representing the histogram's bar height where the width of each bar is `1`, return *the area of the largest rectangle in the histogram*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/04/histogram.jpg)
///
/// ```
/// Input: heights = [2,1,5,6,2,3]
/// Output: 10
/// Explanation: The above is a histogram where width of each bar is 1.
/// The largest rectangle is shown in the red area, which has an area = 10 units.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/04/histogram-1.jpg)
///
/// ```
/// Input: heights = [2,4]
/// Output: 4
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= heights.length <= 10<sup>5</sup>`
/// * `0 <= heights[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-rectangle-in-histogram/
// discuss: https://leetcode.com/problems/largest-rectangle-in-histogram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut monotonic = vec![];

        for height in heights.into_iter().chain(std::iter::once(0)) {
            let mut width = 0;

            while let Some(&(last_height, last_width)) = monotonic.last() {
                if height > last_height {
                    break;
                }

                monotonic.pop();
                width += last_width;
                result = result.max(last_height * width);
            }

            monotonic.push((height, width + 1));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_84() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        let expected = 10;
        assert_eq!(Solution::largest_rectangle_area(heights), expected);
        let heights = vec![2, 4];
        let expected = 4;
        assert_eq!(Solution::largest_rectangle_area(heights), expected);
    }
}
