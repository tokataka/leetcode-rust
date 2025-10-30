///
/// # 2048. Next Greater Numerically Balanced Number
///
/// An integer `x` is **numerically balanced** if for every digit `d` in the number `x`, there are **exactly** `d` occurrences of that digit in `x`.
///
/// Given an integer `n`, return *the **smallest numerically balanced** number **strictly greater** than* `n`*.*
///
/// **Example 1:**
///
/// ```
/// Input: n = 1
/// Output: 22
/// Explanation:
/// 22 is numerically balanced since:
/// - The digit 2 occurs 2 times.
/// It is also the smallest numerically balanced number strictly greater than 1.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 1000
/// Output: 1333
/// Explanation:
/// 1333 is numerically balanced since:
/// - The digit 1 occurs 1 time.
/// - The digit 3 occurs 3 times.
/// It is also the smallest numerically balanced number strictly greater than 1000.
/// Note that 1022 cannot be the answer because 0 appeared more than 0 times.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 3000
/// Output: 3133
/// Explanation:
/// 3133 is numerically balanced since:
/// - The digit 1 occurs 1 time.
/// - The digit 3 occurs 3 times.
/// It is also the smallest numerically balanced number strictly greater than 3000.
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= n <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/next-greater-numerically-balanced-number/
// discuss: https://leetcode.com/problems/next-greater-numerically-balanced-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        for x in n + 1.. {
            let mut freq = [0; 10];
            let mut cur = x as usize;

            while cur > 0 {
                freq[cur % 10] += 1;
                cur /= 10;
            }

            if freq.iter().enumerate().all(|(i, &x)| x == 0 || x == i) {
                return x;
            }
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2048() {
        let n = 1;
        let expected = 22;
        assert_eq!(Solution::next_beautiful_number(n), expected);
        let n = 1000;
        let expected = 1333;
        assert_eq!(Solution::next_beautiful_number(n), expected);
        let n = 3000;
        let expected = 3133;
        assert_eq!(Solution::next_beautiful_number(n), expected);
    }
}
