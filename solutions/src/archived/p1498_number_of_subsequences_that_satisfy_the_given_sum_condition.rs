///
/// # 1498. Number of Subsequences That Satisfy the Given Sum Condition
///
/// You are given an array of integers `nums` and an integer `target`.
///
/// Return *the number of **non-empty** subsequences of* `nums` *such that the sum of the minimum and maximum element on it is less or equal to* `target`. Since the answer may be too large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [3,5,6,7], target = 9
/// Output: 4
/// Explanation: There are 4 subsequences that satisfy the condition.
/// [3] -> Min value + max value <= target (3 + 3 <= 9)
/// [3,5] -> (3 + 5 <= 9)
/// [3,5,6] -> (3 + 6 <= 9)
/// [3,6] -> (3 + 6 <= 9)
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [3,3,6,8], target = 10
/// Output: 6
/// Explanation: There are 6 subsequences that satisfy the condition. (nums can have repeated numbers).
/// [3] , [3] , [3,3], [3,6] , [3,6] , [3,3,6]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [2,3,3,4,6,7], target = 12
/// Output: 61
/// Explanation: There are 63 non-empty subsequences, two of them do not satisfy the condition ([6,7], [7]).
/// Number of valid subsequences (63 - 2 = 61).
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>6</sup>`
/// * `1 <= target <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/
// discuss: https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

fn powmod(mut a: i64, mut b: i64, p: i64) -> i64 {
    let mut result = 1;

    while b > 0 {
        if b & 1 == 1 {
            result = (result * a) % p;
        }

        a = (a * a) % p;
        b >>= 1;
    }

    result
}

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        nums.sort_unstable();

        let mut result = 0;

        for (i, &num) in nums.iter().enumerate() {
            let count = nums[i..].partition_point(|&x| x <= target - num) as i64;

            if count > 0 {
                result = (result + powmod(2, count - 1, MOD)) % MOD;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1498() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        let expected = 4;
        assert_eq!(Solution::num_subseq(nums, target), expected);
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        let expected = 6;
        assert_eq!(Solution::num_subseq(nums, target), expected);
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        let expected = 61;
        assert_eq!(Solution::num_subseq(nums, target), expected);
    }
}
