///
/// # 2009. Minimum Number of Operations to Make Array Continuous
///
/// You are given an integer array `nums`. In one operation, you can replace **any** element in `nums` with **any** integer.
///
/// `nums` is considered **continuous** if both of the following conditions are fulfilled:
///
/// * All elements in `nums` are **unique**.
/// * The difference between the **maximum** element and the **minimum** element in `nums` equals `nums.length - 1`.
///
/// For example, `nums = [4, 2, 5, 3]` is **continuous**, but `nums = [1, 2, 3, 5, 6]` is **not continuous**.
///
/// Return *the **minimum** number of operations to make* `nums` ***continuous***.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [4,2,5,3]
/// Output: 0
/// Explanation: nums is already continuous.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,3,5,6]
/// Output: 1
/// Explanation: One possible solution is to change the last element to 4.
/// The resulting array is [1,2,3,5,4], which is continuous.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [1,10,100,1000]
/// Output: 3
/// Explanation: One possible solution is to:
/// - Change the second element to 2.
/// - Change the third element to 3.
/// - Change the fourth element to 4.
/// The resulting array is [1,2,3,4], which is continuous.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        nums.sort_unstable();
        nums.dedup();

        let mut result = i32::MAX;

        for (i, &num) in nums.iter().enumerate() {
            let right = nums.partition_point(|&x| x < num + n);
            result = result.min(n - (right - i) as i32);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2009() {
        // let nums = vec![4, 2, 5, 3];
        // let expected = 0;
        // assert_eq!(Solution::min_operations(nums), expected);
        // let nums = vec![1, 2, 3, 5, 6];
        // let expected = 1;
        // assert_eq!(Solution::min_operations(nums), expected);
        // let nums = vec![1, 10, 100, 1000];
        // let expected = 3;
        // assert_eq!(Solution::min_operations(nums), expected);
        let nums = vec![8, 5, 9, 9, 8, 4];
        let expected = 2;
        assert_eq!(Solution::min_operations(nums), expected);
    }
}
