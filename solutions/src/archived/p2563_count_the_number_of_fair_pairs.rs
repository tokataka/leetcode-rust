///
/// # 2563. Count the Number of Fair Pairs
///
/// Given a **0-indexed** integer array `nums` of size `n` and two integers `lower` and `upper`, return *the number of fair pairs*.
///
/// A pair `(i, j)` is **fair** if:
///
/// * `0 <= i < j < n`, and
/// * `lower <= nums[i] + nums[j] <= upper`
///
/// **Example 1:**
///
/// ```
/// Input: nums = [0,1,7,4,4,5], lower = 3, upper = 6
/// Output: 6
/// Explanation: There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,7,9,2,5], lower = 11, upper = 11
/// Output: 1
/// Explanation: There is a single fair pair: (2,3).
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `nums.length == n`
/// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
/// * `-10<sup>9</sup> <= lower <= upper <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-fair-pairs/
// discuss: https://leetcode.com/problems/count-the-number-of-fair-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();

        let mut count = 0;

        for (i, &left) in nums.iter().enumerate() {
            let l = nums[i + 1..].partition_point(|&x| x < lower - left);
            let u = nums[i + 1..].partition_point(|&x| x <= upper - left);

            if u > l {
                count += (u - l) as i64;
            }
        }

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2563() {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let lower = 3;
        let upper = 6;
        let expected = 6;
        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), expected);
        let nums = vec![1, 7, 9, 2, 5];
        let lower = 11;
        let upper = 11;
        let expected = 1;
        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), expected);
    }
}
