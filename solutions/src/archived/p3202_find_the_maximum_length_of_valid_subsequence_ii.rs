///
/// # 3202. Find the Maximum Length of Valid Subsequence II
///
/// You are given an integer array `nums` and a **positive** integer `k`.
///
/// A subsequence `sub` of `nums` with length `x` is called **valid** if it satisfies:
///
/// * `(sub[0] + sub[1]) % k == (sub[1] + sub[2]) % k == ... == (sub[x - 2] + sub[x - 1]) % k.`
///
/// Return the length of the **longest** **valid** subsequence of `nums`.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,3,4,5], k = 2
///
/// **Output:** 5
///
/// **Explanation:**
///
/// The longest valid subsequence is `[1, 2, 3, 4, 5]`.
///
/// **Example 2:**
///
/// **Input:** nums = [1,4,2,3,1,4], k = 3
///
/// **Output:** 4
///
/// **Explanation:**
///
/// The longest valid subsequence is `[1, 4, 1, 4]`.
///
/// **Constraints:**
///
/// * `2 <= nums.length <= 10<sup>3</sup>`
/// * `1 <= nums[i] <= 10<sup>7</sup>`
/// * `1 <= k <= 10<sup>3</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/
// discuss: https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut result = 0;
        let mut dp = vec![0; k];

        for mod_sum in 0..k {
            dp.fill(0);
            dp[nums[0] as usize % k] = 1;

            for &num in nums.iter().skip(1) {
                let cur = num as usize % k;
                let prev = (mod_sum + k - cur) % k;

                dp[cur] = dp[cur].max(dp[prev] + 1);
                result = result.max(dp[cur]);
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
    fn test_3202() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;
        let expected = 5;
        assert_eq!(Solution::maximum_length(nums, k), expected);
        let nums = vec![1, 4, 2, 3, 1, 4];
        let k = 3;
        let expected = 4;
        assert_eq!(Solution::maximum_length(nums, k), expected);
    }
}
