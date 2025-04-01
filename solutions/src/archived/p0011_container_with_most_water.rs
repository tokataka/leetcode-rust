///
/// # 11. Container With Most Water
///
/// You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the `i<sup>th</sup>` line are `(i, 0)` and `(i, height[i])`.
///
/// Find two lines that together with the x-axis form a container, such that the container contains the most water.
///
/// Return *the maximum amount of water a container can store*.
///
/// **Notice** that you may not slant the container.
///
/// **Example 1:**
///
/// ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg)
///
/// ```
/// Input: height = [1,8,6,2,5,4,8,3,7]
/// Output: 49
/// Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: height = [1,1]
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `n == height.length`
/// * `2 <= n <= 10<sup>5</sup>`
/// * `0 <= height[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/container-with-most-water/
// discuss: https://leetcode.com/problems/container-with-most-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut result = 0;

        while left < right {
            result = result.max((right - left) as i32 * height[right].min(height[left]));

            match height[left] < height[right] {
                true => left += 1,
                false => right -= 1,
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
    fn test_11() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;
        assert_eq!(Solution::max_area(height), expected);
        let height = vec![1, 1];
        let expected = 1;
        assert_eq!(Solution::max_area(height), expected);
    }
}
