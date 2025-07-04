///
/// # 2294. Partition Array Such That Maximum Difference Is K
///
/// You are given an integer array `nums` and an integer `k`. You may partition `nums` into one or more **subsequences** such that each element in `nums` appears in **exactly** one of the subsequences.
///
/// Return *the **minimum** number of subsequences needed such that the difference between the maximum and minimum values in each subsequence is **at most*** `k`*.*
///
/// A **subsequence** is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [3,6,1,2,5], k = 2
/// Output: 2
/// Explanation:
/// We can partition nums into the two subsequences [3,1,2] and [6,5].
/// The difference between the maximum and minimum value in the first subsequence is 3 - 1 = 2.
/// The difference between the maximum and minimum value in the second subsequence is 6 - 5 = 1.
/// Since two subsequences were created, we return 2. It can be shown that 2 is the minimum number of subsequences needed.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,3], k = 1
/// Output: 2
/// Explanation:
/// We can partition nums into the two subsequences [1,2] and [3].
/// The difference between the maximum and minimum value in the first subsequence is 2 - 1 = 1.
/// The difference between the maximum and minimum value in the second subsequence is 3 - 3 = 0.
/// Since two subsequences were created, we return 2. Note that another optimal solution is to partition nums into the two subsequences [1] and [2,3].
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [2,2,4,5], k = 0
/// Output: 3
/// Explanation:
/// We can partition nums into the three subsequences [2,2], [4], and [5].
/// The difference between the maximum and minimum value in the first subsequences is 2 - 2 = 0.
/// The difference between the maximum and minimum value in the second subsequences is 4 - 4 = 0.
/// The difference between the maximum and minimum value in the third subsequences is 5 - 5 = 0.
/// Since three subsequences were created, we return 3. It can be shown that 3 is the minimum number of subsequences needed.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `0 <= nums[i] <= 10<sup>5</sup>`
/// * `0 <= k <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/
// discuss: https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums.dedup();

        let n = nums.len();

        let mut result = 0;
        let mut idx = 0;

        while idx < n {
            result += 1;

            idx = nums.partition_point(|&x| x < nums[idx] + 1 + k);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2294() {
        let nums = vec![3, 6, 1, 2, 5];
        let k = 2;
        let expected = 2;
        assert_eq!(Solution::partition_array(nums, k), expected);
        let nums = vec![1, 2, 3];
        let k = 1;
        let expected = 2;
        assert_eq!(Solution::partition_array(nums, k), expected);
        let nums = vec![2, 2, 4, 5];
        let k = 0;
        let expected = 3;
        assert_eq!(Solution::partition_array(nums, k), expected);
    }
}
