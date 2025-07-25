///
/// # 3333. Find the Original Typed String II
///
/// Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and **may** press a key for too long, resulting in a character being typed **multiple** times.
///
/// You are given a string `word`, which represents the **final** output displayed on Alice's screen. You are also given a **positive** integer `k`.
///
/// Return the total number of *possible* original strings that Alice *might* have intended to type, if she was trying to type a string of size **at least** `k`.
///
/// Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// **Input:** word = "aabbccdd", k = 7
///
/// **Output:** 5
///
/// **Explanation:**
///
/// The possible strings are: `"aabbccdd"`, `"aabbccd"`, `"aabbcdd"`, `"aabccdd"`, and `"abbccdd"`.
///
/// **Example 2:**
///
/// **Input:** word = "aabbccdd", k = 8
///
/// **Output:** 1
///
/// **Explanation:**
///
/// The only possible string is `"aabbccdd"`.
///
/// **Example 3:**
///
/// **Input:** word = "aaabbb", k = 3
///
/// **Output:** 8
///
/// **Constraints:**
///
/// * `1 <= word.length <= 5 * 10<sup>5</sup>`
/// * `word` consists only of lowercase English letters.
/// * `1 <= k <= 2000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-original-typed-string-ii/
// discuss: https://leetcode.com/problems/find-the-original-typed-string-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let k = k as usize;

        let chunk_counts = word
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .map(|x| x.len())
            .collect::<Vec<_>>();

        let total = chunk_counts.iter().fold(1, |acc, &x| acc * x as i64 % MOD);

        let max_add = match k.checked_sub(chunk_counts.len() + 1) {
            Some(x) => x,
            None => return total as i32,
        };

        let mut dp = vec![0; max_add + 1];
        let mut q = VecDeque::with_capacity(max_add + 1);
        dp[0] = 1;

        for chunk_count in chunk_counts {
            q.clear();

            let mut cur_sum = 0;

            for idx in 0..=max_add {
                cur_sum += dp[idx];
                q.push_back(dp[idx]);

                if idx > chunk_count - 1 {
                    cur_sum -= q.pop_front().unwrap();
                }

                dp[idx] = cur_sum % MOD;
            }
        }

        ((total + MOD - dp.into_iter().fold(0, |acc, x| (acc + x) % MOD)) % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3333() {
        let word = "aabbccdd".to_owned();
        let k = 7;
        let expected = 5;
        assert_eq!(Solution::possible_string_count(word, k), expected);
        let word = "aabbccdd".to_owned();
        let k = 8;
        let expected = 1;
        assert_eq!(Solution::possible_string_count(word, k), expected);
        let word = "aaabbb".to_owned();
        let k = 3;
        let expected = 8;
        assert_eq!(Solution::possible_string_count(word, k), expected);
    }
}
