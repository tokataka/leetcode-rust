///
/// # 679. 24 Game
///
/// You are given an integer array `cards` of length `4`. You have four cards, each containing a number in the range `[1, 9]`. You should arrange the numbers on these cards in a mathematical expression using the operators `['+', '-', '*', '/']` and the parentheses `'('` and `')'` to get the value 24.
///
/// You are restricted with the following rules:
///
/// * The division operator `'/'` represents real division, not integer division.
///   * For example, `4 / (1 - 2 / 3) = 4 / (1 / 3) = 12`.
///
/// * Every operation done is between two numbers. In particular, we cannot use `'-'` as a unary operator.
///   * For example, if `cards = [1, 1, 1, 1]`, the expression `"-1 - 1 - 1 - 1"` is **not allowed**.
///
/// * You cannot concatenate numbers together
///   * For example, if `cards = [1, 2, 1, 2]`, the expression `"12 + 12"` is not valid.
///
/// Return `true` if you can get such expression that evaluates to `24`, and `false` otherwise.
///
/// **Example 1:**
///
/// ```
/// Input: cards = [4,1,8,7]
/// Output: true
/// Explanation: (8-4) * (7-1) = 24
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: cards = [1,2,1,2]
/// Output: false
///
/// ```
///
/// **Constraints:**
///
/// * `cards.length == 4`
/// * `1 <= cards[i] <= 9`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/24-game/
// discuss: https://leetcode.com/problems/24-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        fn _judge_point24(
            points: &mut Vec<f64>,
            used: &mut [bool],
            op_remain: i32,
            cards: &[i32],
        ) -> bool {
            if op_remain == 0 {
                return (points[0] - 24.0).abs() <= 1e-3;
            }

            if points.len() > 1 {
                let a = points.pop().unwrap();
                let b = points.pop().unwrap();

                for x in [b + a, b - a, b * a, b / a] {
                    if !x.is_finite() {
                        continue;
                    }

                    points.push(x);

                    if _judge_point24(points, used, op_remain - 1, cards) {
                        return true;
                    }

                    points.pop();
                }

                points.push(b);
                points.push(a);
            }

            for (i, &card) in cards.iter().enumerate() {
                if used[i] {
                    continue;
                }

                used[i] = true;
                points.push(card as f64);

                if _judge_point24(points, used, op_remain, cards) {
                    return true;
                }

                points.pop();
                used[i] = false;
            }

            false
        }

        _judge_point24(&mut vec![], &mut [false; 4], 3, &cards)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_679() {
        // let cards = vec![4, 1, 8, 7];
        // let expected = true;
        // assert_eq!(Solution::judge_point24(cards), expected);
        // let cards = vec![1, 2, 1, 2];
        // let expected = false;
        // assert_eq!(Solution::judge_point24(cards), expected);
        let cards = vec![1, 3, 4, 6];
        let expected = true;
        assert_eq!(Solution::judge_point24(cards), expected);
    }
}
