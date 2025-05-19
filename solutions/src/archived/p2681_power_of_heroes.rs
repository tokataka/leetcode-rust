///
/// # 2681. Power of Heroes
///
/// You are given a **0-indexed** integer array `nums` representing the strength of some heroes. The **power** of a group of heroes is defined as follows:
///
/// * Let `i<sub>0</sub>`, `i<sub>1</sub>`, ... ,`i<sub>k</sub>` be the indices of the heroes in a group. Then, the power of this group is `max(nums[i<sub>0</sub>], nums[i<sub>1</sub>], ... ,nums[i<sub>k</sub>])<sup>2</sup> * min(nums[i<sub>0</sub>], nums[i<sub>1</sub>], ... ,nums[i<sub>k</sub>])`.
///
/// Return *the sum of the **power** of all **non-empty** groups of heroes possible.* Since the sum could be very large, return it **modulo** `10<sup>9 </sup>+ 7`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,1,4]
/// Output: 141
/// Explanation:
/// 1st group: [2] has power = 22 * 2 = 8.
/// 2nd group: [1] has power = 12 * 1 = 1.
/// 3rd group: [4] has power = 42 * 4 = 64.
/// 4th group: [2,1] has power = 22 * 1 = 4.
/// 5th group: [2,4] has power = 42 * 2 = 32.
/// 6th group: [1,4] has power = 42 * 1 = 16.
/// ​​​​​​​7th group: [2,1,4] has power = 42​​​​​​​ * 1 = 16.
/// The sum of powers of all groups is 8 + 1 + 64 + 4 + 32 + 16 + 16 = 141.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,1,1]
/// Output: 7
/// Explanation: A total of 7 groups are possible, and the power of each group will be 1. Therefore, the sum of the powers of all groups is 7.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/power-of-heroes/
// discuss: https://leetcode.com/problems/power-of-heroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        nums.sort_unstable();

        let mut result = 0;
        let mut left = 0;

        for &num in nums.iter() {
            let num = num as i64;

            result = (result + (left + num) * (num * num % MOD)) % MOD;
            left = (2 * left + num) % MOD;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2681() {
        let nums = vec![2, 1, 4];
        let expected = 141;
        assert_eq!(Solution::sum_of_power(nums), expected);
        let nums = vec![1, 1, 1];
        let expected = 7;
        assert_eq!(Solution::sum_of_power(nums), expected);
    }
}
