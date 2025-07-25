///
/// # 2163. Minimum Difference in Sums After Removal of Elements
///
/// You are given a **0-indexed** integer array `nums` consisting of `3 * n` elements.
///
/// You are allowed to remove any **subsequence** of elements of size **exactly** `n` from `nums`. The remaining `2 * n` elements will be divided into two **equal** parts:
///
/// * The first `n` elements belonging to the first part and their sum is `sum<sub>first</sub>`.
/// * The next `n` elements belonging to the second part and their sum is `sum<sub>second</sub>`.
///
/// The **difference in sums** of the two parts is denoted as `sum<sub>first</sub> - sum<sub>second</sub>`.
///
/// * For example, if `sum<sub>first</sub> = 3` and `sum<sub>second</sub> = 2`, their difference is `1`.
/// * Similarly, if `sum<sub>first</sub> = 2` and `sum<sub>second</sub> = 3`, their difference is `-1`.
///
/// Return *the **minimum difference** possible between the sums of the two parts after the removal of* `n` *elements*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [3,1,2]
/// Output: -1
/// Explanation: Here, nums has 3 elements, so n = 1.
/// Thus we have to remove 1 element from nums and divide the array into two equal parts.
/// - If we remove nums[0] = 3, the array will be [1,2]. The difference in sums of the two parts will be 1 - 2 = -1.
/// - If we remove nums[1] = 1, the array will be [3,2]. The difference in sums of the two parts will be 3 - 2 = 1.
/// - If we remove nums[2] = 2, the array will be [3,1]. The difference in sums of the two parts will be 3 - 1 = 2.
/// The minimum difference between sums of the two parts is min(-1,1,2) = -1.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [7,9,5,8,1,3]
/// Output: 1
/// Explanation: Here n = 2. So we must remove 2 elements and divide the remaining array into two parts containing two elements each.
/// If we remove nums[2] = 5 and nums[3] = 8, the resultant array will be [7,9,1,3]. The difference in sums will be (7+9) - (1+3) = 12.
/// To obtain the minimum difference, we should remove nums[1] = 9 and nums[4] = 1. The resultant array becomes [7,5,8,3]. The difference in sums of the two parts is (7+5) - (8+3) = 1.
/// It can be shown that it is not possible to obtain a difference smaller than 1.
///
/// ```
///
/// **Constraints:**
///
/// * `nums.length == 3 * n`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-difference-in-sums-after-removal-of-elements/
// discuss: https://leetcode.com/problems/minimum-difference-in-sums-after-removal-of-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;

        let mut left_heap = BinaryHeap::new();
        let mut current_left_sum = 0;

        let mut iter_left = nums.iter().map(|&x| x as i64);

        for num in iter_left.by_ref().take(n) {
            left_heap.push(num);
            current_left_sum += num;
        }

        let mut min_left_sums = vec![current_left_sum];

        for num in iter_left.take(n) {
            if let Some(&left_max) = left_heap.peek() {
                if num < left_max {
                    current_left_sum -= left_max - num;
                    left_heap.pop();
                    left_heap.push(num);
                }
            }
            min_left_sums.push(current_left_sum);
        }

        let mut right_heap = BinaryHeap::new();
        let mut current_right_sum = 0;

        let mut iter_right = nums.iter().rev().map(|&x| x as i64);

        for num in iter_right.by_ref().take(n) {
            right_heap.push(Reverse(num));
            current_right_sum += num;
        }

        let mut max_right_sums = vec![current_right_sum];

        for num in iter_right.take(n) {
            if let Some(&Reverse(right_min)) = right_heap.peek() {
                if num > right_min {
                    current_right_sum += num - right_min;
                    right_heap.pop();
                    right_heap.push(Reverse(num));
                }
            }
            max_right_sums.push(current_right_sum);
        }

        min_left_sums
            .into_iter()
            .zip(max_right_sums.into_iter().rev())
            .map(|(left_sum, right_sum)| left_sum - right_sum)
            .min()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2163() {
        let nums = vec![3, 1, 2];
        let expected = -1;
        assert_eq!(Solution::minimum_difference(nums), expected);
        let nums = vec![7, 9, 5, 8, 1, 3];
        let expected = 1;
        assert_eq!(Solution::minimum_difference(nums), expected);
    }
}
