///
/// # 372. Super Pow
///
/// Your task is to calculate `a<sup>b</sup>` mod `1337` where `a` is a positive integer and `b` is an extremely large positive integer given in the form of an array.
///
/// **Example 1:**
///
/// ```
/// Input: a = 2, b = [3]
/// Output: 8
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: a = 2, b = [1,0]
/// Output: 1024
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: a = 1, b = [4,3,3,8,5,2]
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= a <= 2<sup>31</sup> - 1`
/// * `1 <= b.length <= 2000`
/// * `0 <= b[i] <= 9`
/// * `b` does not contain leading zeros.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/super-pow/
// discuss: https://leetcode.com/problems/super-pow/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        const MOD: i32 = 1337;

        let mut result = 1;

        let mut base = a % MOD;

        for mut c in b.into_iter().rev() {
            let mut cur_result = 1;
            let mut cur_base = base;

            for _ in 0..3 {
                if c % 2 == 1 {
                    cur_result = (cur_result * cur_base) % MOD;
                }

                cur_base = (cur_base * cur_base) % MOD;
                c /= 2;
            }

            if c % 2 == 1 {
                cur_result = (cur_result * cur_base) % MOD;
            }

            result = (result * cur_result) % MOD;
            base = (cur_base * base * base) % MOD;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_372() {
        let a = 2;
        let b = vec![3];
        let expected = 8;
        assert_eq!(Solution::super_pow(a, b), expected);
        let a = 2;
        let b = vec![1, 0];
        let expected = 1024;
        assert_eq!(Solution::super_pow(a, b), expected);
        let a = 1;
        let b = vec![4, 3, 3, 8, 5, 2];
        let expected = 1;
        assert_eq!(Solution::super_pow(a, b), expected);
    }
}
