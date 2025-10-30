///
/// # 3350. Adjacent Increasing Subarrays Detection II
///
/// Given an array `nums` of `n` integers, your task is to find the **maximum** value of `k` for which there exist **two** adjacent subarrays of length `k` each, such that both subarrays are **strictly** **increasing**. Specifically, check if there are **two** subarrays of length `k` starting at indices `a` and `b` (`a < b`), where:
///
/// * Both subarrays `nums[a..a + k - 1]` and `nums[b..b + k - 1]` are **strictly increasing**.
/// * The subarrays must be **adjacent**, meaning `b = a + k`.
///
/// Return the **maximum** *possible* value of `k`.
///
/// A **subarray** is a contiguous **non-empty** sequence of elements within an array.
///
/// **Example 1:**
///
/// **Input:** nums = [2,5,7,8,9,2,3,4,3,1]
///
/// **Output:** 3
///
/// **Explanation:**
///
/// * The subarray starting at index 2 is `[7, 8, 9]`, which is strictly increasing.
/// * The subarray starting at index 5 is `[2, 3, 4]`, which is also strictly increasing.
/// * These two subarrays are adjacent, and 3 is the **maximum** possible value of `k` for which two such adjacent strictly increasing subarrays exist.
///
/// **Example 2:**
///
/// **Input:** nums = [1,2,3,4,4,4,4,5,6,7]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// * The subarray starting at index 0 is `[1, 2]`, which is strictly increasing.
/// * The subarray starting at index 2 is `[3, 4]`, which is also strictly increasing.
/// * These two subarrays are adjacent, and 2 is the **maximum** possible value of `k` for which two such adjacent strictly increasing subarrays exist.
///
/// **Constraints:**
///
/// * `2 <= nums.length <= 2 * 10<sup>5</sup>`
/// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii/
// discuss: https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let increasing_counts = nums
            .chunk_by(|a, b| a < b)
            .map(|x| x.len() as i32)
            .collect::<Vec<_>>();

        increasing_counts
            .windows(2)
            .map(|x| *x.iter().min().unwrap())
            .max()
            .unwrap_or(0)
            .max(increasing_counts.iter().max().unwrap() / 2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3350() {
        let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        let expected = 3;
        assert_eq!(Solution::max_increasing_subarrays(nums), expected);
        let nums = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        let expected = 2;
        assert_eq!(Solution::max_increasing_subarrays(nums), expected);
    }
}
