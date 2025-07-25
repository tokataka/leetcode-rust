///
/// # 2410. Maximum Matching of Players With Trainers
///
/// You are given a **0-indexed** integer array `players`, where `players[i]` represents the **ability** of the `i<sup>th</sup>` player. You are also given a **0-indexed** integer array `trainers`, where `trainers[j]` represents the **training capacity** of the `j<sup>th</sup>` trainer.
///
/// The `i<sup>th</sup>` player can **match** with the `j<sup>th</sup>` trainer if the player's ability is **less than or equal to** the trainer's training capacity. Additionally, the `i<sup>th</sup>` player can be matched with at most one trainer, and the `j<sup>th</sup>` trainer can be matched with at most one player.
///
/// Return *the **maximum** number of matchings between* `players` *and* `trainers` *that satisfy these conditions.*
///
/// **Example 1:**
///
/// ```
/// Input: players = [4,7,9], trainers = [8,2,5,8]
/// Output: 2
/// Explanation:
/// One of the ways we can form two matchings is as follows:
/// - players[0] can be matched with trainers[0] since 4 <= 8.
/// - players[1] can be matched with trainers[3] since 7 <= 8.
/// It can be proven that 2 is the maximum number of matchings that can be formed.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: players = [1,1,1], trainers = [10]
/// Output: 1
/// Explanation:
/// The trainer can be matched with any of the 3 players.
/// Each player can only be matched with one trainer, so the maximum answer is 1.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= players.length, trainers.length <= 10<sup>5</sup>`
/// * `1 <= players[i], trainers[j] <= 10<sup>9</sup>`
///
/// **Note:** This question is the same as [ 445: Assign Cookies.](https://leetcode.com/problems/assign-cookies/description/)
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-matching-of-players-with-trainers/
// discuss: https://leetcode.com/problems/maximum-matching-of-players-with-trainers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();

        let mut result = 0;
        let mut idx = 0;

        for player in players {
            idx += trainers[idx..].partition_point(|&x| x < player) + 1;

            if idx <= trainers.len() {
                result += 1;
            }

            if idx >= trainers.len() {
                break;
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
    fn test_2410() {
        let players = vec![4, 7, 9];
        let trainers = vec![8, 2, 5, 8];
        let expected = 2;
        assert_eq!(
            Solution::match_players_and_trainers(players, trainers),
            expected
        );
        let players = vec![1, 1, 1];
        let trainers = vec![10];
        let expected = 1;
        assert_eq!(
            Solution::match_players_and_trainers(players, trainers),
            expected
        );
    }
}
