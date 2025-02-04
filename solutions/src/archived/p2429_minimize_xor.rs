///
/// # 2429. Minimize XOR
///
/// Given two positive integers `num1` and `num2`, find the positive integer `x` such that:
///
/// * `x` has the same number of set bits as `num2`, and
/// * The value `x XOR num1` is **minimal**.
///
/// Note that `XOR` is the bitwise XOR operation.
///
/// Return *the integer* `x`. The test cases are generated such that `x` is **uniquely determined**.
///
/// The number of **set bits** of an integer is the number of `1`'s in its binary representation.
///
/// **Example 1:**
///
/// ```
/// Input: num1 = 3, num2 = 5
/// Output: 3
/// Explanation:
/// The binary representations of num1 and num2 are 0011 and 0101, respectively.
/// The integer 3 has the same number of set bits as num2, and the value 3 XOR 3 = 0 is minimal.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: num1 = 1, num2 = 12
/// Output: 3
/// Explanation:
/// The binary representations of num1 and num2 are 0001 and 1100, respectively.
/// The integer 3 has the same number of set bits as num2, and the value 3 XOR 1 = 2 is minimal.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= num1, num2 <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimize-xor/
// discuss: https://leetcode.com/problems/minimize-xor/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut cur = num2;
        let mut num2_set_bit_count = 0;

        while cur > 0 {
            if cur & 1 == 1 {
                num2_set_bit_count += 1;
            };

            cur >>= 1;
        }

        let mut cur = num1;
        let mut num1_set_bit_count = 0;

        while cur > 0 {
            if cur & 1 == 1 {
                num1_set_bit_count += 1;
            }

            cur >>= 1;
        }

        let mut result = num1;

        if num2_set_bit_count > num1_set_bit_count {
            let mut cur_idx = 0;

            for _ in num1_set_bit_count..num2_set_bit_count {
                while result & (1 << cur_idx) != 0 {
                    cur_idx += 1;
                }

                result |= 1 << cur_idx;
                cur_idx += 1;
            }
        } else {
            let mut cur_idx = 0;

            for _ in num2_set_bit_count..num1_set_bit_count {
                while !result & (1 << cur_idx) != 0 {
                    cur_idx += 1;
                }

                result &= !(1 << cur_idx);
                cur_idx += 1;
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
    fn test_2429() {
        // let num1 = 3;
        // let num2 = 5;
        // let expected = 3;
        // assert_eq!(Solution::minimize_xor(num1, num2), expected);
        // let num1 = 1;
        // let num2 = 12;
        // let expected = 3;
        // assert_eq!(Solution::minimize_xor(num1, num2), expected);
        let num1 = 25;
        let num2 = 72;
        let expected = 24;
        assert_eq!(Solution::minimize_xor(num1, num2), expected);
    }
}
