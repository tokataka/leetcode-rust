///
/// # 3381. Maximum Subarray Sum With Length Divisible by K
///
/// You are given an array of integers `nums` and an integer `k`.
///
/// Return the **maximum** sum of a subarray of `nums`, such that the size of the subarray is **divisible** by `k`.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2], k = 1
///
/// **Output:** 3
///
/// **Explanation:**
///
/// The subarray `[1, 2]` with sum 3 has length equal to 2 which is divisible by 1.
///
/// **Example 2:**
///
/// **Input:** nums = [-1,-2,-3,-4,-5], k = 4
///
/// **Output:** -10
///
/// **Explanation:**
///
/// The maximum sum subarray is `[-1, -2, -3, -4]` which has length equal to 4 which is divisible by 4.
///
/// **Example 3:**
///
/// **Input:** nums = [-5,1,2,-3,4], k = 2
///
/// **Output:** 4
///
/// **Explanation:**
///
/// The maximum sum subarray is `[1, 2, -3, 4]` which has length equal to 4 which is divisible by 2.
///
/// **Constraints:**
///
/// * `1 <= k <= nums.length <= 2 * 10<sup>5</sup>`
/// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/
// discuss: https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut min_prefix_sum = vec![i64::MAX; k];
        min_prefix_sum[0] = 0;

        let mut current_sum = 0i64;
        let mut max_sum = i64::MIN;

        for (i, &num) in nums.iter().enumerate() {
            current_sum += num as i64;
            let remainder = (i + 1) % k;

            if min_prefix_sum[remainder] != i64::MAX {
                max_sum = max_sum.max(current_sum - min_prefix_sum[remainder]);
            }

            if current_sum < min_prefix_sum[remainder] {
                min_prefix_sum[remainder] = current_sum;
            }
        }

        max_sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3381() {
        assert_eq!(Solution::max_subarray_sum(vec![1, 2], 1), 3);
        assert_eq!(Solution::max_subarray_sum(vec![-1, -2, -3, -4, -5], 4), -10);
        assert_eq!(Solution::max_subarray_sum(vec![-5, 1, 2, -3, 4], 2), 4);
    }
}
