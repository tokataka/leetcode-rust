///
/// # 2658. Maximum Number of Fish in a Grid
///
/// You are given a **0-indexed** 2D matrix `grid` of size `m x n`, where `(r, c)` represents:
///
/// * A **land** cell if `grid[r][c] = 0`, or
/// * A **water** cell containing `grid[r][c]` fish, if `grid[r][c] > 0`.
///
/// A fisher can start at any **water** cell `(r, c)` and can do the following operations any number of times:
///
/// * Catch all the fish at cell `(r, c)`, or
/// * Move to any adjacent **water** cell.
///
/// Return *the **maximum** number of fish the fisher can catch if he chooses his starting cell optimally, or* `0` if no water cell exists.
///
/// An **adjacent** cell of the cell `(r, c)`, is one of the cells `(r, c + 1)`, `(r, c - 1)`, `(r + 1, c)` or `(r - 1, c)` if it exists.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2023/03/29/example.png)
///
/// ```
/// Input: grid = [[0,2,1,0],[4,0,0,3],[1,0,0,4],[0,3,2,0]]
/// Output: 7
/// Explanation: The fisher can start at cell (1,3) and collect 3 fish, then move to cell (2,3) and collect 4 fish.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2023/03/29/example2.png)
///
/// ```
/// Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,1]]
/// Output: 1
/// Explanation: The fisher can start at cells (0,0) or (3,3) and collect a single fish.
///
/// ```
///
/// **Constraints:**
///
/// * `m == grid.length`
/// * `n == grid[i].length`
/// * `1 <= m, n <= 10`
/// * `0 <= grid[i][j] <= 10`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/
// discuss: https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let height = grid.len();
        let width = grid[0].len();

        let mut visited = vec![vec![false; width]; height];

        let mut result = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    continue;
                }

                if visited[i][j] {
                    continue;
                }

                visited[i][j] = true;

                let mut sum = cell;
                let mut st = vec![(i, j)];

                while let Some((i, j)) = st.pop() {
                    for (di, dj) in DIRECTIONS {
                        let (ci, cj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                            (Some(ci), Some(cj)) if ci < height && cj < width => (ci, cj),
                            _ => continue,
                        };

                        if grid[ci][cj] == 0 {
                            continue;
                        }

                        if visited[ci][cj] {
                            continue;
                        }

                        visited[ci][cj] = true;
                        sum += grid[ci][cj];
                        st.push((ci, cj));
                    }
                }

                result = result.max(sum);
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
    fn test_2658() {
        let grid = nd_vec![[0, 2, 1, 0], [4, 0, 0, 3], [1, 0, 0, 4], [0, 3, 2, 0]];
        let expected = 7;
        assert_eq!(Solution::find_max_fish(grid), expected);
        let grid = nd_vec![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]];
        let expected = 1;
        assert_eq!(Solution::find_max_fish(grid), expected);
    }
}
