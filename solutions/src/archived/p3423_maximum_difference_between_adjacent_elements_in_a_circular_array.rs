///
/// # 3423. Maximum Difference Between Adjacent Elements in a Circular Array
///
/// Given a **circular** array `nums`, find the **maximum** absolute difference between adjacent elements.
///
/// **Note**: In a circular array, the first and last elements are adjacent.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,4]
///
/// **Output:** 3
///
/// **Explanation:**
///
/// Because `nums` is circular, `nums[0]` and `nums[2]` are adjacent. They have the maximum absolute difference of `|4 - 1| = 3`.
///
/// **Example 2:**
///
/// **Input:** nums = [-5,-10,-5]
///
/// **Output:** 5
///
/// **Explanation:**
///
/// The adjacent elements `nums[0]` and `nums[1]` have the maximum absolute difference of `|-5 - (-10)| = 5`.
///
/// **Constraints:**
///
/// * `2 <= nums.length <= 100`
/// * `-100 <= nums[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/
// discuss: https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_adjacent_distance(mut nums: Vec<i32>) -> i32 {
        nums.push(nums[0]);
        nums.windows(2).map(|x| (x[0] - x[1]).abs()).max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3423() {
        let nums = vec![1, 2, 4];
        let expected = 3;
        assert_eq!(Solution::max_adjacent_distance(nums), expected);
        let nums = vec![-5, -10, -5];
        let expected = 5;
        assert_eq!(Solution::max_adjacent_distance(nums), expected);
    }
}
