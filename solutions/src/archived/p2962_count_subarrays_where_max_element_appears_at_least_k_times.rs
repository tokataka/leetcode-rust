use std::collections::VecDeque;

///
/// # 2962. Count Subarrays Where Max Element Appears at Least K Times
///
/// You are given an integer array `nums` and a **positive** integer `k`.
///
/// Return *the number of subarrays where the **maximum** element of* `nums` *appears **at least*** `k` *times in that subarray.*
///
/// A **subarray** is a contiguous sequence of elements within an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,3,2,3,3], k = 2
/// Output: 6
/// Explanation: The subarrays that contain the element 3 at least 2 times are: [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,4,2,1], k = 3
/// Output: 0
/// Explanation: No subarray contains the element 4 at least 3 times.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>6</sup>`
/// * `1 <= k <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
// discuss: https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_num = *nums.iter().max().unwrap();
        let k = k as usize;

        let mut result = 0;
        let mut q = VecDeque::with_capacity(k);

        let mut right_iter = nums
            .iter()
            .enumerate()
            .filter(|&(_, &x)| x == max_num)
            .map(|x| x.0)
            .chain(std::iter::once(nums.len()));

        for right in right_iter.by_ref().take(k) {
            q.push_back(right);
        }

        for right in right_iter {
            result += (q.front().unwrap() + 1) * (right - q.back().unwrap());
            q.pop_front();
            q.push_back(right);
        }

        result as i64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2962() {
        let nums = vec![1, 3, 2, 3, 3];
        let k = 2;
        let expected = 6;
        assert_eq!(Solution::count_subarrays(nums, k), expected);
        let nums = vec![1, 4, 2, 1];
        let k = 3;
        let expected = 0;
        assert_eq!(Solution::count_subarrays(nums, k), expected);
        let nums = vec![
            61, 23, 38, 23, 56, 40, 82, 56, 82, 82, 82, 70, 8, 69, 8, 7, 19, 14, 58, 42, 82, 10,
            82, 78, 15, 82,
        ];
        let k = 2;
        let expected = 224;
        assert_eq!(Solution::count_subarrays(nums, k), expected);
    }
}
