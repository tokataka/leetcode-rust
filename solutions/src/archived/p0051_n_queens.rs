///
/// # 51. N-Queens
///
/// The **n-queens** puzzle is the problem of placing `n` queens on an `n x n` chessboard such that no two queens attack each other.
///
/// Given an integer `n`, return *all distinct solutions to the **n-queens puzzle***. You may return the answer in **any order**.
///
/// Each solution contains a distinct board configuration of the n-queens' placement, where `'Q'` and `'.'` both indicate a queen and an empty space, respectively.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/11/13/queens.jpg)
///
/// ```
/// Input: n = 4
/// Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
/// Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 1
/// Output: [["Q"]]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 9`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens/
// discuss: https://leetcode.com/problems/n-queens/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn _n_queens(i: usize, queens: &mut Vec<usize>, n: usize) -> Vec<Vec<String>> {
            if i == n {
                let mut board = vec![];
                for &queen in queens.iter() {
                    let mut row = ".".repeat(n);
                    row.replace_range(queen..queen + 1, "Q");
                    board.push(row);
                }

                return vec![board];
            }

            let mut boards = vec![];

            for j in 0..n {
                if queens
                    .iter()
                    .enumerate()
                    .any(|(qi, &qj)| j == qj || i - qi == j.abs_diff(qj))
                {
                    continue;
                }

                queens.push(j);
                boards.extend_from_slice(&_n_queens(i + 1, queens, n));
                queens.pop();
            }

            boards
        }

        _n_queens(0, &mut vec![], n as usize)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51() {
        let n = 4;
        let expected = nd_vec_string![
            [".Q..", "...Q", "Q...", "..Q."],
            ["..Q.", "Q...", "...Q", ".Q.."]
        ];
        assert_eq!(Solution::solve_n_queens(n), expected);
        let n = 1;
        let expected = nd_vec_string![["Q"]];
        assert_eq!(Solution::solve_n_queens(n), expected);
    }
}
