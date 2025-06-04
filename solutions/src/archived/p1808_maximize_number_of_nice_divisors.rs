///
/// # 1808. Maximize Number of Nice Divisors
///
/// You are given a positive integer `primeFactors`. You are asked to construct a positive integer `n` that satisfies the following conditions:
///
/// * The number of prime factors of `n` (not necessarily distinct) is **at most** `primeFactors`.
/// * The number of nice divisors of `n` is maximized. Note that a divisor of `n` is **nice** if it is divisible by every prime factor of `n`. For example, if `n = 12`, then its prime factors are `[2,2,3]`, then `6` and `12` are nice divisors, while `3` and `4` are not.
///
/// Return *the number of nice divisors of* `n`. Since that number can be too large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// Note that a prime number is a natural number greater than `1` that is not a product of two smaller natural numbers. The prime factors of a number `n` is a list of prime numbers such that their product equals `n`.
///
/// **Example 1:**
///
/// ```
/// Input: primeFactors = 5
/// Output: 6
/// Explanation: 200 is a valid value of n.
/// It has 5 prime factors: [2,2,2,5,5], and it has 6 nice divisors: [10,20,40,50,100,200].
/// There is not other value of n that has at most 5 prime factors and more nice divisors.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: primeFactors = 8
/// Output: 18
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= primeFactors <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-number-of-nice-divisors/
// discuss: https://leetcode.com/problems/maximize-number-of-nice-divisors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

fn powmod(mut a: i64, mut b: i64, p: i64) -> i64 {
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

impl Solution {
    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
        if prime_factors <= 3 {
            return prime_factors;
        }

        const MOD: i64 = 1_000_000_007;
        let prime_factors = prime_factors as i64;

        let (pow3, pow2) = match prime_factors % 3 {
            0 => (prime_factors / 3, 0),
            1 => (prime_factors / 3 - 1, 2),
            2 => (prime_factors / 3, 1),
            _ => unreachable!(),
        };

        (powmod(3, pow3, MOD) * 2i64.pow(pow2) % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1808() {
        let prime_factors = 5;
        let expected = 6;
        assert_eq!(Solution::max_nice_divisors(prime_factors), expected);
        let prime_factors = 8;
        let expected = 18;
        assert_eq!(Solution::max_nice_divisors(prime_factors), expected);
    }
}
