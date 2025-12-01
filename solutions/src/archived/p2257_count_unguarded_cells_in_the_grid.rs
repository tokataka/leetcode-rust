///
/// # 2257. Count Unguarded Cells in the Grid
///
/// You are given two integers `m` and `n` representing a **0-indexed** `m x n` grid. You are also given two 2D integer arrays `guards` and `walls` where `guards[i] = [row<sub>i</sub>, col<sub>i</sub>]` and `walls[j] = [row<sub>j</sub>, col<sub>j</sub>]` represent the positions of the `i<sup>th</sup>` guard and `j<sup>th</sup>` wall respectively.
///
/// A guard can see **every** cell in the four cardinal directions (north, east, south, or west) starting from their position unless **obstructed** by a wall or another guard. A cell is **guarded** if there is **at least** one guard that can see it.
///
/// Return *the number of unoccupied cells that are **not** **guarded**.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/10/example1drawio2.png)
///
/// ```
/// Input: m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
/// Output: 7
/// Explanation: The guarded and unguarded cells are shown in red and green respectively in the above diagram.
/// There are a total of 7 unguarded cells, so we return 7.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/10/example2drawio.png)
///
/// ```
/// Input: m = 3, n = 3, guards = [[1,1]], walls = [[0,1],[1,0],[2,1],[1,2]]
/// Output: 4
/// Explanation: The unguarded cells are shown in green in the above diagram.
/// There are a total of 4 unguarded cells, so we return 4.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= m, n <= 10<sup>5</sup>`
/// * `2 <= m * n <= 10<sup>5</sup>`
/// * `1 <= guards.length, walls.length <= 5 * 10<sup>4</sup>`
/// * `2 <= guards.length + walls.length <= m * n`
/// * `guards[i].length == walls[j].length == 2`
/// * `0 <= row<sub>i</sub>, row<sub>j</sub> < m`
/// * `0 <= col<sub>i</sub>, col<sub>j</sub> < n`
/// * All the positions in `guards` and `walls` are **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-unguarded-cells-in-the-grid/
// discuss: https://leetcode.com/problems/count-unguarded-cells-in-the-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (n as usize, m as usize);

        let mut board = vec![vec![0; m]; n];
        let mut result = (m * n) as i32;

        for guard in &guards {
            board[guard[0] as usize][guard[1] as usize] = 1;
            result -= 1;
        }

        for wall in &walls {
            board[wall[0] as usize][wall[1] as usize] = 2;
            result -= 1;
        }

        for guard in &guards {
            let (i, j) = (guard[0] as usize, guard[1] as usize);

            for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (mut ci, mut cj) = (i, j);

                loop {
                    (ci, cj) = match (ci.checked_add_signed(di), cj.checked_add_signed(dj)) {
                        (Some(ci), Some(cj)) if ci < n && cj < m => (ci, cj),
                        _ => break,
                    };

                    if board[ci][cj] == 1 || board[ci][cj] == 2 {
                        break;
                    }

                    if board[ci][cj] == 0 {
                        result -= 1;
                        board[ci][cj] = 3;
                    }
                }
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
    fn test_2257() {
        let m = 4;
        let n = 6;
        let guards = nd_vec![[0, 0], [1, 1], [2, 3]];
        let walls = nd_vec![[0, 1], [2, 2], [1, 4]];
        let expected = 7;
        assert_eq!(Solution::count_unguarded(m, n, guards, walls), expected);
        let m = 3;
        let n = 3;
        let guards = nd_vec![[1, 1]];
        let walls = nd_vec![[0, 1], [1, 0], [2, 1], [1, 2]];
        let expected = 4;
        assert_eq!(Solution::count_unguarded(m, n, guards, walls), expected);
    }
}
