use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 909. Snakes and Ladders
///
/// You are given an `n x n` integer matrix `board` where the cells are labeled from `1` to `n<sup>2</sup>` in a [**Boustrophedon style**](https://en.wikipedia.org/wiki/Boustrophedon) starting from the bottom left of the board (i.e. `board[n - 1][0]`) and alternating direction each row.
///
/// You start on square `1` of the board. In each move, starting from square `curr`, do the following:
///
/// * Choose a destination square `next` with a label in the range `[curr + 1, min(curr + 6, n<sup>2</sup>)]`.
///   * This choice simulates the result of a standard **6-sided die roll**: i.e., there are always at most 6 destinations, regardless of the size of the board.
///
/// * If `next` has a snake or ladder, you **must** move to the destination of that snake or ladder. Otherwise, you move to `next`.
/// * The game ends when you reach the square `n<sup>2</sup>`.
///
/// A board square on row `r` and column `c` has a snake or ladder if `board[r][c] != -1`. The destination of that snake or ladder is `board[r][c]`. Squares `1` and `n<sup>2</sup>` are not the starting points of any snake or ladder.
///
/// Note that you only take a snake or ladder at most once per dice roll. If the destination to a snake or ladder is the start of another snake or ladder, you do **not** follow the subsequent snake or ladder.
///
/// * For example, suppose the board is `[[-1,4],[-1,3]]`, and on the first move, your destination square is `2`. You follow the ladder to square `3`, but do **not** follow the subsequent ladder to `4`.
///
/// Return *the least number of dice rolls required to reach the square* `n<sup>2</sup>`*. If it is not possible to reach the square, return* `-1`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2018/09/23/snakes.png)
///
/// ```
/// Input: board = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
/// Output: 4
/// Explanation:
/// In the beginning, you start at square 1 (at row 5, column 0).
/// You decide to move to square 2 and must take the ladder to square 15.
/// You then decide to move to square 17 and must take the snake to square 13.
/// You then decide to move to square 14 and must take the ladder to square 35.
/// You then decide to move to square 36, ending the game.
/// This is the lowest possible number of moves to reach the last square, so return 4.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: board = [[-1,-1],[-1,3]]
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `n == board.length == board[i].length`
/// * `2 <= n <= 20`
/// * `board[i][j]` is either `-1` or in the range `[1, n<sup>2</sup>]`.
/// * The squares labeled `1` and `n<sup>2</sup>` are not the starting points of any snake or ladder.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/snakes-and-ladders/
// discuss: https://leetcode.com/problems/snakes-and-ladders/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        let mut ladders = Vec::with_capacity(n * n);

        for i in 0..n {
            for j in 0..n {
                let destination = match board[n - 1 - i][if i % 2 == 0 { j } else { n - 1 - j }] {
                    -1 => usize::MAX,
                    x => x as usize - 1,
                };

                ladders.push(destination);
            }
        }

        let mut pq = BinaryHeap::from([(Reverse(0), 0)]);
        let mut visited = vec![false; n * n];

        while let Some((Reverse(rolls), cur)) = pq.pop() {
            if cur == n * n - 1 {
                return rolls;
            }

            for mut next in cur + 1..=(cur + 6).min(n * n - 1) {
                if visited[next] {
                    continue;
                }

                visited[next] = true;
                if ladders[next] != usize::MAX {
                    next = ladders[next];
                }

                pq.push((Reverse(rolls + 1), next));
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_909() {
        // let board = nd_vec![
        //     [-1, -1, -1, -1, -1, -1],
        //     [-1, -1, -1, -1, -1, -1],
        //     [-1, -1, -1, -1, -1, -1],
        //     [-1, 35, -1, -1, 13, -1],
        //     [-1, -1, -1, -1, -1, -1],
        //     [-1, 15, -1, -1, -1, -1]
        // ];
        // let expected = 4;
        // assert_eq!(Solution::snakes_and_ladders(board), expected);
        // let board = nd_vec![[-1, -1], [-1, 3]];
        // let expected = 1;
        // assert_eq!(Solution::snakes_and_ladders(board), expected);
        let board = nd_vec![
            [-1, -1, -1, -1, 48, 5, -1],  // 42 - 48
            [12, 29, 13, 9, -1, 2, 32],   // 41 - 35
            [-1, -1, 21, 7, -1, 12, 49],  // 28 - 34
            [42, 37, 21, 40, -1, 22, 12], // 27 - 21
            [42, -1, 2, -1, -1, -1, 6],   // 14 - 20
            [39, -1, 35, -1, -1, 39, -1], // 13 -  7
            [-1, 36, -1, -1, -1, -1, 5]   // 0  -  6
        ];
        let expected = 3;
        assert_eq!(Solution::snakes_and_ladders(board), expected);
    }
}
