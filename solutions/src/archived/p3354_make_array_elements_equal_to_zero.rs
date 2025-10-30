///
/// # 3354. Make Array Elements Equal to Zero
///
/// You are given an integer array `nums`.
///
/// Start by selecting a starting position `curr` such that `nums[curr] == 0`, and choose a movement **direction** of either left or right.
///
/// After that, you repeat the following process:
///
/// * If `curr` is out of the range `[0, n - 1]`, this process ends.
/// * If `nums[curr] == 0`, move in the current direction by **incrementing** `curr` if you are moving right, or **decrementing** `curr` if you are moving left.
/// * Else if `nums[curr] > 0`:
///   * Decrement `nums[curr]` by 1.
///   * **Reverse** your movement direction (left becomes right and vice versa).
///   * Take a step in your new direction.
///
/// A selection of the initial position `curr` and movement direction is considered **valid** if every element in `nums` becomes 0 by the end of the process.
///
/// Return the number of possible **valid** selections.
///
/// **Example 1:**
///
/// **Input:** nums = [1,0,2,0,3]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// The only possible valid selections are the following:
///
/// * Choose `curr = 3`, and a movement direction to the left.
///   * `[1,0,2,**0**,3] -> [1,0,**2**,0,3] -> [1,0,1,**0**,3] -> [1,0,1,0,**3**] -> [1,0,1,**0**,2] -> [1,0,**1**,0,2] -> [1,0,0,**0**,2] -> [1,0,0,0,**2**] -> [1,0,0,**0**,1] -> [1,0,**0**,0,1] -> [1,**0**,0,0,1] -> [**1**,0,0,0,1] -> [0,**0**,0,0,1] -> [0,0,**0**,0,1] -> [0,0,0,**0**,1] -> [0,0,0,0,**1**] -> [0,0,0,0,0]`.
///
/// * Choose `curr = 3`, and a movement direction to the right.
///   * `[1,0,2,**0**,3] -> [1,0,2,0,**3**] -> [1,0,2,**0**,2] -> [1,0,**2**,0,2] -> [1,0,1,**0**,2] -> [1,0,1,0,**2**] -> [1,0,1,**0**,1] -> [1,0,**1**,0,1] -> [1,0,0,**0**,1] -> [1,0,0,0,**1**] -> [1,0,0,**0**,0] -> [1,0,**0**,0,0] -> [1,**0**,0,0,0] -> [**1**,0,0,0,0] -> [0,0,0,0,0].`
///
/// **Example 2:**
///
/// **Input:** nums = [2,3,4,0,4,1,0]
///
/// **Output:** 0
///
/// **Explanation:**
///
/// There are no possible valid selections.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 100`
/// * `0 <= nums[i] <= 100`
/// * There is at least one element `i` where `nums[i] == 0`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/make-array-elements-equal-to-zero/
// discuss: https://leetcode.com/problems/make-array-elements-equal-to-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut right = nums.iter().sum::<i32>();
        let mut left = 0;

        for num in nums {
            if num == 0 {
                match left - right {
                    -1 | 1 => result += 1,
                    0 => result += 2,
                    _ => (),
                };
            } else {
                left += num;
                right -= num;
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
    fn test_3354() {
        let nums = vec![1, 0, 2, 0, 3];
        let expected = 2;
        assert_eq!(Solution::count_valid_selections(nums), expected);
        let nums = vec![2, 3, 4, 0, 4, 1, 0];
        let expected = 0;
        assert_eq!(Solution::count_valid_selections(nums), expected);
    }
}
