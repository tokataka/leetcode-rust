///
/// # 837. New 21 Game
///
/// Alice plays the following game, loosely based on the card game **"21"**.
///
/// Alice starts with `0` points and draws numbers while she has less than `k` points. During each draw, she gains an integer number of points randomly from the range `[1, maxPts]`, where `maxPts` is an integer. Each draw is independent and the outcomes have equal probabilities.
///
/// Alice stops drawing numbers when she gets `k` **or more points**.
///
/// Return the probability that Alice has `n` or fewer points.
///
/// Answers within `10<sup>-5</sup>` of the actual answer are considered accepted.
///
/// **Example 1:**
///
/// ```
/// Input: n = 10, k = 1, maxPts = 10
/// Output: 1.00000
/// Explanation: Alice gets a single card, then stops.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 6, k = 1, maxPts = 10
/// Output: 0.60000
/// Explanation: Alice gets a single card, then stops.
/// In 6 out of 10 possibilities, she is at or below 6 points.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 21, k = 17, maxPts = 10
/// Output: 0.73278
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= k <= n <= 10<sup>4</sup>`
/// * `1 <= maxPts <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/new-21-game/
// discuss: https://leetcode.com/problems/new-21-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 || k - 1 + max_pts <= n {
            return 1.0;
        }

        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;
        let base = max_pts as f64;

        let mut result = 0.0;

        let mut window = VecDeque::from(vec![0.0; max_pts + 1]);

        window[0] = 1.0;
        window[1] = -1.0;

        let mut cur = 0.0;

        for i in 0..=n {
            cur += window.pop_front().unwrap();

            if i >= k {
                result += cur;
            } else {
                window.push_back(-cur / base);
                cur += cur / base;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_837() {
        let n = 10;
        let k = 1;
        let max_pts = 10;
        let expected = 1.00000;
        assert!((Solution::new21_game(n, k, max_pts) - expected).abs() <= 1e-5);
        let n = 6;
        let k = 1;
        let max_pts = 10;
        let expected = 0.60000;
        assert!((Solution::new21_game(n, k, max_pts) - expected).abs() <= 1e-5);
        let n = 21;
        let k = 17;
        let max_pts = 10;
        let expected = 0.73278;
        assert!((Solution::new21_game(n, k, max_pts) - expected).abs() <= 1e-5);
    }
}
