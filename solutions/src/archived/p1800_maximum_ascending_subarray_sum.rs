///
/// # 1800. Maximum Ascending Subarray Sum
///
/// Given an array of positive integers `nums`, return the *maximum possible sum of an **ascending** subarray in* `nums`.
///
/// A subarray is defined as a contiguous sequence of numbers in an array.
///
/// A subarray `[nums<sub>l</sub>, nums<sub>l+1</sub>, ..., nums<sub>r-1</sub>, nums<sub>r</sub>]` is **ascending** if for all `i` where `l <= i < r`, `nums<sub>i </sub> < nums<sub>i+1</sub>`. Note that a subarray of size `1` is **ascending**.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [10,20,30,5,10,50]
/// Output: 65
/// Explanation: [5,10,50] is the ascending subarray with the maximum sum of 65.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [10,20,30,40,50]
/// Output: 150
/// Explanation: [10,20,30,40,50] is the ascending subarray with the maximum sum of 150.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [12,17,15,13,10,11,12]
/// Output: 33
/// Explanation: [10,11,12] is the ascending subarray with the maximum sum of 33.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 100`
/// * `1 <= nums[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-ascending-subarray-sum/
// discuss: https://leetcode.com/problems/maximum-ascending-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        nums.chunk_by(|a, b| a < b)
            .map(|x| x.iter().sum())
            .max()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1800() {
        let nums = vec![10, 20, 30, 5, 10, 50];
        let expected = 65;
        assert_eq!(Solution::max_ascending_sum(nums), expected);
        let nums = vec![10, 20, 30, 40, 50];
        let expected = 150;
        assert_eq!(Solution::max_ascending_sum(nums), expected);
        let nums = vec![12, 17, 15, 13, 10, 11, 12];
        let expected = 33;
        assert_eq!(Solution::max_ascending_sum(nums), expected);
    }
}
