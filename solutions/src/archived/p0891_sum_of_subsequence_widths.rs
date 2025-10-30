///
/// # 891. Sum of Subsequence Widths
///
/// The **width** of a sequence is the difference between the maximum and minimum elements in the sequence.
///
/// Given an array of integers `nums`, return *the sum of the **widths** of all the non-empty **subsequences** of* `nums`. Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// A **subsequence** is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements. For example, `[3,6,2,7]` is a subsequence of the array `[0,3,1,6,2,2,7]`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,1,3]
/// Output: 6
/// Explanation: The subsequences are [1], [2], [3], [2,1], [2,3], [1,3], [2,1,3].
/// The corresponding widths are 0, 0, 0, 1, 1, 2, 2.
/// The sum of these widths is 6.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [2]
/// Output: 0
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-subsequence-widths/
// discuss: https://leetcode.com/problems/sum-of-subsequence-widths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut nums = nums;
        nums.sort_unstable();

        let mut result = 0;
        let mut prev_sum = 0;
        let mut pow_2 = 1;

        for &num in &nums {
            let num = num as i64;

            result = (result + (pow_2 - 1) * num - prev_sum) % MOD;
            prev_sum = ((prev_sum << 1) + num) % MOD;
            pow_2 = (pow_2 << 1) % MOD;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_891() {
        let nums = vec![2, 1, 3];
        let expected = 6;
        assert_eq!(Solution::sum_subseq_widths(nums), expected);
        let nums = vec![2];
        let expected = 0;
        assert_eq!(Solution::sum_subseq_widths(nums), expected);
        let nums = vec![3, 7, 2, 3];
        let expected = 35;
        assert_eq!(Solution::sum_subseq_widths(nums), expected);
    }
}
