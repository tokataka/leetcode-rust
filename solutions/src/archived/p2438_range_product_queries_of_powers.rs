///
/// # 2438. Range Product Queries of Powers
///
/// Given a positive integer `n`, there exists a **0-indexed** array called `powers`, composed of the **minimum** number of powers of `2` that sum to `n`. The array is sorted in **non-decreasing** order, and there is **only one** way to form the array.
///
/// You are also given a **0-indexed** 2D integer array `queries`, where `queries[i] = [left<sub>i</sub>, right<sub>i</sub>]`. Each `queries[i]` represents a query where you have to find the product of all `powers[j]` with `left<sub>i</sub> <= j <= right<sub>i</sub>`.
///
/// Return *an array* `answers`*, equal in length to* `queries`*, where* `answers[i]` *is the answer to the* `i<sup>th</sup>` *query*. Since the answer to the `i<sup>th</sup>` query may be too large, each `answers[i]` should be returned **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 15, queries = [[0,1],[2,2],[0,3]]
/// Output: [2,4,64]
/// Explanation:
/// For n = 15, powers = [1,2,4,8]. It can be shown that powers cannot be a smaller size.
/// Answer to 1st query: powers[0] * powers[1] = 1 * 2 = 2.
/// Answer to 2nd query: powers[2] = 4.
/// Answer to 3rd query: powers[0] * powers[1] * powers[2] * powers[3] = 1 * 2 * 4 * 8 = 64.
/// Each answer modulo 109 + 7 yields the same answer, so [2,4,64] is returned.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 2, queries = [[0,0]]
/// Output: [2]
/// Explanation:
/// For n = 2, powers = [2].
/// The answer to the only query is powers[0] = 2. The answer modulo 109 + 7 is the same, so [2] is returned.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>9</sup>`
/// * `1 <= queries.length <= 10<sup>5</sup>`
/// * `0 <= start<sub>i</sub> <= end<sub>i</sub> < powers.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/range-product-queries-of-powers/
// discuss: https://leetcode.com/problems/range-product-queries-of-powers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

fn powmod(mut a: i64, mut b: i64, p: i64) -> i64 {
    let mut result = 1;

    while b > 0 {
        if b & 1 == 1 {
            result = (result * a) % p;
        }

        a = (a * a) % p;
        b >>= 1;
    }

    result
}

impl Solution {
    pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;

        let mut prefix_powers = vec![0];
        let mut sum = 0;
        let mut cur = 0;

        while n > 0 {
            if n % 2 == 1 {
                sum += cur;
                prefix_powers.push(sum);
            }

            n /= 2;
            cur += 1;
        }

        queries
            .iter()
            .map(|query| {
                let p = prefix_powers[query[1] as usize + 1] - prefix_powers[query[0] as usize];

                powmod(2, p, MOD) as i32
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2438() {
        let n = 15;
        let queries = nd_vec![[0, 1], [2, 2], [0, 3]];
        let expected = vec![2, 4, 64];
        assert_eq!(Solution::product_queries(n, queries), expected);
        let n = 2;
        let queries = nd_vec![[0, 0]];
        let expected = vec![2];
        assert_eq!(Solution::product_queries(n, queries), expected);
    }
}
