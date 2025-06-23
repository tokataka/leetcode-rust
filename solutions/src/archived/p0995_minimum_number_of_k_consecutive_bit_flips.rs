///
/// # 995. Minimum Number of K Consecutive Bit Flips
///
/// You are given a binary array `nums` and an integer `k`.
///
/// A **k-bit flip** is choosing a **subarray** of length `k` from `nums` and simultaneously changing every `0` in the subarray to `1`, and every `1` in the subarray to `0`.
///
/// Return *the minimum number of **k-bit flips** required so that there is no* `0` *in the array*. If it is not possible, return `-1`.
///
/// A **subarray** is a **contiguous** part of an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [0,1,0], k = 1
/// Output: 2
/// Explanation: Flip nums[0], then flip nums[2].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,1,0], k = 2
/// Output: -1
/// Explanation: No matter how we flip subarrays of size 2, we cannot make the array become [1,1,1].
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [0,0,0,1,0,1,1,0], k = 3
/// Output: 3
/// Explanation:
/// Flip nums[0],nums[1],nums[2]: nums becomes [1,1,1,1,0,1,1,0]
/// Flip nums[4],nums[5],nums[6]: nums becomes [1,1,1,1,1,0,0,0]
/// Flip nums[5],nums[6],nums[7]: nums becomes [1,1,1,1,1,1,1,1]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= k <= nums.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/
// discuss: https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();

        let mut cur_flip = 0;
        let mut result = 0;

        for i in 0..n {
            let num = nums[i];
            cur_flip ^= if i >= k { nums[i - k] } else { 0 };

            if num ^ cur_flip == 0 {
                if i + k > n {
                    return -1;
                }

                nums[i] = 1;
                cur_flip ^= 1;
                result += 1;
            } else {
                nums[i] = 0;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_995() {
        let nums = vec![0, 1, 0];
        let k = 1;
        let expected = 2;
        assert_eq!(Solution::min_k_bit_flips(nums, k), expected);
        let nums = vec![1, 1, 0];
        let k = 2;
        let expected = -1;
        assert_eq!(Solution::min_k_bit_flips(nums, k), expected);
        let nums = vec![0, 0, 0, 1, 0, 1, 1, 0];
        let k = 3;
        let expected = 3;
        assert_eq!(Solution::min_k_bit_flips(nums, k), expected);
    }
}
