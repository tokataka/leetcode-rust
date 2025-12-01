///
/// # 3318. Find X-Sum of All K-Long Subarrays I
///
/// You are given an array `nums` of `n` integers and two integers `k` and `x`.
///
/// The **x-sum** of an array is calculated by the following procedure:
///
/// * Count the occurrences of all elements in the array.
/// * Keep only the occurrences of the top `x` most frequent elements. If two elements have the same number of occurrences, the element with the **bigger** value is considered more frequent.
/// * Calculate the sum of the resulting array.
///
/// **Note** that if an array has less than `x` distinct elements, its **x-sum** is the sum of the array.
///
/// Return an integer array `answer` of length `n - k + 1` where `answer[i]` is the **x-sum** of the subarray `nums[i..i + k - 1]`.
///
/// **Example 1:**
///
/// **Input:** nums = [1,1,2,2,3,4,2,3], k = 6, x = 2
///
/// **Output:** [6,10,12]
///
/// **Explanation:**
///
/// * For subarray `[1, 1, 2, 2, 3, 4]`, only elements 1 and 2 will be kept in the resulting array. Hence, `answer[0] = 1 + 1 + 2 + 2`.
/// * For subarray `[1, 2, 2, 3, 4, 2]`, only elements 2 and 4 will be kept in the resulting array. Hence, `answer[1] = 2 + 2 + 2 + 4`. Note that 4 is kept in the array since it is bigger than 3 and 1 which occur the same number of times.
/// * For subarray `[2, 2, 3, 4, 2, 3]`, only elements 2 and 3 are kept in the resulting array. Hence, `answer[2] = 2 + 2 + 2 + 3 + 3`.
///
/// **Example 2:**
///
/// **Input:** nums = [3,8,7,8,7,5], k = 2, x = 2
///
/// **Output:** [11,15,15,15,12]
///
/// **Explanation:**
///
/// Since `k == x`, `answer[i]` is equal to the sum of the subarray `nums[i..i + k - 1]`.
///
/// **Constraints:**
///
/// * `1 <= n == nums.length <= 50`
/// * `1 <= nums[i] <= 50`
/// * `1 <= x <= k <= nums.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/
// discuss: https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::BTreeSet;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let x = x as usize;

        let mut freq = vec![0; 51];
        let mut ordered_set = BTreeSet::new();

        for &x in nums.iter().take(k - 1) {
            ordered_set.remove(&(freq[x as usize], x));
            freq[x as usize] += 1;
            ordered_set.insert((freq[x as usize], x));
        }

        let mut result = vec![];

        for (&cur, &prev) in nums.iter().skip(k - 1).zip(&nums) {
            ordered_set.remove(&(freq[cur as usize], cur));
            freq[cur as usize] += 1;
            ordered_set.insert((freq[cur as usize], cur));

            result.push(ordered_set.iter().rev().take(x).map(|a| a.0 * a.1).sum());

            ordered_set.remove(&(freq[prev as usize], prev));
            freq[prev as usize] -= 1;
            ordered_set.insert((freq[prev as usize], prev));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3318() {
        let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
        let k = 6;
        let x = 2;
        let expected = vec![6, 10, 12];
        assert_eq!(Solution::find_x_sum(nums, k, x), expected);
        let nums = vec![3, 8, 7, 8, 7, 5];
        let k = 2;
        let x = 2;
        let expected = vec![11, 15, 15, 15, 12];
        assert_eq!(Solution::find_x_sum(nums, k, x), expected);
    }
}
