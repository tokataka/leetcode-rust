///
/// # 2302. Count Subarrays With Score Less Than K
///
/// The **score** of an array is defined as the **product** of its sum and its length.
///
/// * For example, the score of `[1, 2, 3, 4, 5]` is `(1 + 2 + 3 + 4 + 5) * 5 = 75`.
///
/// Given a positive integer array `nums` and an integer `k`, return *the **number of non-empty subarrays** of* `nums` *whose score is **strictly less** than* `k`.
///
/// A **subarray** is a contiguous sequence of elements within an array.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,1,4,3,5], k = 10
/// Output: 6
/// Explanation:
/// The 6 subarrays having scores less than 10 are:
/// - [2] with score 2 * 1 = 2.
/// - [1] with score 1 * 1 = 1.
/// - [4] with score 4 * 1 = 4.
/// - [3] with score 3 * 1 = 3.
/// - [5] with score 5 * 1 = 5.
/// - [2,1] with score (2 + 1) * 2 = 6.
/// Note that subarrays such as [1,4] and [4,3,5] are not considered because their scores are 10 and 36 respectively, while we need scores strictly less than 10.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,1,1], k = 5
/// Output: 5
/// Explanation:
/// Every subarray except [1,1,1] has a score less than 5.
/// [1,1,1] has a score (1 + 1 + 1) * 3 = 9, which is greater than 5.
/// Thus, there are 5 subarrays having scores less than 5.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>5</sup>`
/// * `1 <= k <= 10<sup>15</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-subarrays-with-score-less-than-k/
// discuss: https://leetcode.com/problems/count-subarrays-with-score-less-than-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut result = 0;

        let mut cur_sum = 0;
        let mut cur_len = 0;
        let mut left_iter = nums.iter();

        for &right in nums.iter() {
            cur_sum += right as i64;
            cur_len += 1;

            while cur_sum * cur_len >= k {
                cur_sum -= *left_iter.next().unwrap() as i64;
                cur_len -= 1;
            }

            result += cur_len;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2302() {
        let nums = vec![2, 1, 4, 3, 5];
        let k = 10;
        let expected = 6;
        assert_eq!(Solution::count_subarrays(nums, k), expected);
        let nums = vec![1, 1, 1];
        let k = 5;
        let expected = 5;
        assert_eq!(Solution::count_subarrays(nums, k), expected);
    }
}
