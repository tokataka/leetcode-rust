///
/// # 483. Smallest Good Base
///
/// Given an integer `n` represented as a string, return *the smallest **good base** of* `n`.
///
/// We call `k >= 2` a **good base** of `n`, if all digits of `n` base `k` are `1`'s.
///
/// **Example 1:**
///
/// ```
/// Input: n = "13"
/// Output: "3"
/// Explanation: 13 base 3 is 111.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = "4681"
/// Output: "8"
/// Explanation: 4681 base 8 is 11111.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = "1000000000000000000"
/// Output: "999999999999999999"
/// Explanation: 1000000000000000000 base 999999999999999999 is 11.
///
/// ```
///
/// **Constraints:**
///
/// * `n` is an integer in the range `[3, 10<sup>18</sup>]`.
/// * `n` does not contain any leading zeros.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-good-base/
// discuss: https://leetcode.com/problems/smallest-good-base/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n = n.parse::<i128>().unwrap();

        for k in (3..=n.ilog2() + 1).rev() {
            let mut left = 2;
            let mut right = n - 1;

            while left <= right {
                let base = (left + right) / 2;

                match base.checked_pow(k) {
                    Some(base_k) => match (n * (base - 1) + 1).cmp(&base_k) {
                        std::cmp::Ordering::Equal => return format!("{base}"),
                        std::cmp::Ordering::Less => right = base - 1,
                        std::cmp::Ordering::Greater => left = base + 1,
                    },
                    None => right = base - 1,
                }
            }
        }

        format!("{}", n - 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_483() {
        let n = "13".to_owned();
        let expected = "3".to_owned();
        assert_eq!(Solution::smallest_good_base(n), expected);
        let n = "4681".to_owned();
        let expected = "8".to_owned();
        assert_eq!(Solution::smallest_good_base(n), expected);
        let n = "1000000000000000000".to_owned();
        let expected = "999999999999999999".to_owned();
        assert_eq!(Solution::smallest_good_base(n), expected);
    }
}
