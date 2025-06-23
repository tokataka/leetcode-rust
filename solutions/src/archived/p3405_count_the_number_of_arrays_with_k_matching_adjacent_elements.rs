///
/// # 3405. Count the Number of Arrays with K Matching Adjacent Elements
///
/// You are given three integers `n`, `m`, `k`. A **good array** `arr` of size `n` is defined as follows:
///
/// * Each element in `arr` is in the **inclusive** range `[1, m]`.
/// * *Exactly* `k` indices `i` (where `1 <= i < n`) satisfy the condition `arr[i - 1] == arr[i]`.
///
/// Return the number of **good arrays** that can be formed.
///
/// Since the answer may be very large, return it **modulo** `10<sup>9 </sup>+ 7`.
///
/// **Example 1:**
///
/// **Input:** n = 3, m = 2, k = 1
///
/// **Output:** 4
///
/// **Explanation:**
///
/// * There are 4 good arrays. They are `[1, 1, 2]`, `[1, 2, 2]`, `[2, 1, 1]` and `[2, 2, 1]`.
/// * Hence, the answer is 4.
///
/// **Example 2:**
///
/// **Input:** n = 4, m = 2, k = 2
///
/// **Output:** 6
///
/// **Explanation:**
///
/// * The good arrays are `[1, 1, 1, 2]`, `[1, 1, 2, 2]`, `[1, 2, 2, 2]`, `[2, 1, 1, 1]`, `[2, 2, 1, 1]` and `[2, 2, 2, 1]`.
/// * Hence, the answer is 6.
///
/// **Example 3:**
///
/// **Input:** n = 5, m = 2, k = 0
///
/// **Output:** 2
///
/// **Explanation:**
///
/// * The good arrays are `[1, 2, 1, 2, 1]` and `[2, 1, 2, 1, 2]`. Hence, the answer is 2.
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>5</sup>`
/// * `1 <= m <= 10<sup>5</sup>`
/// * `0 <= k <= n - 1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-arrays-with-k-matching-adjacent-elements/
// discuss: https://leetcode.com/problems/count-the-number-of-arrays-with-k-matching-adjacent-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = n as i64;
        let m = m as i64;
        let k = k as i64;

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

        fn inv(a: i64, p: i64) -> i64 {
            powmod(a, p - 2, p)
        }

        let fact: Vec<i64> = (1..n).fold(vec![1], |mut acc, x| {
            acc.push((acc.last().unwrap() * x) % MOD);
            acc
        });

        let mut inv_fact = vec![0; n as usize];
        inv_fact[n as usize - 1] = inv(fact[n as usize - 1], MOD);

        for i in (1..n).rev() {
            inv_fact[i as usize - 1] = inv_fact[i as usize] * i % MOD;
        }

        let comb = |n: i64, k: i64, p: i64| -> i64 {
            ((fact[n as usize] * inv_fact[k as usize] % p) * inv_fact[(n - k) as usize]) % p
        };

        (m * powmod(m - 1, n - 1 - k, MOD) % MOD * comb(n - 1, k, MOD) % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3405() {
        // let n = 3;
        // let m = 2;
        // let k = 1;
        // let expected = 4;
        // assert_eq!(Solution::count_good_arrays(n, m, k), expected);
        // let n = 4;
        // let m = 2;
        // let k = 2;
        // let expected = 6;
        // assert_eq!(Solution::count_good_arrays(n, m, k), expected);
        // let n = 5;
        // let m = 2;
        // let k = 0;
        // let expected = 2;
        // assert_eq!(Solution::count_good_arrays(n, m, k), expected);
        let n = 1;
        let m = 1;
        let k = 0;
        let expected = 1;
        assert_eq!(Solution::count_good_arrays(n, m, k), expected);
    }
}
