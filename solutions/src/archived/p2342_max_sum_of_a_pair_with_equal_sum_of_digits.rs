///
/// # 2342. Max Sum of a Pair With Equal Sum of Digits
///
/// You are given a **0-indexed** array `nums` consisting of **positive** integers. You can choose two indices `i` and `j`, such that `i != j`, and the sum of digits of the number `nums[i]` is equal to that of `nums[j]`.
///
/// Return *the **maximum** value of* `nums[i] + nums[j]` *that you can obtain over all possible indices* `i` *and* `j` *that satisfy the conditions.*
///
/// **Example 1:**
///
/// ```
/// Input: nums = [18,43,36,13,7]
/// Output: 54
/// Explanation: The pairs (i, j) that satisfy the conditions are:
/// - (0, 2), both numbers have a sum of digits equal to 9, and their sum is 18 + 36 = 54.
/// - (1, 4), both numbers have a sum of digits equal to 7, and their sum is 43 + 7 = 50.
/// So the maximum sum that we can obtain is 54.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [10,12,19,14]
/// Output: -1
/// Explanation: There are no two numbers that satisfy the conditions, so we return -1.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
// discuss: https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut digit_sum_max = [0; 100];

        for num in nums {
            let mut digit_sum = 0;
            let mut cur = num;
            while cur > 0 {
                digit_sum += cur % 10;
                cur /= 10;
            }

            let prev_max = digit_sum_max[digit_sum as usize];

            if prev_max > 0 {
                result = result.max(prev_max + num);
            }

            digit_sum_max[digit_sum as usize] = prev_max.max(num);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2342() {
        let nums = vec![18, 43, 36, 13, 7];
        let expected = 54;
        assert_eq!(Solution::maximum_sum(nums), expected);
        let nums = vec![10, 12, 19, 14];
        let expected = -1;
        assert_eq!(Solution::maximum_sum(nums), expected);
    }
}
