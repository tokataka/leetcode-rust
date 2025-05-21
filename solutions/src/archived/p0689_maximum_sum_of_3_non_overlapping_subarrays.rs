///
/// # 689. Maximum Sum of 3 Non-Overlapping Subarrays
///
/// Given an integer array `nums` and an integer `k`, find three non-overlapping subarrays of length `k` with maximum sum and return them.
///
/// Return the result as a list of indices representing the starting position of each interval (**0-indexed**). If there are multiple answers, return the lexicographically smallest one.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,1,2,6,7,5,1], k = 2
/// Output: [0,3,5]
/// Explanation: Subarrays [1, 2], [2, 6], [7, 5] correspond to the starting indices [0, 3, 5].
/// We could have also taken [2, 1], but an answer of [1, 3, 5] would be lexicographically smaller.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,1,2,1,2,1,2,1], k = 2
/// Output: [0,2,4]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 2 * 10<sup>4</sup>`
/// * `1 <= nums[i] < 2<sup>16</sup>`
/// * `1 <= k <= floor(nums.length / 3)`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/
// discuss: https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        let mut cache = vec![vec![(0, 0); 3]; n + 1];
        let mut window_sum = nums.iter().rev().take(k - 1).sum::<i32>();

        for i in (0..=n - k).rev() {
            window_sum += nums[i];

            for remain_idx in 0..3 {
                let cur = window_sum + cache[i + k].get(remain_idx + 1).map(|x| x.0).unwrap_or(0);

                cache[i][remain_idx] = match cur >= cache[i + 1][remain_idx].0 {
                    true => (cur, i),
                    false => cache[i + 1][remain_idx],
                };
            }

            window_sum -= nums[i + k - 1];
        }

        let mut cur = cache[0][0].1;

        let mut result = vec![cur as i32];

        for remain_idx in 1..3 {
            cur = cache[cur + k][remain_idx].1;
            result.push(cur as i32);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_689() {
        // let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
        // let k = 2;
        // let expected = vec![0, 3, 5];
        // assert_eq!(Solution::max_sum_of_three_subarrays(nums, k), expected);
        // let nums = vec![1, 2, 1, 2, 1, 2, 1, 2, 1];
        // let k = 2;
        // let expected = vec![0, 2, 4];
        // assert_eq!(Solution::max_sum_of_three_subarrays(nums, k), expected);
        // let nums = vec![4, 5, 10, 6, 11, 17, 4, 11, 1, 3];
        // let k = 1;
        // let expected = vec![4, 5, 7];
        // assert_eq!(Solution::max_sum_of_three_subarrays(nums, k), expected);
        let nums = vec![7, 13, 20, 19, 19, 2, 10, 1, 1, 19];
        let k = 3;
        let expected = vec![1, 4, 7];
        assert_eq!(Solution::max_sum_of_three_subarrays(nums, k), expected);
    }
}
