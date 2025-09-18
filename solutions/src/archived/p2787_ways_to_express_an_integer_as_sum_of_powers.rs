///
/// # 2787. Ways to Express an Integer as Sum of Powers
///
/// Given two **positive** integers `n` and `x`.
///
/// Return *the number of ways* `n` *can be expressed as the sum of the* `x<sup>th</sup>` *power of **unique** positive integers, in other words, the number of sets of unique integers* `[n<sub>1</sub>, n<sub>2</sub>, ..., n<sub>k</sub>]` *where* `n = n<sub>1</sub><sup>x</sup> + n<sub>2</sub><sup>x</sup> + ... + n<sub>k</sub><sup>x</sup>`*.*
///
/// Since the result can be very large, return it modulo `10<sup>9</sup> + 7`.
///
/// For example, if `n = 160` and `x = 3`, one way to express `n` is `n = 2<sup>3</sup> + 3<sup>3</sup> + 5<sup>3</sup>`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 10, x = 2
/// Output: 1
/// Explanation: We can express n as the following: n = 32 + 12 = 10.
/// It can be shown that it is the only way to express 10 as the sum of the 2nd power of unique integers.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 4, x = 1
/// Output: 2
/// Explanation: We can express n in the following ways:
/// - n = 41 = 4.
/// - n = 31 + 11 = 4.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 300`
/// * `1 <= x <= 5`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers/
// discuss: https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;
        let x = x as u32;

        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for nx in (1usize..).map(|i| i.pow(x)).take_while(|&nx| nx <= n) {
            for prev in (0..=n - nx).rev() {
                dp[nx + prev] = (dp[nx + prev] + dp[prev]) % MOD;
            }
        }

        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2787() {
        // let n = 10;
        // let x = 2;
        // let expected = 1;
        // assert_eq!(Solution::number_of_ways(n, x), expected);
        // let n = 4;
        // let x = 1;
        // let expected = 2;
        // assert_eq!(Solution::number_of_ways(n, x), expected);
        let n = 5;
        let x = 1;
        let expected = 3;
        assert_eq!(Solution::number_of_ways(n, x), expected);
    }
}
