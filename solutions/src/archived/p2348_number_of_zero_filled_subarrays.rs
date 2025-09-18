///
/// # 2348. Number of Zero-Filled Subarrays
///
/// Given an integer array `nums`, return *the number of **subarrays** filled with* `0`.
///
/// A **subarray** is a contiguous non-empty sequence of elements within an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,3,0,0,2,0,0,4]
/// Output: 6
/// Explanation:
/// There are 4 occurrences of [0] as a subarray.
/// There are 2 occurrences of [0,0] as a subarray.
/// There is no occurrence of a subarray with a size more than 2 filled with 0. Therefore, we return 6.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [0,0,0,2,0,0]
/// Output: 9
/// Explanation:
/// There are 5 occurrences of [0] as a subarray.
/// There are 3 occurrences of [0,0] as a subarray.
/// There is 1 occurrence of [0,0,0] as a subarray.
/// There is no occurrence of a subarray with a size more than 3 filled with 0. Therefore, we return 9.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [2,10,2019]
/// Output: 0
/// Explanation: There is no subarray filled with 0. Therefore, we return 0.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-zero-filled-subarrays/
// discuss: https://leetcode.com/problems/number-of-zero-filled-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut count = 0;

        for num in nums {
            if num == 0 {
                count += 1;
                result += count;
            } else {
                count = 0;
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
    fn test_2348() {
        let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
        let expected = 6;
        assert_eq!(Solution::zero_filled_subarray(nums), expected);
        let nums = vec![0, 0, 0, 2, 0, 0];
        let expected = 9;
        assert_eq!(Solution::zero_filled_subarray(nums), expected);
        let nums = vec![2, 10, 2019];
        let expected = 0;
        assert_eq!(Solution::zero_filled_subarray(nums), expected);
    }
}
