///
/// # 442. Find All Duplicates in an Array
///
/// Given an integer array `nums` of length `n` where all the integers of `nums` are in the range `[1, n]` and each integer appears **at most** **twice**, return *an array of all the integers that appears **twice***.
///
/// You must write an algorithm that runs in `O(n)` time and uses only *constant* auxiliary space, excluding the space needed to store the output
///
/// **Example 1:**
///
/// ```
/// Input: nums = [4,3,2,7,8,2,3,1]
/// Output: [2,3]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,1,2]
/// Output: [1]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [1]
/// Output: []
///
/// ```
///
/// **Constraints:**
///
/// * `n == nums.length`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= n`
/// * Each element in `nums` appears **once** or **twice**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-duplicates-in-an-array/
// discuss: https://leetcode.com/problems/find-all-duplicates-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let d = nums.len() as i32 + 1;

        let mut result = vec![];

        for i in 0..nums.len() {
            let cur = (nums[i] % d - 1) as usize;

            nums[cur] += d;

            if nums[cur] / d == 2 {
                result.push(nums[i] % d);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_442() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let expected = vec![2, 3];
        assert_eq!(Solution::find_duplicates(nums), expected);
        let nums = vec![1, 1, 2];
        let expected = vec![1];
        assert_eq!(Solution::find_duplicates(nums), expected);
        let nums = vec![1];
        let expected = vec![];
        assert_eq!(Solution::find_duplicates(nums), expected);
    }
}
