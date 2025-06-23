///
/// # 2025. Maximum Number of Ways to Partition an Array
///
/// You are given a **0-indexed** integer array `nums` of length `n`. The number of ways to **partition** `nums` is the number of `pivot` indices that satisfy both conditions:
///
/// * `1 <= pivot < n`
/// * `nums[0] + nums[1] + ... + nums[pivot - 1] == nums[pivot] + nums[pivot + 1] + ... + nums[n - 1]`
///
/// You are also given an integer `k`. You can choose to change the value of **one** element of `nums` to `k`, or to leave the array **unchanged**.
///
/// Return *the **maximum** possible number of ways to **partition*** `nums` *to satisfy both conditions after changing **at most** one element*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,-1,2], k = 3
/// Output: 1
/// Explanation: One optimal approach is to change nums[0] to k. The array becomes [3,-1,2].
/// There is one way to partition the array:
/// - For pivot = 2, we have the partition [3,-1 | 2]: 3 + -1 == 2.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [0,0,0], k = 1
/// Output: 2
/// Explanation: The optimal approach is to leave the array unchanged.
/// There are two ways to partition the array:
/// - For pivot = 1, we have the partition [0 | 0,0]: 0 == 0 + 0.
/// - For pivot = 2, we have the partition [0,0 | 0]: 0 + 0 == 0.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [22,4,-25,-20,-15,15,-16,7,19,-10,0,-13,-14], k = -33
/// Output: 4
/// Explanation: One optimal approach is to change nums[2] to k. The array becomes [22,4,-33,-20,-15,15,-16,7,19,-10,0,-13,-14].
/// There are four ways to partition the array.
///
/// ```
///
/// **Constraints:**
///
/// * `n == nums.length`
/// * `2 <= n <= 10<sup>5</sup>`
/// * `-10<sup>5</sup> <= k, nums[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-ways-to-partition-an-array/
// discuss: https://leetcode.com/problems/maximum-number-of-ways-to-partition-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut cur_sum = 0;
        let mut occurance: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            cur_sum += num;
            if i < n - 1 {
                occurance.entry(cur_sum).or_default().push(i);
            }
        }

        let total_sum = cur_sum;

        let mut result = 0;

        if total_sum % 2 == 0 {
            result = occurance
                .get(&(total_sum / 2))
                .map(|x| x.len())
                .unwrap_or(0) as i32
        };

        for (idx, &num) in nums.iter().enumerate() {
            let diff = k - num;

            if (total_sum + diff) % 2 != 0 {
                continue;
            }

            let target_before = (total_sum + diff) / 2;
            let target_after = target_before - diff;

            let mut cur_result = occurance
                .get(&target_before)
                .map(|o| o.partition_point(|&x| x < idx))
                .unwrap_or(0) as i32;

            cur_result += occurance
                .get(&target_after)
                .map(|o| o.len() - o.partition_point(|&x| x < idx))
                .unwrap_or(0) as i32;

            result = result.max(cur_result);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025() {
        let nums = vec![2, -1, 2];
        let k = 3;
        let expected = 1;
        assert_eq!(Solution::ways_to_partition(nums, k), expected);
        let nums = vec![0, 0, 0];
        let k = 1;
        let expected = 2;
        assert_eq!(Solution::ways_to_partition(nums, k), expected);
        let nums = vec![22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14];
        let k = -33;
        let expected = 4;
        assert_eq!(Solution::ways_to_partition(nums, k), expected);
    }
}
