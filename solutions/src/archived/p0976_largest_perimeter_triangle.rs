///
/// # 976. Largest Perimeter Triangle
///
/// Given an integer array `nums`, return *the largest perimeter of a triangle with a non-zero area, formed from three of these lengths*. If it is impossible to form any triangle of a non-zero area, return `0`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,1,2]
/// Output: 5
/// Explanation: You can form a triangle with three side lengths: 1, 2, and 2.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,1,10]
/// Output: 0
/// Explanation:
/// You cannot use the side lengths 1, 1, and 2 to form a triangle.
/// You cannot use the side lengths 1, 1, and 10 to form a triangle.
/// You cannot use the side lengths 1, 2, and 10 to form a triangle.
/// As we cannot use any three side lengths to form a triangle of non-zero area, we return 0.
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= nums.length <= 10<sup>4</sup>`
/// * `1 <= nums[i] <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-perimeter-triangle/
// discuss: https://leetcode.com/problems/largest-perimeter-triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        nums.windows(3)
            .rev()
            .find_map(|x| (x[2] < x[0] + x[1]).then_some(x[0] + x[1] + x[2]))
            .unwrap_or(0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_976() {
        let nums = vec![2, 1, 2];
        let expected = 5;
        assert_eq!(Solution::largest_perimeter(nums), expected);
        let nums = vec![1, 2, 1, 10];
        let expected = 0;
        assert_eq!(Solution::largest_perimeter(nums), expected);
    }
}
