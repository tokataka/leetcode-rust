///
/// # 3346. Maximum Frequency of an Element After Performing Operations I
///
/// You are given an integer array `nums` and two integers `k` and `numOperations`.
///
/// You must perform an **operation** `numOperations` times on `nums`, where in each operation you:
///
/// * Select an index `i` that was **not** selected in any previous operations.
/// * Add an integer in the range `[-k, k]` to `nums[i]`.
///
/// Return the **maximum** possible frequency of any element in `nums` after performing the **operations**.
///
/// **Example 1:**
///
/// **Input:** nums = [1,4,5], k = 1, numOperations = 2
///
/// **Output:** 2
///
/// **Explanation:**
///
/// We can achieve a maximum frequency of two by:
///
/// * Adding 0 to `nums[1]`. `nums` becomes `[1, 4, 5]`.
/// * Adding -1 to `nums[2]`. `nums` becomes `[1, 4, 4]`.
///
/// **Example 2:**
///
/// **Input:** nums = [5,11,20,20], k = 5, numOperations = 1
///
/// **Output:** 2
///
/// **Explanation:**
///
/// We can achieve a maximum frequency of two by:
///
/// * Adding 0 to `nums[1]`.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>5</sup>`
/// * `0 <= k <= 10<sup>5</sup>`
/// * `0 <= numOperations <= nums.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i/
// discuss: https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let k = k as usize;

        let mut freq = vec![0; 100001];
        let mut min_num = usize::MAX;
        let mut max_num = 0;

        for num in nums {
            freq[num as usize] += 1;
            min_num = min_num.min(num as usize);
            max_num = max_num.max(num as usize);
        }

        let mut result = 1;

        let mut range_count = freq.iter().take(min_num + k).sum::<i32>();

        for (num, &count) in freq.iter().enumerate().take(max_num + 1).skip(min_num) {
            if num + k <= 100000 {
                range_count += freq[num + k];
            }

            result = result.max(count + num_operations.min(range_count - count));

            if num >= k {
                range_count -= freq[num - k];
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
    fn test_3346() {
        // let nums = vec![1, 4, 5];
        // let k = 1;
        // let num_operations = 2;
        // let expected = 2;
        // assert_eq!(Solution::max_frequency(nums, k, num_operations), expected);
        // let nums = vec![5, 11, 20, 20];
        // let k = 5;
        // let num_operations = 1;
        // let expected = 2;
        // assert_eq!(Solution::max_frequency(nums, k, num_operations), expected);
        let nums = vec![88, 53];
        let k = 27;
        let num_operations = 2;
        let expected = 2;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), expected);
    }
}
