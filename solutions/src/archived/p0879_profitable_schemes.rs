use std::collections::HashMap;

///
/// # 879. Profitable Schemes
///
/// There is a group of `n` members, and a list of various crimes they could commit. The `i<sup>th</sup>` crime generates a `profit[i]` and requires `group[i]` members to participate in it. If a member participates in one crime, that member can't participate in another crime.
///
/// Let's call a **profitable scheme** any subset of these crimes that generates at least `minProfit` profit, and the total number of members participating in that subset of crimes is at most `n`.
///
/// Return the number of schemes that can be chosen. Since the answer may be very large, **return it modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 5, minProfit = 3, group = [2,2], profit = [2,3]
/// Output: 2
/// Explanation: To make a profit of at least 3, the group could either commit crimes 0 and 1, or just crime 1.
/// In total, there are 2 schemes.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
/// Output: 7
/// Explanation: To make a profit of at least 5, the group could commit any crimes, as long as they commit one.
/// There are 7 possible schemes: (0), (1), (2), (0,1), (0,2), (1,2), and (0,1,2).
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 100`
/// * `0 <= minProfit <= 100`
/// * `1 <= group.length <= 100`
/// * `1 <= group[i] <= 100`
/// * `profit.length == group.length`
/// * `0 <= profit[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/profitable-schemes/
// discuss: https://leetcode.com/problems/profitable-schemes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        fn _profitable_schemes(
            idx: usize,
            n: i32,
            min_profit: i32,
            cache: &mut HashMap<(usize, i32, i32), i32>,
            group: &Vec<i32>,
            profit: &Vec<i32>,
        ) -> i32 {
            if let Some(&x) = cache.get(&(idx, n, min_profit)) {
                return x;
            }

            if idx >= group.len() {
                return 0;
            }

            let mut result = 0;

            let g = group[idx];
            let p = profit[idx];

            if n >= g {
                let next_n = n - g;
                let next_min_profit = (min_profit - p).max(0);
                if next_min_profit == 0 {
                    result += 1;
                }

                result +=
                    _profitable_schemes(idx + 1, next_n, next_min_profit, cache, group, profit);
            }

            result += _profitable_schemes(idx + 1, n, min_profit, cache, group, profit);
            result %= MOD;

            cache.insert((idx, n, min_profit), result);
            result
        }

        _profitable_schemes(0, n, min_profit, &mut HashMap::new(), &group, &profit)
            + if min_profit == 0 { 1 } else { 0 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_879() {
        let n = 5;
        let min_profit = 3;
        let group = vec![2, 2];
        let profit = vec![2, 3];
        let expected = 2;
        assert_eq!(
            Solution::profitable_schemes(n, min_profit, group, profit),
            expected
        );
        let n = 10;
        let min_profit = 5;
        let group = vec![2, 3, 5];
        let profit = vec![6, 7, 8];
        let expected = 7;
        assert_eq!(
            Solution::profitable_schemes(n, min_profit, group, profit),
            expected
        );
    }
}
