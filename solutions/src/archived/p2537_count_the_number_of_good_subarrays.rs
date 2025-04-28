use std::collections::HashMap;

///
/// # 2537. Count the Number of Good Subarrays
///
/// Given an integer array `nums` and an integer `k`, return *the number of **good** subarrays of* `nums`.
///
/// A subarray `arr` is **good** if there are **at least** `k` pairs of indices `(i, j)` such that `i < j` and `arr[i] == arr[j]`.
///
/// A **subarray** is a contiguous **non-empty** sequence of elements within an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,1,1,1,1], k = 10
/// Output: 1
/// Explanation: The only good subarray is the array nums itself.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [3,1,4,3,2,2,4], k = 2
/// Output: 4
/// Explanation: There are 4 different good subarrays:
/// - [3,1,4,3,2,2] that has 2 pairs.
/// - [3,1,4,3,2,2,4] that has 3 pairs.
/// - [1,4,3,2,2,4] that has 2 pairs.
/// - [4,3,2,2,4] that has 2 pairs.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i], k <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-good-subarrays/
// discuss: https://leetcode.com/problems/count-the-number-of-good-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut num_counts: HashMap<i32, i32> = HashMap::new();
        let mut pairs = 0;
        let mut result = 0;
        let mut left = 0;

        for &num in &nums {
            let right_count = num_counts.entry(num).or_default();

            pairs += *right_count;
            *right_count += 1;

            while pairs >= k {
                let left_count = num_counts.entry(nums[left]).or_default();

                *left_count -= 1;
                pairs -= *left_count;

                left += 1;
            }

            result += left as i64;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2537() {
        let nums = vec![1, 1, 1, 1, 1];
        let k = 10;
        let expected = 1;
        assert_eq!(Solution::count_good(nums, k), expected);
        let nums = vec![3, 1, 4, 3, 2, 2, 4];
        let k = 2;
        let expected = 4;
        assert_eq!(Solution::count_good(nums, k), expected);
    }
}
