///
/// # 3201. Find the Maximum Length of Valid Subsequence I
///
/// You are given an integer array `nums`.
///
/// A subsequence `sub` of `nums` with length `x` is called **valid** if it satisfies:
///
/// * `(sub[0] + sub[1]) % 2 == (sub[1] + sub[2]) % 2 == ... == (sub[x - 2] + sub[x - 1]) % 2.`
///
/// Return the length of the **longest** **valid** subsequence of `nums`.
///
/// A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,3,4]
///
/// **Output:** 4
///
/// **Explanation:**
///
/// The longest valid subsequence is `[1, 2, 3, 4]`.
///
/// **Example 2:**
///
/// **Input:** nums = [1,2,1,1,2,1,2]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// The longest valid subsequence is `[1, 2, 1, 2, 1, 2]`.
///
/// **Example 3:**
///
/// **Input:** nums = [1,3]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// The longest valid subsequence is `[1, 3]`.
///
/// **Constraints:**
///
/// * `2 <= nums.length <= 2 * 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>7</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/
// discuss: https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut alternate_even = 0;
        let mut alternate_odd = 0;
        let mut all_even = 0;
        let mut all_odd = 0;

        for num in nums {
            if num & 1 == 0 {
                alternate_even = alternate_even.max(alternate_odd + 1);
                all_even += 1;
            } else {
                alternate_odd = alternate_odd.max(alternate_even + 1);
                all_odd += 1;
            }
        }

        alternate_even.max(alternate_odd).max(all_even).max(all_odd)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3201() {
        let nums = vec![1, 2, 3, 4];
        let expected = 4;
        assert_eq!(Solution::maximum_length(nums), expected);
        let nums = vec![1, 2, 1, 1, 2, 1, 2];
        let expected = 6;
        assert_eq!(Solution::maximum_length(nums), expected);
        let nums = vec![1, 3];
        let expected = 2;
        assert_eq!(Solution::maximum_length(nums), expected);
    }
}
