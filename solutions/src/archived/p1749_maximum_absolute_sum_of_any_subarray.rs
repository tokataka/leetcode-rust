///
/// # 1749. Maximum Absolute Sum of Any Subarray
///
/// You are given an integer array `nums`. The **absolute sum** of a subarray `[nums<sub>l</sub>, nums<sub>l+1</sub>, ..., nums<sub>r-1</sub>, nums<sub>r</sub>]` is `abs(nums<sub>l</sub> + nums<sub>l+1</sub> + ... + nums<sub>r-1</sub> + nums<sub>r</sub>)`.
///
/// Return *the **maximum** absolute sum of any **(possibly empty)** subarray of* `nums`.
///
/// Note that `abs(x)` is defined as follows:
///
/// * If `x` is a negative integer, then `abs(x) = -x`.
/// * If `x` is a non-negative integer, then `abs(x) = x`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,-3,2,3,-4]
/// Output: 5
/// Explanation: The subarray [2,3] has absolute sum = abs(2+3) = abs(5) = 5.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [2,-5,1,-4,3,-2]
/// Output: 8
/// Explanation: The subarray [-5,1,-4] has absolute sum = abs(-5+1-4) = abs(-8) = 8.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/
// discuss: https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        nums.iter()
            .scan((0, 0), |state, num| {
                *state = (0.max(state.0 + num), 0.max(state.1 - num));
                Some(state.0.max(state.1))
            })
            .max()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1749() {
        let nums = vec![1, -3, 2, 3, -4];
        let expected = 5;
        assert_eq!(Solution::max_absolute_sum(nums), expected);
        let nums = vec![2, -5, 1, -4, 3, -2];
        let expected = 8;
        assert_eq!(Solution::max_absolute_sum(nums), expected);
    }
}
