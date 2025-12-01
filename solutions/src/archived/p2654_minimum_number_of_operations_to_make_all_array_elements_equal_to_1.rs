///
/// # 2654. Minimum Number of Operations to Make All Array Elements Equal to 1
///
/// You are given a **0-indexed** array `nums` consisiting of **positive** integers. You can do the following operation on the array **any** number of times:
///
/// * Select an index `i` such that `0 <= i < n - 1` and replace either of `nums[i]` or `nums[i+1]` with their gcd value.
///
/// Return *the **minimum** number of operations to make all elements of* `nums` *equal to* `1`. If it is impossible, return `-1`.
///
/// The gcd of two integers is the greatest common divisor of the two integers.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,6,3,4]
/// Output: 4
/// Explanation: We can do the following operations:
/// - Choose index i = 2 and replace nums[2] with gcd(3,4) = 1. Now we have nums = [2,6,1,4].
/// - Choose index i = 1 and replace nums[1] with gcd(6,1) = 1. Now we have nums = [2,1,1,4].
/// - Choose index i = 0 and replace nums[0] with gcd(2,1) = 1. Now we have nums = [1,1,1,4].
/// - Choose index i = 2 and replace nums[3] with gcd(1,4) = 1. Now we have nums = [1,1,1,1].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [2,10,6,14]
/// Output: -1
/// Explanation: It can be shown that it is impossible to make all the elements equal to 1.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= nums.length <= 50`
/// * `1 <= nums[i] <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if a == 0 {
                return b;
            }

            gcd(b % a, a)
        }

        let n = nums.len();

        match nums.iter().filter(|&&num| num == 1).count() {
            0 => (),
            x => return (n - x) as i32,
        };

        let mut min_gcd_1 = usize::MAX;

        for i in 0..n {
            let mut cur = nums[i];

            for j in i + 1..n {
                cur = gcd(cur, nums[j]);

                if cur == 1 {
                    min_gcd_1 = min_gcd_1.min(j - i);
                    break;
                }
            }
        }

        match min_gcd_1 {
            usize::MAX => -1,
            x => (x + n - 1) as i32,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2654() {
        // let nums = vec![2, 6, 3, 4];
        // let expected = 4;
        // assert_eq!(Solution::min_operations(nums), expected);
        // let nums = vec![2, 10, 6, 14];
        // let expected = -1;
        // assert_eq!(Solution::min_operations(nums), expected);
        let nums = vec![
            829356, 871205, 641766, 78591, 260856, 710909, 133123, 455954, 107107, 218468, 262055,
            994733, 179279, 617084, 38344, 12115, 487401, 867064, 500306, 700803, 40075, 588622,
            352540, 152206, 344708, 801918, 791898, 57740, 695497, 95854, 31243, 434525, 291006,
            650351, 814412, 735227, 39227, 45001, 889640, 88883, 287184, 279102, 915413, 394596,
            99699, 633787, 328900, 142098, 202908, 399154,
        ];
        let expected = 50;
        assert_eq!(Solution::min_operations(nums), expected);
    }
}
