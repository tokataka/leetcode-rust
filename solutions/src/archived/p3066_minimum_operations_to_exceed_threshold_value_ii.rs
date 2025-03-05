use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 3066. Minimum Operations to Exceed Threshold Value II
///
/// You are given a **0-indexed** integer array `nums`, and an integer `k`.
///
/// In one operation, you will:
///
/// * Take the two smallest integers `x` and `y` in `nums`.
/// * Remove `x` and `y` from `nums`.
/// * Add `min(x, y) * 2 + max(x, y)` anywhere in the array.
///
/// **Note** that you can only apply the described operation if `nums` contains at least two elements.
///
/// Return *the **minimum** number of operations needed so that all elements of the array are greater than or equal to* `k`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,11,10,1,3], k = 10
/// Output: 2
/// Explanation: In the first operation, we remove elements 1 and 2, then add 1 * 2 + 2 to nums. nums becomes equal to [4, 11, 10, 3].
/// In the second operation, we remove elements 3 and 4, then add 3 * 2 + 4 to nums. nums becomes equal to [10, 11, 10].
/// At this stage, all the elements of nums are greater than or equal to 10 so we can stop.
/// It can be shown that 2 is the minimum number of operations needed so that all elements of the array are greater than or equal to 10.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,1,2,4,9], k = 20
/// Output: 4
/// Explanation: After one operation, nums becomes equal to [2, 4, 9, 3].
/// After two operations, nums becomes equal to [7, 4, 9].
/// After three operations, nums becomes equal to [15, 9].
/// After four operations, nums becomes equal to [33].
/// At this stage, all the elements of nums are greater than 20 so we can stop.
/// It can be shown that 4 is the minimum number of operations needed so that all elements of the array are greater than or equal to 20.
/// ```
///
/// **Constraints:**
///
/// * `2 <= nums.length <= 2 * 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `1 <= k <= 10<sup>9</sup>`
/// * The input is generated such that an answer always exists. That is, there exists some sequence of operations after which all elements of the array are greater than or equal to `k`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/
// discuss: https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut pq = BinaryHeap::from_iter(nums.into_iter().map(|x| Reverse(x as i64)));
        let k = k as i64;

        for i in 0.. {
            if let Some(&Reverse(top)) = pq.peek() {
                if top >= k {
                    return i;
                }
            }

            let Reverse(a) = pq.pop().unwrap();
            let Reverse(b) = pq.pop().unwrap();

            pq.push(Reverse(a.min(b) * 2 + a.max(b)));
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3066() {
        let nums = vec![2, 11, 10, 1, 3];
        let k = 10;
        let expected = 2;
        assert_eq!(Solution::min_operations(nums, k), expected);
        let nums = vec![1, 1, 2, 4, 9];
        let k = 20;
        let expected = 4;
        assert_eq!(Solution::min_operations(nums, k), expected);
    }
}
