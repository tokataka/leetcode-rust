///
/// # 2221. Find Triangular Sum of an Array
///
/// You are given a **0-indexed** integer array `nums`, where `nums[i]` is a digit between `0` and `9` (**inclusive**).
///
/// The **triangular sum** of `nums` is the value of the only element present in `nums` after the following process terminates:
///
/// 1. Let `nums` comprise of `n` elements. If `n == 1`, **end** the process. Otherwise, **create** a new **0-indexed** integer array `newNums` of length `n - 1`.
/// 2. For each index `i`, where `0 <= i < n - 1`, **assign** the value of `newNums[i]` as `(nums[i] + nums[i+1]) % 10`, where `%` denotes modulo operator.
/// 3. **Replace** the array `nums` with `newNums`.
/// 4. **Repeat** the entire process starting from step 1.
///
/// Return *the triangular sum of* `nums`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/02/22/ex1drawio.png)
///
/// ```
/// Input: nums = [1,2,3,4,5]
/// Output: 8
/// Explanation:
/// The above diagram depicts the process from which we obtain the triangular sum of the array.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [5]
/// Output: 5
/// Explanation:
/// Since there is only one element in nums, the triangular sum is the value of that element itself.
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 1000`
/// * `0 <= nums[i] <= 9`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-triangular-sum-of-an-array/
// discuss: https://leetcode.com/problems/find-triangular-sum-of-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        for n in (1..nums.len()).rev() {
            for i in 0..n {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
        }

        nums[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2221() {
        let nums = vec![1, 2, 3, 4, 5];
        let expected = 8;
        assert_eq!(Solution::triangular_sum(nums), expected);
        let nums = vec![5];
        let expected = 5;
        assert_eq!(Solution::triangular_sum(nums), expected);
    }
}
