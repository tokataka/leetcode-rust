///
/// # 1755. Closest Subsequence Sum
///
/// You are given an integer array `nums` and an integer `goal`.
///
/// You want to choose a subsequence of `nums` such that the sum of its elements is the closest possible to `goal`. That is, if the sum of the subsequence's elements is `sum`, then you want to **minimize the absolute difference** `abs(sum - goal)`.
///
/// Return *the **minimum** possible value of* `abs(sum - goal)`.
///
/// Note that a subsequence of an array is an array formed by removing some elements **(possibly all or none)** of the original array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [5,-7,3,5], goal = 6
/// Output: 0
/// Explanation: Choose the whole array as a subsequence, with a sum of 6.
/// This is equal to the goal, so the absolute difference is 0.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [7,-9,15,-2], goal = -5
/// Output: 1
/// Explanation: Choose the subsequence [7,-9,-2], with a sum of -4.
/// The absolute difference is abs(-4 - (-5)) = abs(1) = 1, which is the minimum.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [1,2,3], goal = -7
/// Output: 7
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 40`
/// * `-10<sup>7</sup> <= nums[i] <= 10<sup>7</sup>`
/// * `-10<sup>9</sup> <= goal <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/closest-subsequence-sum/
// discuss: https://leetcode.com/problems/closest-subsequence-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    fn get_sums(nums: &[i32]) -> Vec<i32> {
        let mut sums = std::collections::HashSet::new();
        sums.insert(0);
        for &num in nums {
            let new_sums: Vec<i32> = sums.iter().map(|&s| s + num).collect();
            sums.extend(new_sums);
        }
        sums.into_iter().collect()
    }

    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let sums1 = Self::get_sums(&nums[..n / 2]);
        let mut sums2 = Self::get_sums(&nums[n / 2..]);
        sums2.sort_unstable();

        let mut min_diff = sums1
            .iter()
            .map(|&s| (s - goal).abs())
            .min()
            .unwrap_or(i32::MAX);
        min_diff = min_diff.min(
            sums2
                .iter()
                .map(|&s| (s - goal).abs())
                .min()
                .unwrap_or(i32::MAX),
        );

        for &s1 in &sums1 {
            let target = goal - s1;
            match sums2.binary_search(&target) {
                Ok(_) => return 0,
                Err(i) => {
                    if i < sums2.len() {
                        min_diff = min_diff.min((s1 + sums2[i] - goal).abs());
                    }
                    if i > 0 {
                        min_diff = min_diff.min((s1 + sums2[i - 1] - goal).abs());
                    }
                }
            }
        }

        min_diff
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1755() {
        let nums = vec![5, -7, 3, 5];
        let goal = 6;
        let expected = 0;
        assert_eq!(Solution::min_abs_difference(nums, goal), expected);
        let nums = vec![7, -9, 15, -2];
        let goal = -5;
        let expected = 1;
        assert_eq!(Solution::min_abs_difference(nums, goal), expected);
        let nums = vec![1, 2, 3];
        let goal = -7;
        let expected = 7;
        assert_eq!(Solution::min_abs_difference(nums, goal), expected);
    }
}
