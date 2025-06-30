///
/// # 2099. Find Subsequence of Length K With the Largest Sum
///
/// You are given an integer array `nums` and an integer `k`. You want to find a **subsequence** of `nums` of length `k` that has the **largest** sum.
///
/// Return ***any** such subsequence as an integer array of length* `k`.
///
/// A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,1,3,3], k = 2
/// Output: [3,3]
/// Explanation:
/// The subsequence has the largest sum of 3 + 3 = 6.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [-1,-2,3,4], k = 3
/// Output: [-1,3,4]
/// Explanation:
/// The subsequence has the largest sum of -1 + 3 + 4 = 6.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [3,4,3,3], k = 2
/// Output: [3,4]
/// Explanation:
/// The subsequence has the largest sum of 3 + 4 = 7.
/// Another possible subsequence is [4, 3].
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 1000`
/// * `-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>`
/// * `1 <= k <= nums.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/
// discuss: https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums_with_idx = nums.into_iter().enumerate().collect::<Vec<_>>();

        nums_with_idx.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        nums_with_idx.truncate(k as usize);
        nums_with_idx.sort_unstable();

        nums_with_idx.into_iter().map(|(_, x)| x).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2099() {
        let nums = vec![2, 1, 3, 3];
        let k = 2;
        let expected = vec![3, 3];
        assert_eq!(Solution::max_subsequence(nums, k), expected);
        let nums = vec![-1, -2, 3, 4];
        let k = 3;
        let expected = vec![-1, 3, 4];
        assert_eq!(Solution::max_subsequence(nums, k), expected);
        let nums = vec![3, 4, 3, 3];
        let k = 2;
        let expected = vec![3, 4];
        assert_eq!(Solution::max_subsequence(nums, k), expected);
    }
}
