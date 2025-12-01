///
/// # 794. Valid Tic-Tac-Toe State
///
/// Given a Tic-Tac-Toe board as a string array `board`, return `true` if and only if it is possible to reach this board position during the course of a valid tic-tac-toe game.
///
/// The board is a `3 x 3` array that consists of characters `' '`, `'X'`, and `'O'`. The `' '` character represents an empty square.
///
/// Here are the rules of Tic-Tac-Toe:
///
/// * Players take turns placing characters into empty squares `' '`.
/// * The first player always places `'X'` characters, while the second player always places `'O'` characters.
/// * `'X'` and `'O'` characters are always placed into empty squares, never filled ones.
/// * The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
/// * The game also ends if all squares are non-empty.
/// * No more moves can be played if the game is over.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/05/15/tictactoe1-grid.jpg)
///
/// ```
/// Input: board = ["O  ","   ","   "]
/// Output: false
/// Explanation: The first player always plays "X".
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/05/15/tictactoe2-grid.jpg)
///
/// ```
/// Input: board = ["XOX"," X ","   "]
/// Output: false
/// Explanation: Players take turns making moves.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/05/15/tictactoe4-grid.jpg)
///
/// ```
/// Input: board = ["XOX","O O","XOX"]
/// Output: true
///
/// ```
///
/// **Constraints:**
///
/// * `board.length == 3`
/// * `board[i].length == 3`
/// * `board[i][j]` is either `'X'`, `'O'`, or `' '`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-tic-tac-toe-state/
// discuss: https://leetcode.com/problems/valid-tic-tac-toe-state/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let board = board
            .into_iter()
            .map(|row| row.into_bytes())
            .collect::<Vec<_>>();

        let mut x_o_diff = 0;
        let mut x_win = false;
        let mut o_win = false;

        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == b'X' {
                    x_o_diff += 1;
                }

                if board[i][j] == b'O' {
                    x_o_diff -= 1;
                }
            }
        }

        if (0..3).any(|i| (0..3).all(|j| board[i][j] == b'X'))
            || (0..3).any(|i| (0..3).all(|j| board[j][i] == b'X'))
            || (0..3).all(|i| board[i][i] == b'X')
            || (0..3).all(|i| board[i][2 - i] == b'X')
        {
            x_win = true;
        }

        if (0..3).any(|i| (0..3).all(|j| board[i][j] == b'O'))
            || (0..3).any(|i| (0..3).all(|j| board[j][i] == b'O'))
            || (0..3).all(|i| board[i][i] == b'O')
            || (0..3).all(|i| board[i][2 - i] == b'O')
        {
            o_win = true;
        }

        matches!((x_o_diff, x_win, o_win), (1, _, false) | (0, false, _))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_794() {
        let board = vec_string!["O  ", "   ", "   "];
        let expected = false;
        assert_eq!(Solution::valid_tic_tac_toe(board), expected);
        let board = vec_string!["XOX", " X ", "   "];
        let expected = false;
        assert_eq!(Solution::valid_tic_tac_toe(board), expected);
        let board = vec_string!["XOX", "O O", "XOX"];
        let expected = true;
        assert_eq!(Solution::valid_tic_tac_toe(board), expected);
    }
}
