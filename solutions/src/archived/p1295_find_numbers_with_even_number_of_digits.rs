///
/// # 1295. Find Numbers with Even Number of Digits
///
/// Given an array `nums` of integers, return how many of them contain an **even number** of digits.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [12,345,2,6,7896]
/// Output: 2
/// Explanation:
/// 12 contains 2 digits (even number of digits).
/// 345 contains 3 digits (odd number of digits).
/// 2 contains 1 digit (odd number of digits).
/// 6 contains 1 digit (odd number of digits).
/// 7896 contains 4 digits (even number of digits).
/// Therefore only 12 and 7896 contain an even number of digits.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [555,901,482,1771]
/// Output: 1
/// Explanation:
/// Only 1771 contains an even number of digits.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 500`
/// * `1 <= nums[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
// discuss: https://leetcode.com/problems/find-numbers-with-even-number-of-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter().filter(|&x| x.ilog10() % 2 == 1).count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1295() {
        let nums = vec![12, 345, 2, 6, 7896];
        let expected = 2;
        assert_eq!(Solution::find_numbers(nums), expected);
        let nums = vec![555, 901, 482, 1771];
        let expected = 1;
        assert_eq!(Solution::find_numbers(nums), expected);
    }
}
