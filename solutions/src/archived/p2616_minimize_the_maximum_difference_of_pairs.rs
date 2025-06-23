///
/// # 2616. Minimize the Maximum Difference of Pairs
///
/// You are given a **0-indexed** integer array `nums` and an integer `p`. Find `p` pairs of indices of `nums` such that the **maximum** difference amongst all the pairs is **minimized**. Also, ensure no index appears more than once amongst the `p` pairs.
///
/// Note that for a pair of elements at the index `i` and `j`, the difference of this pair is `|nums[i] - nums[j]|`, where `|x|` represents the **absolute** **value** of `x`.
///
/// Return *the **minimum** **maximum** difference among all* `p` *pairs.* We define the maximum of an empty set to be zero.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [10,1,2,7,1,3], p = 2
/// Output: 1
/// Explanation: The first pair is formed from the indices 1 and 4, and the second pair is formed from the indices 2 and 5.
/// The maximum difference is max(|nums[1] - nums[4]|, |nums[2] - nums[5]|) = max(0, 1) = 1. Therefore, we return 1.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [4,2,1,2], p = 1
/// Output: 0
/// Explanation: Let the indices 1 and 3 form a pair. The difference of that pair is |2 - 2| = 0, which is the minimum we can attain.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `0 <= nums[i] <= 10<sup>9</sup>`
/// * `0 <= p <= (nums.length)/2`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/
// discuss: https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        if p == 0 {
            return 0;
        }

        fn check_valid(mid: i32, num_diffs: &[i32], p: i32) -> bool {
            let mut cur = 0;
            let mut count = 0;

            while cur < num_diffs.len() {
                if num_diffs[cur] <= mid {
                    count += 1;
                    if count >= p {
                        return true;
                    }

                    cur += 2;
                } else {
                    cur += 1;
                }
            }

            false
        }

        nums.sort_unstable();

        let mut num_diffs = Vec::with_capacity(nums.len() - 1);
        let mut max_diff = 0;

        for x in nums.windows(2) {
            let diff = (x[0] - x[1]).abs();
            num_diffs.push(diff);
            max_diff = max_diff.max(diff);
        }

        let mut left = 0;
        let mut right = max_diff;

        while left < right {
            let mid = (right + left) / 2;

            match check_valid(mid, &num_diffs, p) {
                true => right = mid,
                false => left = mid + 1,
            }
        }

        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2616() {
        // let nums = vec![10, 1, 2, 7, 1, 3];
        // let p = 2;
        // let expected = 1;
        // assert_eq!(Solution::minimize_max(nums, p), expected);
        // let nums = vec![4, 2, 1, 2];
        // let p = 1;
        // let expected = 0;
        // assert_eq!(Solution::minimize_max(nums, p), expected);
        let nums = vec![2, 6, 2, 4, 2, 2, 0, 2];
        let p = 4;
        let expected = 2;
        assert_eq!(Solution::minimize_max(nums, p), expected);
    }
}
