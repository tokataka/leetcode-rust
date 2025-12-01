///
/// # 1510. Stone Game IV
///
/// Alice and Bob take turns playing a game, with Alice starting first.
///
/// Initially, there are `n` stones in a pile. On each player's turn, that player makes a *move* consisting of removing **any** non-zero **square number** of stones in the pile.
///
/// Also, if a player cannot make a move, he/she loses the game.
///
/// Given a positive integer `n`, return `true` if and only if Alice wins the game otherwise return `false`, assuming both players play optimally.
///
/// **Example 1:**
///
/// ```
/// Input: n = 1
/// Output: true
/// Explanation: Alice can remove 1 stone winning the game because Bob doesn't have any moves.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 2
/// Output: false
/// Explanation: Alice can only remove 1 stone, after that Bob removes the last one winning the game (2 -> 1 -> 0).
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 4
/// Output: true
/// Explanation: n is already a perfect square, Alice can win with one move, removing 4 stones (4 -> 0).
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/stone-game-iv/
// discuss: https://leetcode.com/problems/stone-game-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n + 1];

        let squares = (1..)
            .map(|x| x * x)
            .take_while(|&x| x <= n)
            .collect::<Vec<_>>();

        for i in 0..n {
            if !dp[i] {
                for &square in squares.iter().take_while(|&&square| i + square <= n) {
                    dp[i + square] = true;
                }
            }
        }

        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1510() {
        let n = 1;
        let expected = true;
        assert_eq!(Solution::winner_square_game(n), expected);
        let n = 2;
        let expected = false;
        assert_eq!(Solution::winner_square_game(n), expected);
        let n = 4;
        let expected = true;
        assert_eq!(Solution::winner_square_game(n), expected);
    }
}
