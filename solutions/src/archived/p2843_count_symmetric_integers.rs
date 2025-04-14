///
/// # 2843.   Count Symmetric Integers
///
/// You are given two positive integers `low` and `high`.
///
/// An integer `x` consisting of `2 * n` digits is **symmetric** if the sum of the first `n` digits of `x` is equal to the sum of the last `n` digits of `x`. Numbers with an odd number of digits are never symmetric.
///
/// Return *the **number of symmetric** integers in the range* `[low, high]`.
///
/// **Example 1:**
///
/// ```
/// Input: low = 1, high = 100
/// Output: 9
/// Explanation: There are 9 symmetric integers between 1 and 100: 11, 22, 33, 44, 55, 66, 77, 88, and 99.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: low = 1200, high = 1230
/// Output: 4
/// Explanation: There are 4 symmetric integers between 1200 and 1230: 1203, 1212, 1221, and 1230.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= low <= high <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-symmetric-integers/
// discuss: https://leetcode.com/problems/count-symmetric-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high)
            .filter(|&x| {
                let n = x.ilog10() + 1;
                if n % 2 == 1 {
                    return false;
                }

                let mut sum = 0;
                let mut cur = x;

                for _ in 0..n / 2 {
                    sum += cur % 10;
                    cur /= 10;
                }

                for _ in 0..n / 2 {
                    sum -= cur % 10;
                    cur /= 10;
                }

                sum == 0
            })
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2843() {
        let low = 1;
        let high = 100;
        let expected = 9;
        assert_eq!(Solution::count_symmetric_integers(low, high), expected);
        let low = 1200;
        let high = 1230;
        let expected = 4;
        assert_eq!(Solution::count_symmetric_integers(low, high), expected);
    }
}
