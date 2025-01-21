use std::collections::HashSet;

///
/// # 1368. Minimum Cost to Make at Least One Valid Path in a Grid
///
/// Given an `m x n` grid. Each cell of the grid has a sign pointing to the next cell you should visit if you are currently in this cell. The sign of `grid[i][j]` can be:
///
/// * `1` which means go to the cell to the right. (i.e go from `grid[i][j]` to `grid[i][j + 1]`)
/// * `2` which means go to the cell to the left. (i.e go from `grid[i][j]` to `grid[i][j - 1]`)
/// * `3` which means go to the lower cell. (i.e go from `grid[i][j]` to `grid[i + 1][j]`)
/// * `4` which means go to the upper cell. (i.e go from `grid[i][j]` to `grid[i - 1][j]`)
///
/// Notice that there could be some signs on the cells of the grid that point outside the grid.
///
/// You will initially start at the upper left cell `(0, 0)`. A valid path in the grid is a path that starts from the upper left cell `(0, 0)` and ends at the bottom-right cell `(m - 1, n - 1)` following the signs on the grid. The valid path does not have to be the shortest.
///
/// You can modify the sign on a cell with `cost = 1`. You can modify the sign on a cell **one time only**.
///
/// Return *the minimum cost to make the grid have at least one valid path*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/02/13/grid1.png)
///
/// ```
/// Input: grid = [[1,1,1,1],[2,2,2,2],[1,1,1,1],[2,2,2,2]]
/// Output: 3
/// Explanation: You will start at point (0, 0).
/// The path to (3, 3) is as follows. (0, 0) --> (0, 1) --> (0, 2) --> (0, 3) change the arrow to down with cost = 1 --> (1, 3) --> (1, 2) --> (1, 1) --> (1, 0) change the arrow to down with cost = 1 --> (2, 0) --> (2, 1) --> (2, 2) --> (2, 3) change the arrow to down with cost = 1 --> (3, 3)
/// The total cost = 3.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/02/13/grid2.png)
///
/// ```
/// Input: grid = [[1,1,3],[3,2,2],[1,1,4]]
/// Output: 0
/// Explanation: You can follow the path from (0, 0) to (2, 2).
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2020/02/13/grid3.png)
///
/// ```
/// Input: grid = [[1,2],[4,3]]
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `m == grid.length`
/// * `n == grid[i].length`
/// * `1 <= m, n <= 100`
/// * `1 <= grid[i][j] <= 4`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/
// discuss: https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        // [0: USELESS, 1: RIGHT, 2: LEFT, 3: DOWN, 4: UP]
        const DIRECTION: [(i32, i32); 5] = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];
        let height = grid.len();
        let width = grid[0].len();

        let mut st = vec![(0, 0)];
        let mut visited = HashSet::from([(0, 0)]);
        let mut next_st = HashSet::new();
        let mut next_visited = HashSet::new();

        for cost in 0.. {
            while let Some((ci, cj)) = st.pop() {
                if (ci, cj) == (height - 1, width - 1) {
                    return cost;
                }

                let cur_dir = grid[ci][cj] as usize;

                for (d, &(di, dj)) in DIRECTION.iter().enumerate().skip(1) {
                    let (i, j) = match (
                        ci.checked_add_signed(di as isize),
                        cj.checked_add_signed(dj as isize),
                    ) {
                        (Some(i), Some(j)) if i < height && j < width => (i, j),
                        _ => continue,
                    };

                    if cur_dir == d {
                        if visited.contains(&(i, j)) {
                            continue;
                        }

                        visited.insert((i, j));
                        next_visited.insert((i, j));
                        st.push((i, j));
                    } else {
                        if next_visited.contains(&(i, j)) {
                            continue;
                        }

                        next_visited.insert((i, j));
                        next_st.insert((i, j));
                    }
                }
            }

            st = Vec::from_iter(&next_st - &visited);

            visited = next_visited.clone();

            next_st = HashSet::new();
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1368() {
        let grid = nd_vec![[1, 1, 1, 1], [2, 2, 2, 2], [1, 1, 1, 1], [2, 2, 2, 2]];
        let expected = 3;
        assert_eq!(Solution::min_cost(grid), expected);
        let grid = nd_vec![[1, 1, 3], [3, 2, 2], [1, 1, 4]];
        let expected = 0;
        assert_eq!(Solution::min_cost(grid), expected);
        let grid = nd_vec![[1, 2], [4, 3]];
        let expected = 1;
        assert_eq!(Solution::min_cost(grid), expected);
    }
}
