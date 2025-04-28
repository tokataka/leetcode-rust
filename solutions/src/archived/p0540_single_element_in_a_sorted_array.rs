///
/// # 540. Single Element in a Sorted Array
///
/// You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once.
///
/// Return *the single element that appears only once*.
///
/// Your solution must run in `O(log n)` time and `O(1)` space.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,1,2,3,3,4,4,8,8]
/// Output: 2
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [3,3,7,7,10,11,11]
/// Output: 10
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `0 <= nums[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/single-element-in-a-sorted-array/
// discuss: https://leetcode.com/problems/single-element-in-a-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        if right == 1 {
            return nums[0];
        }

        while right - left > 3 {
            let mut mid = (left + right) / 2;

            if (mid - left) % 2 == 1 {
                mid += 1;
            }

            if nums[mid - 1] == nums[mid - 2] {
                left = mid;
            } else {
                right = mid + 1;
            }
        }

        if nums[left] != nums[left + 1] {
            nums[left]
        } else {
            nums[right - 1]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_540() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        let expected = 2;
        assert_eq!(Solution::single_non_duplicate(nums), expected);
        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        let expected = 10;
        assert_eq!(Solution::single_non_duplicate(nums), expected);
    }
}
