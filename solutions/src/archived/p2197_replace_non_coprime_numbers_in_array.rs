///
/// # 2197. Replace Non-Coprime Numbers in Array
///
/// You are given an array of integers `nums`. Perform the following steps:
///
/// 1. Find **any** two **adjacent** numbers in `nums` that are **non-coprime**.
/// 2. If no such numbers are found, **stop** the process.
/// 3. Otherwise, delete the two numbers and **replace** them with their **LCM (Least Common Multiple)**.
/// 4. **Repeat** this process as long as you keep finding two adjacent non-coprime numbers.
///
/// Return *the **final** modified array.* It can be shown that replacing adjacent non-coprime numbers in **any** arbitrary order will lead to the same result.
///
/// The test cases are generated such that the values in the final array are **less than or equal** to `10<sup>8</sup>`.
///
/// Two values `x` and `y` are **non-coprime** if `GCD(x, y) > 1` where `GCD(x, y)` is the **Greatest Common Divisor** of `x` and `y`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [6,4,3,2,7,6,2]
/// Output: [12,7,6]
/// Explanation:
/// - (6, 4) are non-coprime with LCM(6, 4) = 12. Now, nums = [12,3,2,7,6,2].
/// - (12, 3) are non-coprime with LCM(12, 3) = 12. Now, nums = [12,2,7,6,2].
/// - (12, 2) are non-coprime with LCM(12, 2) = 12. Now, nums = [12,7,6,2].
/// - (6, 2) are non-coprime with LCM(6, 2) = 6. Now, nums = [12,7,6].
/// There are no more adjacent non-coprime numbers in nums.
/// Thus, the final modified array is [12,7,6].
/// Note that there are other ways to obtain the same resultant array.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [2,2,1,1,3,3,3]
/// Output: [2,1,1,3]
/// Explanation:
/// - (3, 3) are non-coprime with LCM(3, 3) = 3. Now, nums = [2,2,1,1,3,3].
/// - (3, 3) are non-coprime with LCM(3, 3) = 3. Now, nums = [2,2,1,1,3].
/// - (2, 2) are non-coprime with LCM(2, 2) = 2. Now, nums = [2,1,1,3].
/// There are no more adjacent non-coprime numbers in nums.
/// Thus, the final modified array is [2,1,1,3].
/// Note that there are other ways to obtain the same resultant array.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>5</sup>`
/// * The test cases are generated such that the values in the final array are **less than or equal** to `10<sup>8</sup>`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/replace-non-coprime-numbers-in-array/
// discuss: https://leetcode.com/problems/replace-non-coprime-numbers-in-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn non_coprime(a: i32, b: i32) -> Option<i32> {
            let mut x = a;
            let mut y = b;

            while x > 0 {
                (x, y) = (y % x, x);
            }

            (y != 1).then_some(a / y * b)
        }

        let mut result = vec![];

        for num in nums {
            let mut cur = num;

            while let Some(&last) = result.last() {
                if let Some(x) = non_coprime(cur, last) {
                    cur = x;
                    result.pop();
                } else {
                    break;
                }
            }

            result.push(cur);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2197() {
        let nums = vec![6, 4, 3, 2, 7, 6, 2];
        let expected = vec![12, 7, 6];
        assert_eq!(Solution::replace_non_coprimes(nums), expected);
        let nums = vec![2, 2, 1, 1, 3, 3, 3];
        let expected = vec![2, 1, 1, 3];
        assert_eq!(Solution::replace_non_coprimes(nums), expected);
    }
}
