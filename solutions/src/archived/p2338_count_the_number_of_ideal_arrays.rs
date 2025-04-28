use std::collections::HashMap;

///
/// # 2338. Count the Number of Ideal Arrays
///
/// You are given two integers `n` and `maxValue`, which are used to describe an **ideal** array.
///
/// A **0-indexed** integer array `arr` of length `n` is considered **ideal** if the following conditions hold:
///
/// * Every `arr[i]` is a value from `1` to `maxValue`, for `0 <= i < n`.
/// * Every `arr[i]` is divisible by `arr[i - 1]`, for `0 < i < n`.
///
/// Return *the number of **distinct** ideal arrays of length* `n`. Since the answer may be very large, return it modulo `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 2, maxValue = 5
/// Output: 10
/// Explanation: The following are the possible ideal arrays:
/// - Arrays starting with the value 1 (5 arrays): [1,1], [1,2], [1,3], [1,4], [1,5]
/// - Arrays starting with the value 2 (2 arrays): [2,2], [2,4]
/// - Arrays starting with the value 3 (1 array): [3,3]
/// - Arrays starting with the value 4 (1 array): [4,4]
/// - Arrays starting with the value 5 (1 array): [5,5]
/// There are a total of 5 + 2 + 1 + 1 + 1 = 10 distinct ideal arrays.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 5, maxValue = 3
/// Output: 11
/// Explanation: The following are the possible ideal arrays:
/// - Arrays starting with the value 1 (9 arrays):
///    - With no other distinct values (1 array): [1,1,1,1,1]
///    - With 2nd distinct value 2 (4 arrays): [1,1,1,1,2], [1,1,1,2,2], [1,1,2,2,2], [1,2,2,2,2]
///    - With 2nd distinct value 3 (4 arrays): [1,1,1,1,3], [1,1,1,3,3], [1,1,3,3,3], [1,3,3,3,3]
/// - Arrays starting with the value 2 (1 array): [2,2,2,2,2]
/// - Arrays starting with the value 3 (1 array): [3,3,3,3,3]
/// There are a total of 9 + 1 + 1 = 11 distinct ideal arrays.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 10<sup>4</sup>`
/// * `1 <= maxValue <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-ideal-arrays/
// discuss: https://leetcode.com/problems/count-the-number-of-ideal-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: usize = 1_000_000_007;
        let n = n as usize;
        let max_value = max_value as usize;

        let mut prime_map = (0..=max_value).collect::<Vec<_>>();

        let mut x = 2;

        while x * x <= max_value {
            if prime_map[x] == x {
                for i in (x * x..=max_value).step_by(x) {
                    prime_map[i] = x;
                }
            }

            x += 1;
        }

        fn powmod(mut a: usize, mut b: usize, p: usize) -> usize {
            a %= p;

            if a == 0 {
                return 0;
            }

            let mut product = 1;

            while b > 0 {
                if b & 1 == 1 {
                    product = (product * a) % p;
                    b -= 1;
                }

                a = (a * a) % p;
                b /= 2;
            }

            product
        }

        fn inv(a: usize, p: usize) -> usize {
            powmod(a, p - 2, p)
        }

        let fact: Vec<usize> = (1..=10050).fold(vec![1], |mut acc, x| {
            acc.push((acc.last().unwrap() * x) % MOD);
            acc
        });

        let comb = |n: usize, k: usize, p: usize| -> usize {
            ((fact[n] * inv(fact[k], p) % p) * inv(fact[n - k], p)) % p
        };

        let mut result = 0;
        let mut prime_count = HashMap::new();

        for x in 1..=max_value {
            let mut cur = x;

            while prime_map[cur] > 1 {
                *prime_count.entry(prime_map[cur]).or_default() += 1;
                cur /= prime_map[cur];
            }

            let mut count = 1;

            for &c in prime_count.values() {
                if c > 0 {
                    count = (count * comb(n + c - 1, c, MOD)) % MOD;
                }
            }

            result = (result + count) % MOD;
            prime_count.clear();
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2338() {
        // let n = 2;
        // let max_value = 5;
        // let expected = 10;
        // assert_eq!(Solution::ideal_arrays(n, max_value), expected);
        // let n = 5;
        // let max_value = 3;
        // let expected = 11;
        // assert_eq!(Solution::ideal_arrays(n, max_value), expected);
        let n = 12;
        let max_value = 7;
        let expected = 271;
        assert_eq!(Solution::ideal_arrays(n, max_value), expected);
    }
}
