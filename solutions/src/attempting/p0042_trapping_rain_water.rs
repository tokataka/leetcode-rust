///
/// # 42. Trapping Rain Water
///
/// Given `n` non-negative integers representing an elevation map where the width of each bar is `1`, compute how much water it can trap after raining.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png)
///
/// ```
/// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
/// Output: 6
/// Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: height = [4,2,0,3,2,5]
/// Output: 9
///
/// ```
///
/// **Constraints:**
///
/// * `n == height.length`
/// * `1 <= n <= 2 * 10<sup>4</sup>`
/// * `0 <= height[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water/
// discuss: https://leetcode.com/problems/trapping-rain-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut monotonic = Vec::with_capacity(height.len());

        for (i, &h) in height.iter().enumerate() {
            let i = i as i32;

            let mut prev_h = 0;

            while let Some(&(last_i, last_h)) = monotonic.last() {
                if last_h > h {
                    result += (i - last_i - 1) * (h - prev_h);
                    break;
                }

                result += (i - last_i - 1) * (last_h - prev_h);
                prev_h = last_h;
                monotonic.pop();
            }

            monotonic.push((i, h));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let expected = 6;
        assert_eq!(Solution::trap(height), expected);
        let height = vec![4, 2, 0, 3, 2, 5];
        let expected = 9;
        assert_eq!(Solution::trap(height), expected);
    }
}
