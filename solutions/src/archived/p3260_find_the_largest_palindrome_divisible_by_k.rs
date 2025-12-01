///
/// # 3260. Find the Largest Palindrome Divisible by K
///
/// You are given two **positive** integers `n` and `k`.
///
/// An integer `x` is called **k-palindromic** if:
///
/// * `x` is a palindrome.
/// * `x` is divisible by `k`.
///
/// Return the **largest** integer having `n` digits (as a string) that is **k-palindromic**.
///
/// **Note** that the integer must **not** have leading zeros.
///
/// **Example 1:**
///
/// **Input:** n = 3, k = 5
///
/// **Output:** "595"
///
/// **Explanation:**
///
/// 595 is the largest k-palindromic integer with 3 digits.
///
/// **Example 2:**
///
/// **Input:** n = 1, k = 4
///
/// **Output:** "8"
///
/// **Explanation:**
///
/// 4 and 8 are the only k-palindromic integers with 1 digit.
///
/// **Example 3:**
///
/// **Input:** n = 5, k = 6
///
/// **Output:** "89898"
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>5</sup>`
/// * `1 <= k <= 9`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-largest-palindrome-divisible-by-k/
// discuss: https://leetcode.com/problems/find-the-largest-palindrome-divisible-by-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn largest_palindrome(n: i32, k: i32) -> String {
        if [1, 3, 9].contains(&k) {
            return "9".repeat(n as usize);
        }

        if k == 2 {
            if n <= 2 {
                return "8".repeat(n as usize);
            } else {
                return "8".to_string() + &"9".repeat(n as usize - 2) + "8";
            }
        }

        if k == 4 {
            if n <= 4 {
                return "8".repeat(n as usize);
            } else {
                return "88".to_string() + &"9".repeat(n as usize - 4) + "88";
            }
        }

        if k == 8 {
            if n <= 6 {
                return "8".repeat(n as usize);
            } else {
                return "888".to_string() + &"9".repeat(n as usize - 6) + "888";
            }
        }

        if k == 5 {
            if n <= 2 {
                return "5".repeat(n as usize);
            } else {
                return "5".to_string() + &"9".repeat(n as usize - 2) + "5";
            }
        }

        if k == 6 {
            if n <= 2 {
                return "6".repeat(n as usize);
            } else {
                let prefix_count = (n - 3) / 2;
                let remain_count = (n - 2) - prefix_count * 2;
                return "8".to_string()
                    + &"9".repeat(prefix_count as usize)
                    + if remain_count == 1 { "8" } else { "77" }
                    + &"9".repeat(prefix_count as usize)
                    + "8";
            }
        }

        if k == 7 {
            if n <= 2 {
                return "7".repeat(n as usize);
            } else {
                let nine_prefix_count = n / 2 - (n / 2) % 6;
                let nine_prefix_remain = n - nine_prefix_count * 2;

                if nine_prefix_remain == 0 {
                    return "9".repeat(n as usize);
                }

                let prefix_count = (nine_prefix_remain - 1) / 2;
                let remain_count = nine_prefix_remain - prefix_count * 2;

                let mut base = 10u64.pow(prefix_count as u32) - 1;
                base *= 1 + 10u64.pow(prefix_count as u32 + remain_count as u32);

                for x in (0..=9).rev() {
                    let cur = base
                        + x * 10u64.pow(prefix_count as u32)
                            * if remain_count == 1 { 1 } else { 11 };

                    if cur % 7 == 0 {
                        return "9".repeat(nine_prefix_count as usize)
                            + &cur.to_string()
                            + &"9".repeat(nine_prefix_count as usize);
                    }
                }
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
    fn test_3260() {
        let n = 3;
        let k = 5;
        let expected = "595".to_owned();
        assert_eq!(Solution::largest_palindrome(n, k), expected);
        let n = 1;
        let k = 4;
        let expected = "8".to_owned();
        assert_eq!(Solution::largest_palindrome(n, k), expected);
        let n = 5;
        let k = 6;
        let expected = "89898".to_owned();
        assert_eq!(Solution::largest_palindrome(n, k), expected);
    }
}
