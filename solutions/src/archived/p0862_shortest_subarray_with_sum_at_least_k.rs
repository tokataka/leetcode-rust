///
/// # 862. Shortest Subarray with Sum at Least K
///
/// Given an integer array `nums` and an integer `k`, return *the length of the shortest non-empty **subarray** of* `nums` *with a sum of at least* `k`. If there is no such **subarray**, return `-1`.
///
/// A **subarray** is a **contiguous** part of an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1], k = 1
/// Output: 1
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2], k = 4
/// Output: -1
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [2,-1,2], k = 3
/// Output: 3
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>`
/// * `1 <= k <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/
// discuss: https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = usize::MAX;
        let mut st: Vec<(i64, usize)> = vec![(0, 0)];
        let mut partial_sum = 0;

        for (cur_idx, cur_val) in nums.into_iter().enumerate() {
            let right_idx = cur_idx + 1;
            partial_sum += cur_val as i64;

            match st.partition_point(|&x| x.0 <= partial_sum - k as i64) {
                0 => {}
                idx => {
                    let (_, left_idx) = st[idx - 1];

                    result = result.min(right_idx - left_idx);
                }
            };

            while let Some(x) = st.last() {
                if x.0 >= partial_sum {
                    st.pop();
                } else {
                    break;
                }
            }

            st.push((partial_sum, right_idx));
        }

        match result {
            usize::MAX => -1,
            x => x as i32,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_862() {
        let nums = vec![1];
        let k = 1;
        let expected = 1;
        assert_eq!(Solution::shortest_subarray(nums, k), expected);
        let nums = vec![1, 2];
        let k = 4;
        let expected = -1;
        assert_eq!(Solution::shortest_subarray(nums, k), expected);
        let nums = vec![2, -1, 2];
        let k = 3;
        let expected = 3;
        assert_eq!(Solution::shortest_subarray(nums, k), expected);
    }
}
