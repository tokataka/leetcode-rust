///
/// # 2865. Beautiful Towers I
///
/// You are given an array `heights` of `n` integers representing the number of bricks in `n` consecutive towers. Your task is to remove some bricks to form a **mountain-shaped** tower arrangement. In this arrangement, the tower heights are non-decreasing, reaching a maximum peak value with one or multiple consecutive towers and then non-increasing.
///
/// Return the **maximum possible sum** of heights of a mountain-shaped tower arrangement.
///
/// **Example 1:**
///
/// **Input:** heights = [5,3,4,1,1]
///
/// **Output:** 13
///
/// **Explanation:**
///
/// We remove some bricks to make `heights = [5,3,3,1,1]`, the peak is at index 0.
///
/// **Example 2:**
///
/// **Input:** heights = [6,5,3,9,2,7]
///
/// **Output:** 22
///
/// **Explanation:**
///
/// We remove some bricks to make `heights = [3,3,3,9,2,2]`, the peak is at index 3.
///
/// **Example 3:**
///
/// **Input:** heights = [3,2,5,5,2,3]
///
/// **Output:** 18
///
/// **Explanation:**
///
/// We remove some bricks to make `heights = [2,2,5,5,2,2]`, the peak is at index 2 or 3.
///
/// **Constraints:**
///
/// * `1 <= n == heights.length <= 10<sup>3</sup>`
/// * `1 <= heights[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/beautiful-towers-i/
// discuss: https://leetcode.com/problems/beautiful-towers-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximum_sum_of_heights(heights: Vec<i32>) -> i64 {
        let mut heights = heights;
        let mut diffs = vec![0; heights.len()];

        for t in 0..2 {
            if t == 1 {
                heights.reverse();
                diffs.reverse();
            }

            let mut monotonic = Vec::with_capacity(heights.len());
            let mut diff = 0;

            for (i, &height) in heights.iter().enumerate() {
                let mut width = 1;

                while let Some(&(last_height, last_width)) = monotonic.last() {
                    if last_height < height {
                        break;
                    }

                    diff += last_width * (last_height - height) as i64;
                    width += last_width;
                    monotonic.pop();
                }

                diffs[i] += diff;
                monotonic.push((height, width));
            }
        }

        heights.iter().map(|&x| x as i64).sum::<i64>() - diffs.iter().min().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2865() {
        let heights = vec![5, 3, 4, 1, 1];
        let expected = 13;
        assert_eq!(Solution::maximum_sum_of_heights(heights), expected);
        let heights = vec![6, 5, 3, 9, 2, 7];
        let expected = 22;
        assert_eq!(Solution::maximum_sum_of_heights(heights), expected);
        let heights = vec![3, 2, 5, 5, 2, 3];
        let expected = 18;
        assert_eq!(Solution::maximum_sum_of_heights(heights), expected);
    }
}
