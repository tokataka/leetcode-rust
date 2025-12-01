///
/// # 1611. Minimum One Bit Operations to Make Integers Zero
///
/// Given an integer `n`, you must transform it into `0` using the following operations any number of times:
///
/// * Change the rightmost (`0<sup>th</sup>`) bit in the binary representation of `n`.
/// * Change the `i<sup>th</sup>` bit in the binary representation of `n` if the `(i-1)<sup>th</sup>` bit is set to `1` and the `(i-2)<sup>th</sup>` through `0<sup>th</sup>` bits are set to `0`.
///
/// Return *the minimum number of operations to transform* `n` *into* `0`*.*
///
/// **Example 1:**
///
/// ```
/// Input: n = 3
/// Output: 2
/// Explanation: The binary representation of 3 is "11".
/// "11" -> "01" with the 2nd operation since the 0th bit is 1.
/// "01" -> "00" with the 1st operation.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 6
/// Output: 4
/// Explanation: The binary representation of 6 is "110".
/// "110" -> "010" with the 2nd operation since the 1st bit is 1 and 0th through 0th bits are 0.
/// "010" -> "011" with the 1st operation.
/// "011" -> "001" with the 2nd operation since the 0th bit is 1.
/// "001" -> "000" with the 1st operation.
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= n <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/
// discuss: https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut result = 0;
        let mut cur = 0;

        for i in (0..31).rev() {
            if (n >> i) & 1 != cur {
                result += 1 << i;
                cur = 1;
            } else {
                cur = 0;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1611() {
        // let n = 3;
        // let expected = 2;
        // assert_eq!(Solution::minimum_one_bit_operations(n), expected);
        // let n = 6;
        // let expected = 4;
        // assert_eq!(Solution::minimum_one_bit_operations(n), expected);
        let n = 9;
        let expected = 14;
        assert_eq!(Solution::minimum_one_bit_operations(n), expected);
    }
}
