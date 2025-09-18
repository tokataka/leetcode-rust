///
/// # 37. Sudoku Solver
///
/// Write a program to solve a Sudoku puzzle by filling the empty cells.
///
/// A sudoku solution must satisfy **all of the following rules**:
///
/// 1. Each of the digits `1-9` must occur exactly once in each row.
/// 2. Each of the digits `1-9` must occur exactly once in each column.
/// 3. Each of the digits `1-9` must occur exactly once in each of the 9 `3x3` sub-boxes of the grid.
///
/// The `'.'` character indicates empty cells.
///
/// **Example 1:**
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)
///
/// ```
/// Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
/// Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
/// Explanation: The input board is shown above and the only valid solution is shown below:
///
/// ```
///
/// **Constraints:**
///
/// * `board.length == 9`
/// * `board[i].length == 9`
/// * `board[i][j]` is a digit or `'.'`.
/// * It is **guaranteed** that the input board has only one solution.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/sudoku-solver/
// discuss: https://leetcode.com/problems/sudoku-solver/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn _solve_sudoku(ij: usize, board: &mut Vec<Vec<char>>) -> bool {
            if ij >= 81 {
                return true;
            }

            let (i, j) = (ij / 9, ij % 9);

            if board[i][j] != '.' {
                return _solve_sudoku(ij + 1, board);
            }

            'outer: for digit in '1'..='9' {
                // row
                for jj in 0..9 {
                    if board[i][jj] == digit {
                        continue 'outer;
                    }
                }

                // col
                for ii in 0..9 {
                    if board[ii][j] == digit {
                        continue 'outer;
                    }
                }

                // grid
                for ii in 0..3 {
                    for jj in 0..3 {
                        if board[i - i % 3 + ii][j - j % 3 + jj] == digit {
                            continue 'outer;
                        }
                    }
                }

                board[i][j] = digit;

                if _solve_sudoku(ij + 1, board) {
                    return true;
                }

                board[i][j] = '.';
            }

            false
        }

        _solve_sudoku(0, board);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_37() {
        let mut board = nd_vec![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        let expected = [
            ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, expected);
    }
}
