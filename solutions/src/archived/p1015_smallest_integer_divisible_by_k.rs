///
/// # 1015. Smallest Integer Divisible by K
///
/// Given a positive integer `k`, you need to find the **length** of the **smallest** positive integer `n` such that `n` is divisible by `k`, and `n` only contains the digit `1`.
///
/// Return *the **length** of* `n`. If there is no such `n`, return -1.
///
/// **Note:** `n` may not fit in a 64-bit signed integer.
///
/// **Example 1:**
///
/// ```
/// Input: k = 1
/// Output: 1
/// Explanation: The smallest answer is n = 1, which has length 1.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: k = 2
/// Output: -1
/// Explanation: There is no such positive integer n divisible by 2.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: k = 3
/// Output: 3
/// Explanation: The smallest answer is n = 111, which has length 3.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= k <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-integer-divisible-by-k/
// discuss: https://leetcode.com/problems/smallest-integer-divisible-by-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut cur = 0;
        let mut result = 0;
        let mut visited = vec![false; k as usize];

        loop {
            cur = (cur * 10 + 1) % k;
            result += 1;

            if cur == 0 {
                return result;
            }

            if visited[cur as usize] {
                return -1;
            }

            visited[cur as usize] = true;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1015() {
        assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);
        assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);
        assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
    }
}
