///
/// # 1262. Greatest Sum Divisible by Three
///
/// Given an integer array `nums`, return *the **maximum possible sum** of elements of the array such that it is divisible by three*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [3,6,5,1,8]
/// Output: 18
/// Explanation: Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3).
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [4]
/// Output: 0
/// Explanation: Since 4 is not divisible by 3, do not pick any number.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [1,2,3,4,4]
/// Output: 12
/// Explanation: Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3).
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 4 * 10<sup>4</sup>`
/// * `1 <= nums[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/greatest-sum-divisible-by-three/
// discuss: https://leetcode.com/problems/greatest-sum-divisible-by-three/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = [0, i32::MIN, i32::MIN];

        for num in nums {
            for x in dp {
                let i = (x + num) as usize % 3;
                dp[i] = dp[i].max(x + num);
            }
        }

        dp[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1262() {
        assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
        assert_eq!(Solution::max_sum_div_three(vec![4]), 0);
        assert_eq!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
    }
}
