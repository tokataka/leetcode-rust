///
/// # 2258. Escape the Spreading Fire
///
/// You are given a **0-indexed** 2D integer array `grid` of size `m x n` which represents a field. Each cell has one of three values:
///
/// * `0` represents grass,
/// * `1` represents fire,
/// * `2` represents a wall that you and fire cannot pass through.
///
/// You are situated in the top-left cell, `(0, 0)`, and you want to travel to the safehouse at the bottom-right cell, `(m - 1, n - 1)`. Every minute, you may move to an **adjacent** grass cell. **After** your move, every fire cell will spread to all **adjacent** cells that are not walls.
///
/// Return *the **maximum** number of minutes that you can stay in your initial position before moving while still safely reaching the safehouse*. If this is impossible, return `-1`. If you can **always** reach the safehouse regardless of the minutes stayed, return `10<sup>9</sup>`.
///
/// Note that even if the fire spreads to the safehouse immediately after you have reached it, it will be counted as safely reaching the safehouse.
///
/// A cell is **adjacent** to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/10/ex1new.jpg)
///
/// ```
/// Input: grid = [[0,2,0,0,0,0,0],[0,0,0,2,2,1,0],[0,2,0,0,1,2,0],[0,0,2,2,2,0,2],[0,0,0,0,0,0,0]]
/// Output: 3
/// Explanation: The figure above shows the scenario where you stay in the initial position for 3 minutes.
/// You will still be able to safely reach the safehouse.
/// Staying for more than 3 minutes will not allow you to safely reach the safehouse.
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/10/ex2new2.jpg)
///
/// ```
/// Input: grid = [[0,0,0,0],[0,1,2,0],[0,2,0,0]]
/// Output: -1
/// Explanation: The figure above shows the scenario where you immediately move towards the safehouse.
/// Fire will spread to any cell you move towards and it is impossible to safely reach the safehouse.
/// Thus, -1 is returned.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/10/ex3new.jpg)
///
/// ```
/// Input: grid = [[0,0,0],[2,2,0],[1,2,0]]
/// Output: 1000000000
/// Explanation: The figure above shows the initial grid.
/// Notice that the fire is contained by walls and you will always be able to safely reach the safehouse.
/// Thus, 109 is returned.
///
/// ```
///
/// **Constraints:**
///
/// * `m == grid.length`
/// * `n == grid[i].length`
/// * `2 <= m, n <= 300`
/// * `4 <= m * n <= 2 * 10<sup>4</sup>`
/// * `grid[i][j]` is either `0`, `1`, or `2`.
/// * `grid[0][0] == grid[m - 1][n - 1] == 0`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/escape-the-spreading-fire/
// discuss: https://leetcode.com/problems/escape-the-spreading-fire/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn maximum_minutes(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut fires = VecDeque::new();

        // -1: grass, -2: wall, 0..: fire times
        for (i, row) in grid.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = match cell {
                    0 => -1,
                    1 => 0,
                    2 => -2,
                    _ => unreachable!(),
                };

                if *cell == 0 {
                    fires.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = fires.pop_front() {
            let fire_time = grid[i][j] + 1;

            for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (ci, cj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(ci), Some(cj)) if ci < n && cj < m => (ci, cj),
                    _ => continue,
                };

                if grid[ci][cj] == -1 {
                    grid[ci][cj] = fire_time;
                    fires.push_back((ci, cj));
                }
            }
        }

        let mut visited = vec![vec![false; m]; n];
        let mut pq = BinaryHeap::from([(i32::MAX, 0, (0, 0))]);

        while let Some((max_wait, move_count, (i, j))) = pq.pop() {
            if visited[i][j] {
                continue;
            }

            visited[i][j] = true;

            if (i, j) == (n - 1, m - 1) {
                return match max_wait {
                    i32::MAX => 10i32.pow(9),
                    x => x - 1,
                };
            }

            let next_move_count = move_count + 1;

            for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (ci, cj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(ci), Some(cj)) if ci < n && cj < m => (ci, cj),
                    _ => continue,
                };

                let next_max_wait = match grid[ci][cj] {
                    -1 => max_wait,
                    -2 => continue,
                    x => max_wait
                        .min(x - next_move_count + if (ci, cj) == (n - 1, m - 1) { 1 } else { 0 }),
                };

                if next_max_wait > 0 {
                    pq.push((next_max_wait, next_move_count, (ci, cj)));
                }
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
    fn test_2258() {
        // let grid = nd_vec![
        //     [0, 2, 0, 0, 0, 0, 0],
        //     [0, 0, 0, 2, 2, 1, 0],
        //     [0, 2, 0, 0, 1, 2, 0],
        //     [0, 0, 2, 2, 2, 0, 2],
        //     [0, 0, 0, 0, 0, 0, 0]
        // ];
        // let expected = 3;
        // assert_eq!(Solution::maximum_minutes(grid), expected);
        // let grid = nd_vec![[0, 0, 0, 0], [0, 1, 2, 0], [0, 2, 0, 0]];
        // let expected = -1;
        // assert_eq!(Solution::maximum_minutes(grid), expected);
        // let grid = nd_vec![[0, 0, 0], [2, 2, 0], [1, 2, 0]];
        // let expected = 1000000000;
        // assert_eq!(Solution::maximum_minutes(grid), expected);
        let grid = nd_vec![
            [0, 2, 0, 0, 1],
            [0, 2, 0, 2, 2],
            [0, 2, 0, 0, 0],
            [0, 0, 2, 2, 0],
            [0, 0, 0, 0, 0]
        ];
        let expected = 0;
        assert_eq!(Solution::maximum_minutes(grid), expected);
    }
}
