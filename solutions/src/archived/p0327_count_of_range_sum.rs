use std::collections::BTreeSet;

///
/// # 327. Count of Range Sum
///
/// Given an integer array `nums` and two integers `lower` and `upper`, return *the number of range sums that lie in* `[lower, upper]` *inclusive*.
///
/// Range sum `S(i, j)` is defined as the sum of the elements in `nums` between indices `i` and `j` inclusive, where `i <= j`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [-2,5,-1], lower = -2, upper = 2
/// Output: 3
/// Explanation: The three ranges are: [0,0], [2,2], and [0,2] and their respective sums are: -2, -1, 2.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [0], lower = 0, upper = 0
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1`
/// * `-10<sup>5</sup> <= lower <= upper <= 10<sup>5</sup>`
/// * The answer is **guaranteed** to fit in a **32-bit** integer.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-of-range-sum/
// discuss: https://leetcode.com/problems/count-of-range-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let lower = lower as i64;
        let upper = upper as i64;

        let mut prefix_sum = vec![0];

        for num in nums {
            prefix_sum.push(num as i64 + prefix_sum.last().unwrap());
        }

        let mut result = 0;
        let mut right = BTreeSet::new();

        for i in (0..n).rev() {
            let cur = prefix_sum[i];

            right.insert((prefix_sum[i + 1], i));

            result += right.range((lower + cur, 0)..(upper + cur + 1, 0)).count();
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_327() {
        let nums = vec![-2, 5, -1];
        let lower = -2;
        let upper = 2;
        let expected = 3;
        assert_eq!(Solution::count_range_sum(nums, lower, upper), expected);
        let nums = vec![0];
        let lower = 0;
        let upper = 0;
        let expected = 1;
        assert_eq!(Solution::count_range_sum(nums, lower, upper), expected);
    }
}
