use std::collections::VecDeque;

///
/// # 3036. Number of Subarrays That Match a Pattern II
///
/// You are given a **0-indexed** integer array `nums` of size `n`, and a **0-indexed** integer array `pattern` of size `m` consisting of integers `-1`, `0`, and `1`.
///
/// A subarray `nums[i..j]` of size `m + 1` is said to match the `pattern` if the following conditions hold for each element `pattern[k]`:
///
/// * `nums[i + k + 1] > nums[i + k]` if `pattern[k] == 1`.
/// * `nums[i + k + 1] == nums[i + k]` if `pattern[k] == 0`.
/// * `nums[i + k + 1] < nums[i + k]` if `pattern[k] == -1`.
///
/// Return *the **count** of subarrays in* `nums` *that match the* `pattern`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,3,4,5,6], pattern = [1,1]
/// Output: 4
/// Explanation: The pattern [1,1] indicates that we are looking for strictly increasing subarrays of size 3. In the array nums, the subarrays [1,2,3], [2,3,4], [3,4,5], and [4,5,6] match this pattern.
/// Hence, there are 4 subarrays in nums that match the pattern.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,4,4,1,3,5,5,3], pattern = [1,0,-1]
/// Output: 2
/// Explanation: Here, the pattern [1,0,-1] indicates that we are looking for a sequence where the first number is smaller than the second, the second is equal to the third, and the third is greater than the fourth. In the array nums, the subarrays [1,4,4,1], and [3,5,5,3] match this pattern.
/// Hence, there are 2 subarrays in nums that match the pattern.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n == nums.length <= 10<sup>6</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `1 <= m == pattern.length < n`
/// * `-1 <= pattern[i] <= 1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-subarrays-that-match-a-pattern-ii/
// discuss: https://leetcode.com/problems/number-of-subarrays-that-match-a-pattern-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        const PATTERNS: i64 = 3;
        let mut size_pow_mod = 1;

        for _ in 0..pattern.len() - 1 {
            size_pow_mod = (size_pow_mod * PATTERNS) % MOD;
        }

        let mut window = VecDeque::new();
        let mut hash_window = 0;
        let mut hash_pattern = 0;

        let mut nums_iter = nums.windows(2).map(|x| (x[1] - x[0]).signum());

        for num in nums_iter.by_ref().take(pattern.len()) {
            window.push_back(num);
            hash_window = (PATTERNS * hash_window + num as i64 + 1) % MOD;
        }

        for &p in &pattern {
            hash_pattern = (PATTERNS * hash_pattern + p as i64 + 1) % MOD;
        }

        let mut result = 0;

        if hash_window == hash_pattern {
            result += 1;
        }

        for right in nums_iter {
            let left = window.pop_front().unwrap();
            window.push_back(right);

            hash_window =
                (PATTERNS * (hash_window + 2 * MOD - size_pow_mod * (left as i64 + 1)) + right as i64 + 1) % MOD;

            if hash_window == hash_pattern {
                result += 1;
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
    fn test_3036() {
        // let nums = vec![1, 2, 3, 4, 5, 6];
        // let pattern = vec![1, 1];
        // let expected = 4;
        // assert_eq!(Solution::count_matching_subarrays(nums, pattern), expected);
        // let nums = vec![1, 4, 4, 1, 3, 5, 5, 3];
        // let pattern = vec![1, 0, -1];
        // let expected = 2;
        // assert_eq!(Solution::count_matching_subarrays(nums, pattern), expected);
        let nums = vec![
            38798125, 38991139, 38991139, 44663141, 48097102, 44663141, 38991139, 38991139,
            38991139, 38798125, 38798125, 10980483, 38798125, 10980483, 38798125, 38798125,
            38798125, 38991139, 38798125, 38991139, 44663141, 44663141, 38991139, 38991139,
            38991139, 38798125, 38991139, 38991139, 38798125, 38798125, 38798125, 10980483,
            38798125, 38991139, 38798125, 10980483, 38798125, 38798125, 10980483, 10980483,
            10980483, 5743467, 5743467, 10980483, 5743467, 5743467, 5743467, 5743467, 10980483,
            38798125, 10980483, 10980483, 5743467, 5743467, 10980483, 10980483, 38798125, 38991139,
            44663141, 48097102, 48812719, 48812719, 58709950, 90054772, 58709950, 48812719,
            48812719, 48097102, 44663141, 48097102, 48812719, 48812719, 48812719, 48812719,
            58709950, 48812719, 58709950, 90054772, 101615965, 101615965, 101615965,
        ];
        let pattern = vec![
            1, 1, -1, -1, 0, 0, -1, 0, -1, 1, -1, 1, 0, 0, 1, -1, 1, 1, 0, -1, 0, 0, -1, 1, 0, -1,
            0, 0, -1, 1, 1, -1, -1, 1, 0, -1, 0, 0, -1, 0, 1, -1, 0, 0, 0, 1, 1, -1, 0, -1, 0, 1,
            0, 1, 1, 1, 1, 1, 0, 1, 1, -1, -1, 0, -1, -1, 1, 1, 0, 0, 0, 1, -1, 1, 1, 1, 0, 0,
        ];
        let expected = 1;
        assert_eq!(Solution::count_matching_subarrays(nums, pattern), expected);
    }
}
