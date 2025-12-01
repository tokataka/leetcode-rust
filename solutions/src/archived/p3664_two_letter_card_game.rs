///
/// # 3664. Two-Letter Card Game
///
/// You are given a deck of cards represented by a string array `cards`, and each card displays two lowercase letters.
///
/// You are also given a letter `x`. You play a game with the following rules:
///
/// * Start with 0 points.
/// * On each turn, you must find two **compatible** cards from the deck that both contain the letter `x` in any position.
/// * Remove the pair of cards and earn **1 point**.
/// * The game ends when you can no longer find a pair of compatible cards.
///
/// Return the **maximum** number of points you can gain with optimal play.
///
/// Two cards are **compatible** if the strings differ in **exactly** 1 position.
///
/// **Example 1:**
///
/// **Input:** cards = ["aa","ab","ba","ac"], x = "a"
///
/// **Output:** 2
///
/// **Explanation:**
///
/// * On the first turn, select and remove cards `"ab"` and `"ac"`, which are compatible because they differ at only index 1.
/// * On the second turn, select and remove cards `"aa"` and `"ba"`, which are compatible because they differ at only index 0.
///
/// Because there are no more compatible pairs, the total score is 2.
///
/// **Example 2:**
///
/// **Input:** cards = ["aa","ab","ba"], x = "a"
///
/// **Output:** 1
///
/// **Explanation:**
///
/// * On the first turn, select and remove cards `"aa"` and `"ba"`.
///
/// Because there are no more compatible pairs, the total score is 1.
///
/// **Example 3:**
///
/// **Input:** cards = ["aa","ab","ba","ac"], x = "b"
///
/// **Output:** 0
///
/// **Explanation:**
///
/// The only cards that contain the character `'b'` are `"ab"` and `"ba"`. However, they differ in both indices, so they are not compatible. Thus, the output is 0.
///
/// **Constraints:**
///
/// * `2 <= cards.length <= 10<sup>5</sup>`
/// * `cards[i].length == 2`
/// * Each `cards[i]` is composed of only lowercase English letters between `'a'` and `'j'`.
/// * `x` is a lowercase English letter between `'a'` and `'j'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/two-letter-card-game/
// discuss: https://leetcode.com/problems/two-letter-card-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn score(cards: Vec<String>, x: char) -> i32 {
        let mut cards_split = [[0; 128], [0; 128]];
        let mut cards_max = [0, 0];
        let mut cards_sum = [0, 0];
        let mut both = 0;

        let x = x as usize;

        for card in cards {
            let card = card.as_bytes();
            let card = [card[0] as usize, card[1] as usize];

            if card[0] == x && card[1] == x {
                both += 1;
            } else if card[0] == x {
                cards_split[0][card[1]] += 1;
                cards_max[0] = cards_max[0].max(cards_split[0][card[1]]);
                cards_sum[0] += 1;
            } else if card[1] == x {
                cards_split[1][card[0]] += 1;
                cards_max[1] = cards_max[1].max(cards_split[1][card[0]]);
                cards_sum[1] += 1;
            }
        }

        let mut result = 0;

        for b in 0..=both {
            result = result.max(
                [b, both - b]
                    .into_iter()
                    .enumerate()
                    .map(|(i, b)| {
                        let max = b.max(cards_max[i]);
                        let sum = cards_sum[i] + b;

                        if max * 2 > sum {
                            sum - max
                        } else {
                            sum / 2
                        }
                    })
                    .sum(),
            );
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3664() {
        assert_eq!(Solution::score(vec_string!["aa", "ab", "ba", "ac"], 'a'), 2);
        assert_eq!(Solution::score(vec_string!["aa", "ab", "ba"], 'a'), 1);
        assert_eq!(Solution::score(vec_string!["aa", "ab", "ba", "ac"], 'b'), 0);
    }
}
