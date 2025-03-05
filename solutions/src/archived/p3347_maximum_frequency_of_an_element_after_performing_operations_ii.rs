///
/// # 3347. Maximum Frequency of an Element After Performing Operations II
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
/// * Adding 0 to `nums[1]`, after which `nums` becomes `[1, 4, 5]`.
/// * Adding -1 to `nums[2]`, after which `nums` becomes `[1, 4, 4]`.
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
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `0 <= k <= 10<sup>9</sup>`
/// * `0 <= numOperations <= nums.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-ii/
// discuss: https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut nums = nums;

        nums.sort();

        let mut nums_keys = nums
            .iter()
            .flat_map(|&num| [num - k, num, num + k])
            .collect::<Vec<_>>();

        nums_keys.sort();
        nums_keys.dedup();

        let mut result = 0;

        for num in nums_keys {
            let lower = nums.partition_point(|&x| x < num - k) as i32;
            let upper = nums.partition_point(|&x| x <= num + k) as i32;

            let num_lower = nums.partition_point(|&x| x < num) as i32;
            let num_upper = nums.partition_point(|&x| x <= num) as i32;

            let num_count = num_upper - num_lower;
            let ops = (upper - lower - num_count).min(num_operations);

            result = result.max(num_count + ops);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3347() {
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
        let nums = vec![9];
        let k = 0;
        let num_operations = 0;
        let expected = 1;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), expected);
    }
}
