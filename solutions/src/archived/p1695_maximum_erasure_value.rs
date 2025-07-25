///
/// # 1695. Maximum Erasure Value
///
/// You are given an array of positive integers `nums` and want to erase a subarray containing **unique elements**. The **score** you get by erasing the subarray is equal to the **sum** of its elements.
///
/// Return *the **maximum score** you can get by erasing **exactly one** subarray.*
///
/// An array `b` is called to be a subarray of `a` if it forms a contiguous subsequence of `a`, that is, if it is equal to `a[l],a[l+1],...,a[r]` for some `(l,r)`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [4,2,4,5,6]
/// Output: 17
/// Explanation: The optimal subarray here is [2,4,5,6].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [5,2,1,2,5,2,1,2,5]
/// Output: 8
/// Explanation: The optimal subarray here is [5,2,1] or [1,2,5].
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-erasure-value/
// discuss: https://leetcode.com/problems/maximum-erasure-value/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut sum = 0;
        let mut count = vec![0; 10001];

        let mut left_idx = 0;

        for &right in &nums {
            sum += right;
            count[right as usize] += 1;

            while count[right as usize] > 1 {
                let left = nums[left_idx];
                sum -= left;
                count[left as usize] -= 1;

                left_idx += 1;
            }

            max_sum = max_sum.max(sum);
        }

        max_sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1695() {
        let nums = vec![4, 2, 4, 5, 6];
        let expected = 17;
        assert_eq!(Solution::maximum_unique_subarray(nums), expected);
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        let expected = 8;
        assert_eq!(Solution::maximum_unique_subarray(nums), expected);
    }
}
