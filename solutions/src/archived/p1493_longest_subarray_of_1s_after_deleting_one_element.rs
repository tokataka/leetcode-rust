///
/// # 1493. Longest Subarray of 1's After Deleting One Element
///
/// Given a binary array `nums`, you should delete one element from it.
///
/// Return *the size of the longest non-empty subarray containing only* `1`*'s in the resulting array*. Return `0` if there is no such subarray.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,1,0,1]
/// Output: 3
/// Explanation: After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1's.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [0,1,1,1,0,1,1,0,1]
/// Output: 5
/// Explanation: After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1's is [1,1,1,1,1].
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [1,1,1]
/// Output: 2
/// Explanation: You must delete one element.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `nums[i]` is either `0` or `1`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
// discuss: https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut prev = 0;
        let mut cur = 0;

        for &num in &nums {
            if num == 0 {
                prev = cur;
                cur = 0;
            } else {
                cur += 1;
                result = result.max(prev + cur);
            }
        }

        result.min(nums.len() as i32 - 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1493() {
        let nums = vec![1, 1, 0, 1];
        let expected = 3;
        assert_eq!(Solution::longest_subarray(nums), expected);
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let expected = 5;
        assert_eq!(Solution::longest_subarray(nums), expected);
        let nums = vec![1, 1, 1];
        let expected = 2;
        assert_eq!(Solution::longest_subarray(nums), expected);
    }
}
