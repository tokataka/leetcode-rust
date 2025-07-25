///
/// # 3487. Maximum Unique Subarray Sum After Deletion
///
/// You are given an integer array `nums`.
///
/// You are allowed to delete any number of elements from `nums` without making it **empty**. After performing the deletions, select a subarray of `nums` such that:
///
/// 1. All elements in the subarray are **unique**.
/// 2. The sum of the elements in the subarray is **maximized**.
///
/// Return the **maximum sum** of such a subarray.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,3,4,5]
///
/// **Output:** 15
///
/// **Explanation:**
///
/// Select the entire array without deleting any element to obtain the maximum sum.
///
/// **Example 2:**
///
/// **Input:** nums = [1,1,0,1,1]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// Delete the element `nums[0] == 1`, `nums[1] == 1`, `nums[2] == 0`, and `nums[3] == 1`. Select the entire array `[1]` to obtain the maximum sum.
///
/// **Example 3:**
///
/// **Input:** nums = [1,2,-1,-2,1,0,-1]
///
/// **Output:** 3
///
/// **Explanation:**
///
/// Delete the elements `nums[2] == -1` and `nums[3] == -2`, and select the subarray `[2, 1]` from `[1, 2, 1, 0, -1]` to obtain the maximum sum.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 100`
/// * `-100 <= nums[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion/
// discuss: https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut used = [false; 101];
        let mut sum = 0;
        let mut max = -101;

        for &num in &nums {
            max = max.max(num);

            if num <= 0 || used[num as usize] {
                continue;
            }

            sum += num;
            used[num as usize] = true;
        }

        if max <= 0 {
            max
        } else {
            sum
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3487() {
        let nums = vec![1, 2, 3, 4, 5];
        let expected = 15;
        assert_eq!(Solution::max_sum(nums), expected);
        let nums = vec![1, 1, 0, 1, 1];
        let expected = 1;
        assert_eq!(Solution::max_sum(nums), expected);
        let nums = vec![1, 2, -1, -2, 1, 0, -1];
        let expected = 3;
        assert_eq!(Solution::max_sum(nums), expected);
    }
}
