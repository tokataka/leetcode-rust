///
/// # 560. Subarray Sum Equals K
///
/// Given an array of integers `nums` and an integer `k`, return *the total number of subarrays whose sum equals to* `k`.
///
/// A subarray is a contiguous **non-empty** sequence of elements within an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,1,1], k = 2
/// Output: 2
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,3], k = 3
/// Output: 2
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 2 * 10<sup>4</sup>`
/// * `-1000 <= nums[i] <= 1000`
/// * `-10<sup>7</sup> <= k <= 10<sup>7</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-sum-equals-k/
// discuss: https://leetcode.com/problems/subarray-sum-equals-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum_count_map = HashMap::new();
        sum_count_map.insert(0, 1);

        let mut result = 0;
        let mut sum = 0;

        for num in nums {
            sum += num;

            result += sum_count_map.get(&(sum - k)).unwrap_or(&0);
            *sum_count_map.entry(sum).or_default() += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_560() {
        let nums = vec![1, 1, 1];
        let k = 2;
        let expected = 2;
        assert_eq!(Solution::subarray_sum(nums, k), expected);
        let nums = vec![1, 2, 3];
        let k = 3;
        let expected = 2;
        assert_eq!(Solution::subarray_sum(nums, k), expected);
    }
}
