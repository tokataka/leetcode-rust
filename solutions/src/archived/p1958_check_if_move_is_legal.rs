///
/// # 1958. Check if Move is Legal
///
/// You are given a **0-indexed** `8 x 8` grid `board`, where `board[r][c]` represents the cell `(r, c)` on a game board. On the board, free cells are represented by `'.'`, white cells are represented by `'W'`, and black cells are represented by `'B'`.
///
/// Each move in this game consists of choosing a free cell and changing it to the color you are playing as (either white or black). However, a move is only **legal** if, after changing it, the cell becomes the **endpoint of a good line** (horizontal, vertical, or diagonal).
///
/// A **good line** is a line of **three or more cells (including the endpoints)** where the endpoints of the line are **one color**, and the remaining cells in the middle are the **opposite color** (no cells in the line are free). You can find examples for good lines in the figure below:
///
/// ![](https://assets.leetcode.com/uploads/2021/07/22/goodlines5.png)
///
/// Given two integers `rMove` and `cMove` and a character `color` representing the color you are playing as (white or black), return `true` *if changing cell* `(rMove, cMove)` *to color* `color` *is a **legal** move, or* `false` *if it is not legal*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/07/10/grid11.png)
///
/// ```
/// Input: board = [[".",".",".","B",".",".",".","."],[".",".",".","W",".",".",".","."],[".",".",".","W",".",".",".","."],[".",".",".","W",".",".",".","."],["W","B","B",".","W","W","W","B"],[".",".",".","B",".",".",".","."],[".",".",".","B",".",".",".","."],[".",".",".","W",".",".",".","."]], rMove = 4, cMove = 3, color = "B"
/// Output: true
/// Explanation: '.', 'W', and 'B' are represented by the colors blue, white, and black respectively, and cell (rMove, cMove) is marked with an 'X'.
/// The two good lines with the chosen cell as an endpoint are annotated above with the red rectangles.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/07/10/grid2.png)
///
/// ```
/// Input: board = [[".",".",".",".",".",".",".","."],[".","B",".",".","W",".",".","."],[".",".","W",".",".",".",".","."],[".",".",".","W","B",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".","B","W",".","."],[".",".",".",".",".",".","W","."],[".",".",".",".",".",".",".","B"]], rMove = 4, cMove = 4, color = "W"
/// Output: false
/// Explanation: While there are good lines with the chosen cell as a middle cell, there are no good lines with the chosen cell as an endpoint.
///
/// ```
///
/// **Constraints:**
///
/// * `board.length == board[r].length == 8`
/// * `0 <= rMove, cMove < 8`
/// * `board[rMove][cMove] == '.'`
/// * `color` is either `'B'` or `'W'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-move-is-legal/
// discuss: https://leetcode.com/problems/check-if-move-is-legal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        const DIRECTION: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let (start_i, start_j) = (r_move as usize, c_move as usize);
        let height = board.len();
        let width = board[0].len();

        let inner = match color {
            'W' => 'B',
            'B' => 'W',
            _ => unreachable!(),
        };

        for (di, dj) in DIRECTION {
            let (mut i, mut j) = match (
                start_i.checked_add_signed(di),
                start_j.checked_add_signed(dj),
            ) {
                (Some(i), Some(j)) if i < height && j < width => (i, j),
                _ => continue,
            };

            if board[i][j] != inner {
                continue;
            }

            loop {
                (i, j) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(i), Some(j)) if i < height && j < width => (i, j),
                    _ => break,
                };

                match board[i][j] {
                    '.' => break,
                    x if x == inner => (),
                    x if x == color => return true,
                    _ => unreachable!(),
                };
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1958() {
        let board = nd_vec![
            ['.', '.', '.', 'B', '.', '.', '.', '.'],
            ['.', '.', '.', 'W', '.', '.', '.', '.'],
            ['.', '.', '.', 'W', '.', '.', '.', '.'],
            ['.', '.', '.', 'W', '.', '.', '.', '.'],
            ['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
            ['.', '.', '.', 'B', '.', '.', '.', '.'],
            ['.', '.', '.', 'B', '.', '.', '.', '.'],
            ['.', '.', '.', 'W', '.', '.', '.', '.']
        ];
        let r_move = 4;
        let c_move = 3;
        let color = 'B';
        let expected = true;
        assert_eq!(Solution::check_move(board, r_move, c_move, color), expected);
        let board = nd_vec![
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', 'B', '.', '.', 'W', '.', '.', '.'],
            ['.', '.', 'W', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'W', 'B', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', 'B', 'W', '.', '.'],
            ['.', '.', '.', '.', '.', '.', 'W', '.'],
            ['.', '.', '.', '.', '.', '.', '.', 'B']
        ];
        let r_move = 4;
        let c_move = 4;
        let color = 'W';
        let expected = false;
        assert_eq!(Solution::check_move(board, r_move, c_move, color), expected);
    }
}
