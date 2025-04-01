///
/// # 2401. Longest Nice Subarray
///
/// You are given an array `nums` consisting of **positive** integers.
///
/// We call a subarray of `nums` **nice** if the bitwise **AND** of every pair of elements that are in **different** positions in the subarray is equal to `0`.
///
/// Return *the length of the **longest** nice subarray*.
///
/// A **subarray** is a **contiguous** part of an array.
///
/// **Note** that subarrays of length `1` are always considered nice.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,3,8,48,10]
/// Output: 3
/// Explanation: The longest nice subarray is [3,8,48]. This subarray satisfies the conditions:
/// - 3 AND 8 = 0.
/// - 3 AND 48 = 0.
/// - 8 AND 48 = 0.
/// It can be proven that no longer nice subarray can be obtained, so we return 3.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [3,1,5,11,13]
/// Output: 1
/// Explanation: The length of the longest nice subarray is 1. Any subarray of length 1 can be chosen.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-nice-subarray/
// discuss: https://leetcode.com/problems/longest-nice-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut nice = 0;

        let mut result = 0;

        while right < nums.len() {
            while nums[right] & nice != 0 {
                nice &= !nums[left];
                left += 1;
            }

            nice |= nums[right];
            right += 1;

            result = result.max(right - left);
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2401() {
        let nums = vec![1, 3, 8, 48, 10];
        let expected = 3;
        assert_eq!(Solution::longest_nice_subarray(nums), expected);
        let nums = vec![3, 1, 5, 11, 13];
        let expected = 1;
        assert_eq!(Solution::longest_nice_subarray(nums), expected);
    }
}
