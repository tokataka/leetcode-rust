///
/// # 2327. Number of People Aware of a Secret
///
/// On day `1`, one person discovers a secret.
///
/// You are given an integer `delay`, which means that each person will **share** the secret with a new person **every day**, starting from `delay` days after discovering the secret. You are also given an integer `forget`, which means that each person will **forget** the secret `forget` days after discovering it. A person **cannot** share the secret on the same day they forgot it, or on any day afterwards.
///
/// Given an integer `n`, return *the number of people who know the secret at the end of day* `n`. Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 6, delay = 2, forget = 4
/// Output: 5
/// Explanation:
/// Day 1: Suppose the first person is named A. (1 person)
/// Day 2: A is the only person who knows the secret. (1 person)
/// Day 3: A shares the secret with a new person, B. (2 people)
/// Day 4: A shares the secret with a new person, C. (3 people)
/// Day 5: A forgets the secret, and B shares the secret with a new person, D. (3 people)
/// Day 6: B shares the secret with E, and C shares the secret with F. (5 people)
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 4, delay = 1, forget = 3
/// Output: 6
/// Explanation:
/// Day 1: The first person is named A. (1 person)
/// Day 2: A shares the secret with B. (2 people)
/// Day 3: A and B share the secret with 2 new people, C and D. (4 people)
/// Day 4: A forgets the secret. B, C, and D share the secret with 3 new people. (6 people)
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 1000`
/// * `1 <= delay < forget <= n`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-people-aware-of-a-secret/
// discuss: https://leetcode.com/problems/number-of-people-aware-of-a-secret/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut inactive: VecDeque<i64> = vec![0; delay as usize].into();
        inactive[0] = 1;

        let mut active: VecDeque<i64> = vec![0; (forget - delay) as usize].into();
        let mut active_sum = 0;

        for _ in 0..n - 1 {
            let last_inactive = inactive.pop_back().unwrap() % MOD;

            active_sum -= active.pop_back().unwrap();
            active_sum += last_inactive;
            active.push_front(last_inactive);

            inactive.push_front(active_sum);
        }

        ((inactive.iter().sum::<i64>() + active_sum) % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2327() {
        // let n = 6;
        // let delay = 2;
        // let forget = 4;
        // let expected = 5;
        // assert_eq!(Solution::people_aware_of_secret(n, delay, forget), expected);
        // let n = 4;
        // let delay = 1;
        // let forget = 3;
        // let expected = 6;
        // assert_eq!(Solution::people_aware_of_secret(n, delay, forget), expected);
        let n = 684;
        let delay = 18;
        let forget = 496;
        let expected = 653668527;
        assert_eq!(Solution::people_aware_of_secret(n, delay, forget), expected);
    }
}
