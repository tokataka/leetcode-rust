///
/// # 3449. Maximize the Minimum Game Score
///
/// You are given an array `points` of size `n` and an integer `m`. There is another array `gameScore` of size `n`, where `gameScore[i]` represents the score achieved at the `i<sup>th</sup>` game. Initially, `gameScore[i] == 0` for all `i`.
///
/// You start at index -1, which is outside the array (before the first position at index 0). You can make **at most** `m` moves. In each move, you can either:
///
/// * Increase the index by 1 and add `points[i]` to `gameScore[i]`.
/// * Decrease the index by 1 and add `points[i]` to `gameScore[i]`.
///
/// **Note** that the index must always remain within the bounds of the array after the first move.
///
/// Return the **maximum possible minimum** value in `gameScore` after **at most** `m` moves.
///
/// **Example 1:**
///
/// **Input:** points = [2,4], m = 3
///
/// **Output:** 4
///
/// **Explanation:**
///
/// Initially, index `i = -1` and `gameScore = [0, 0]`.
///
/// |    Move    |Index|gameScore|
/// |------------|-----|---------|
/// |Increase `i`|  0  |`[2, 0]` |
/// |Increase `i`|  1  |`[2, 4]` |
/// |Decrease `i`|  0  |`[4, 4]` |
///
/// The minimum value in `gameScore` is 4, and this is the maximum possible minimum among all configurations. Hence, 4 is the output.
///
/// **Example 2:**
///
/// **Input:** points = [1,2,3], m = 5
///
/// **Output:** 2
///
/// **Explanation:**
///
/// Initially, index `i = -1` and `gameScore = [0, 0, 0]`.
///
/// |    Move    |Index| gameScore |
/// |------------|-----|-----------|
/// |Increase `i`|  0  |`[1, 0, 0]`|
/// |Increase `i`|  1  |`[1, 2, 0]`|
/// |Decrease `i`|  0  |`[2, 2, 0]`|
/// |Increase `i`|  1  |`[2, 4, 0]`|
/// |Increase `i`|  2  |`[2, 4, 3]`|
///
/// The minimum value in `gameScore` is 2, and this is the maximum possible minimum among all configurations. Hence, 2 is the output.
///
/// **Constraints:**
///
/// * `2 <= n == points.length <= 5 * 10<sup>4</sup>`
/// * `1 <= points[i] <= 10<sup>6</sup>`
/// * `1 <= m <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-the-minimum-game-score/
// discuss: https://leetcode.com/problems/maximize-the-minimum-game-score/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_score(points: Vec<i32>, m: i32) -> i64 {
        let n = points.len();
        let m = m as i64;

        if m < n as i64 {
            return 0;
        }

        let points = points.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let mut left = 0;
        let mut right = 1_000_000_000_000_000;

        while left < right {
            let target = (left + right + 1) / 2;

            let mut total_moves = 0;
            let mut prev_moves = 0;

            for i in 0..n {
                let cur_moves = (target - prev_moves * points[i] + -1).max(0) / points[i];

                total_moves += 1 + cur_moves * 2;

                if i == n - 1 && target <= prev_moves * points[i] {
                    total_moves -= 1;
                }

                if total_moves > m {
                    break;
                }

                prev_moves = cur_moves;
            }

            if total_moves > m {
                right = target - 1;
            } else {
                left = target;
            }
        }

        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3449() {
        // let points = vec![2, 4];
        // let m = 3;
        // let expected = 4;
        // assert_eq!(Solution::max_score(points, m), expected);
        // let points = vec![1, 2, 3];
        // let m = 5;
        // let expected = 2;
        // assert_eq!(Solution::max_score(points, m), expected);
        // let points = vec![12, 16, 19, 9];
        // let m = 7;
        // let expected = 12;
        // assert_eq!(Solution::max_score(points, m), expected);
        let points = vec![1; 50000];
        let m = 1000000000;
        let expected = 20000;
        assert_eq!(Solution::max_score(points, m), expected);
    }
}
