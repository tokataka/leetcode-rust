///
/// # 2419. Longest Subarray With Maximum Bitwise AND
///
/// You are given an integer array `nums` of size `n`.
///
/// Consider a **non-empty** subarray from `nums` that has the **maximum** possible **bitwise AND**.
///
/// * In other words, let `k` be the maximum value of the bitwise AND of **any** subarray of `nums`. Then, only subarrays with a bitwise AND equal to `k` should be considered.
///
/// Return *the length of the **longest** such subarray*.
///
/// The bitwise AND of an array is the bitwise AND of all the numbers in it.
///
/// A **subarray** is a contiguous sequence of elements within an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,3,3,2,2]
/// Output: 2
/// Explanation:
/// The maximum possible bitwise AND of a subarray is 3.
/// The longest subarray with that value is [3,3], so we return 2.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,3,4]
/// Output: 1
/// Explanation:
/// The maximum possible bitwise AND of a subarray is 4.
/// The longest subarray with that value is [4], so we return 1.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/
// discuss: https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut max = 0;
        let mut count = 0;

        for num in nums {
            if num < max {
                count = 0;
                continue;
            }

            if num > max {
                result = 0;
                count = 0;
                max = num;
            }

            count += 1;
            result = result.max(count);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2419() {
        let nums = vec![1, 2, 3, 3, 2, 2];
        let expected = 2;
        assert_eq!(Solution::longest_subarray(nums), expected);
        let nums = vec![1, 2, 3, 4];
        let expected = 1;
        assert_eq!(Solution::longest_subarray(nums), expected);
    }
}
