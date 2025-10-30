///
/// # 3397. Maximum Number of Distinct Elements After Operations
///
/// You are given an integer array `nums` and an integer `k`.
///
/// You are allowed to perform the following **operation** on each element of the array **at most** *once*:
///
/// * Add an integer in the range `[-k, k]` to the element.
///
/// Return the **maximum** possible number of **distinct** elements in `nums` after performing the **operations**.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,2,3,3,4], k = 2
///
/// **Output:** 6
///
/// **Explanation:**
///
/// `nums` changes to `[-1, 0, 1, 2, 3, 4]` after performing operations on the first four elements.
///
/// **Example 2:**
///
/// **Input:** nums = [4,4,4,4], k = 1
///
/// **Output:** 3
///
/// **Explanation:**
///
/// By adding -1 to `nums[0]` and 1 to `nums[1]`, `nums` changes to `[3, 5, 4, 4]`.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `0 <= k <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/
// discuss: https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut result = 0;
        let mut prev = i32::MIN;

        for num in nums {
            if num + k <= prev {
                continue;
            }

            prev = (prev + 1).max(num - k);
            result += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3397() {
        let nums = vec![1, 2, 2, 3, 3, 4];
        let k = 2;
        let expected = 6;
        assert_eq!(Solution::max_distinct_elements(nums, k), expected);
        let nums = vec![4, 4, 4, 4];
        let k = 1;
        let expected = 3;
        assert_eq!(Solution::max_distinct_elements(nums, k), expected);
    }
}
