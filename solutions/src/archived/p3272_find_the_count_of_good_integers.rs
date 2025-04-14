///
/// # 3272. Find the Count of Good Integers
///
/// You are given two **positive** integers `n` and `k`.
///
/// An integer `x` is called **k-palindromic** if:
///
/// * `x` is a palindrome.
/// * `x` is divisible by `k`.
///
/// An integer is called **good** if its digits can be *rearranged* to form a **k-palindromic** integer. For example, for `k = 2`, 2020 can be rearranged to form the *k-palindromic* integer 2002, whereas 1010 cannot be rearranged to form a *k-palindromic* integer.
///
/// Return the count of **good** integers containing `n` digits.
///
/// **Note** that *any* integer must **not** have leading zeros, **neither** before **nor** after rearrangement. For example, 1010 *cannot* be rearranged to form 101.
///
/// **Example 1:**
///
/// **Input:** n = 3, k = 5
///
/// **Output:** 27
///
/// **Explanation:**
///
/// *Some* of the good integers are:
///
/// * 551 because it can be rearranged to form 515.
/// * 525 because it is already k-palindromic.
///
/// **Example 2:**
///
/// **Input:** n = 1, k = 4
///
/// **Output:** 2
///
/// **Explanation:**
///
/// The two good integers are 4 and 8.
///
/// **Example 3:**
///
/// **Input:** n = 5, k = 6
///
/// **Output:** 2468
///
/// **Constraints:**
///
/// * `1 <= n <= 10`
/// * `1 <= k <= 9`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-count-of-good-integers/
// discuss: https://leetcode.com/problems/find-the-count-of-good-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let mut dict: HashSet<String> = HashSet::new();
        let base = 10i32.pow(((n - 1) / 2) as u32);
        let skip = (n & 1) as usize;
        /* Enumerate the number of palindrome numbers of n digits */
        for i in base..base * 10 {
            let s = i.to_string();
            let rev: String = s.chars().rev().skip(skip).collect();
            let combined = format!("{}{}", s, rev);
            let palindromic_integer: i64 = combined.parse().unwrap();
            /* If the current palindrome number is a k-palindromic integer */
            if palindromic_integer % (k as i64) == 0 {
                let mut sorted_chars: Vec<char> = combined.chars().collect();
                sorted_chars.sort();
                dict.insert(sorted_chars.into_iter().collect());
            }
        }

        let mut factorial = vec![1i64; (n + 1) as usize];
        for i in 1..=n as usize {
            factorial[i] = factorial[i - 1] * (i as i64);
        }

        let mut ans = 0i64;
        for s in dict {
            let mut cnt = [0; 10];
            for c in s.chars() {
                cnt[c.to_digit(10).unwrap() as usize] += 1;
            }
            /* Calculate permutations and combinations */
            let mut tot = (n as i64 - cnt[0] as i64) * factorial[(n - 1) as usize];
            for &x in cnt.iter() {
                tot /= factorial[x];
            }
            ans += tot;
        }

        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3272() {
        let n = 3;
        let k = 5;
        let expected = 27;
        assert_eq!(Solution::count_good_integers(n, k), expected);
        let n = 1;
        let k = 4;
        let expected = 2;
        assert_eq!(Solution::count_good_integers(n, k), expected);
        let n = 5;
        let k = 6;
        let expected = 2468;
        assert_eq!(Solution::count_good_integers(n, k), expected);
    }
}
