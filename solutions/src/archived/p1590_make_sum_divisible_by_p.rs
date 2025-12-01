///
/// # 1590. Make Sum Divisible by P
///
/// Given an array of positive integers `nums`, remove the **smallest** subarray (possibly **empty**) such that the **sum** of the remaining elements is divisible by `p`. It is **not** allowed to remove the whole array.
///
/// Return *the length of the smallest subarray that you need to remove, or* `-1` *if it's impossible*.
///
/// A **subarray** is defined as a contiguous block of elements in the array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [3,1,4,2], p = 6
/// Output: 1
/// Explanation: The sum of the elements in nums is 10, which is not divisible by 6. We can remove the subarray [4], and the sum of the remaining elements is 6, which is divisible by 6.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [6,3,5,2], p = 9
/// Output: 2
/// Explanation: We cannot remove a single element to get a sum divisible by 9. The best way is to remove the subarray [5,2], leaving us with [6,3] with sum 9.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [1,2,3], p = 3
/// Output: 0
/// Explanation: Here the sum is 6. which is already divisible by 3. Thus we do not need to remove anything.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `1 <= p <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/make-sum-divisible-by-p/
// discuss: https://leetcode.com/problems/make-sum-divisible-by-p/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut target = 0;

        for &num in &nums {
            target = (target + num) % p;
        }

        if target == 0 {
            return 0;
        }

        let mut occurence: HashMap<i32, usize> = HashMap::new();
        occurence.insert(0, 0);

        let mut sum = 0;
        let mut result = usize::MAX;

        for (i, &num) in nums.iter().enumerate() {
            sum = (sum + num) % p;

            if let Some(&prev_i) = occurence.get(&((sum + p - target) % p)) {
                result = result.min(i + 1 - prev_i);
            }

            occurence.insert(sum, i + 1);
        }

        if result == nums.len() {
            -1
        } else {
            result as i32
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1590() {
        assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
        assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 2);
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 3), 0);
    }
}
