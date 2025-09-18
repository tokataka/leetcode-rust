///
/// # 808. Soup Servings
///
/// You have two soups, **A** and **B**, each starting with `n` mL. On every turn, one of the following four serving operations is chosen *at random*, each with probability `0.25` **independent** of all previous turns:
///
/// * pour 100 mL from type A and 0 mL from type B
/// * pour 75 mL from type A and 25 mL from type B
/// * pour 50 mL from type A and 50 mL from type B
/// * pour 25 mL from type A and 75 mL from type B
///
/// **Note:**
///
/// * There is no operation that pours 0 mL from A and 100 mL from B.
/// * The amounts from A and B are poured *simultaneously* during the turn.
/// * If an operation asks you to pour **more than** you have left of a soup, pour all that remains of that soup.
///
/// The process stops immediately after any turn in which *one of the soups* is used up.
///
/// Return the probability that A is used up *before* B, plus half the probability that both soups are used up in the **same turn**. Answers within `10<sup>-5</sup>` of the actual answer will be accepted.
///
/// **Example 1:**
///
/// ```
/// Input: n = 50
/// Output: 0.62500
/// Explanation:
/// If we perform either of the first two serving operations, soup A will become empty first.
/// If we perform the third operation, A and B will become empty at the same time.
/// If we perform the fourth operation, B will become empty first.
/// So the total probability of A becoming empty first plus half the probability that A and B become empty at the same time, is 0.25 * (1 + 1 + 0.5 + 0) = 0.625.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 100
/// Output: 0.71875
/// Explanation:
/// If we perform the first serving operation, soup A will become empty first.
/// If we perform the second serving operations, A will become empty on performing operation [1, 2, 3], and both A and B become empty on performing operation 4.
/// If we perform the third operation, A will become empty on performing operation [1, 2], and both A and B become empty on performing operation 3.
/// If we perform the fourth operation, A will become empty on performing operation 1, and both A and B become empty on performing operation 2.
/// So the total probability of A becoming empty first plus half the probability that A and B become empty at the same time, is 0.71875.
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= n <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/soup-servings/
// discuss: https://leetcode.com/problems/soup-servings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n == 0 {
            return 0.5;
        }

        if n > 4800 {
            return 1.0;
        }

        let mut result = 0.0;

        let mut cur = HashMap::new();
        let mut next = HashMap::new();

        cur.insert((0, 0), 0.25);

        while !cur.is_empty() {
            for ((a, b), prob) in cur.drain() {
                for (da, db) in [(100, 0), (75, 25), (50, 50), (25, 75)] {
                    match (a + da >= n, b + db >= n) {
                        (true, true) => result += 0.5 * prob,
                        (true, false) => result += prob,
                        (false, true) => (),
                        (false, false) => *next.entry((a + da, b + db)).or_default() += prob / 4.0,
                    };
                }
            }

            (cur, next) = (next, cur);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_808() {
        let n = 50;
        let expected = 0.62500;
        assert_eq!(Solution::soup_servings(n), expected);
        let n = 100;
        let expected = 0.71875;
        assert_eq!(Solution::soup_servings(n), expected);
        // let n = 5000;
        // let expected = 0.99999;
        // assert_eq!(Solution::soup_servings(n), expected);
    }
}
