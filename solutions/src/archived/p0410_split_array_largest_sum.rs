use std::collections::HashMap;

///
/// # 410. Split Array Largest Sum
///
/// Given an integer array `nums` and an integer `k`, split `nums` into `k` non-empty subarrays such that the largest sum of any subarray is **minimized**.
///
/// Return *the minimized largest sum of the split*.
///
/// A **subarray** is a contiguous part of the array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [7,2,5,10,8], k = 2
/// Output: 18
/// Explanation: There are four ways to split nums into two subarrays.
/// The best way is to split it into [7,2,5] and [10,8], where the largest sum among the two subarrays is only 18.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,3,4,5], k = 2
/// Output: 9
/// Explanation: There are four ways to split nums into two subarrays.
/// The best way is to split it into [1,2,3] and [4,5], where the largest sum among the two subarrays is only 9.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 1000`
/// * `0 <= nums[i] <= 10<sup>6</sup>`
/// * `1 <= k <= min(50, nums.length)`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/split-array-largest-sum/
// discuss: https://leetcode.com/problems/split-array-largest-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut cache = HashMap::new();

        let prefix_sum = nums.iter().fold(vec![0], |mut acc, x| {
            acc.push(acc.last().unwrap() + x);
            acc
        });

        fn _split_array(
            cache: &mut HashMap<(usize, i32), i32>,
            prefix_sum: &[i32],
            cur: usize,
            split_remain: i32,
        ) -> i32 {
            if let Some(&x) = cache.get(&(cur, split_remain)) {
                return x;
            }

            if split_remain == 0 {
                return prefix_sum.last().unwrap() - prefix_sum[cur];
            }

            let prev_sum = prefix_sum[cur];
            let sum_div = (prefix_sum.last().unwrap() - prev_sum) / (split_remain + 1);

            let start_idx = (cur + 1).max(prefix_sum.partition_point(|&x| x < prev_sum + sum_div) - 1);

            let mut result = i32::MAX;

            for idx in start_idx..prefix_sum.len() - split_remain as usize {
                let cur_sum = prefix_sum[idx] - prev_sum;
                let next = _split_array(cache, prefix_sum, idx, split_remain - 1);

                let maybe_result = cur_sum.max(next);

                if maybe_result >= result {
                    break;
                }

                result = maybe_result;
            }

            cache.insert((cur, split_remain), result);

            result
        }

        _split_array(&mut cache, &prefix_sum, 0, k - 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_410() {
        let nums = vec![7, 2, 5, 10, 8];
        let k = 2;
        let expected = 18;
        assert_eq!(Solution::split_array(nums, k), expected);
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;
        let expected = 9;
        assert_eq!(Solution::split_array(nums, k), expected);
    }
}
