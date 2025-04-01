use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 2503. Maximum Number of Points From Grid Queries
///
/// You are given an `m x n` integer matrix `grid` and an array `queries` of size `k`.
///
/// Find an array `answer` of size `k` such that for each integer `queries[i]` you start in the **top left** cell of the matrix and repeat the following process:
///
/// * If `queries[i]` is **strictly** greater than the value of the current cell that you are in, then you get one point if it is your first time visiting this cell, and you can move to any **adjacent** cell in all `4` directions: up, down, left, and right.
/// * Otherwise, you do not get any points, and you end this process.
///
/// After the process, `answer[i]` is the **maximum** number of points you can get. **Note** that for each query you are allowed to visit the same cell **multiple** times.
///
/// Return *the resulting array* `answer`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2025/03/15/image1.png)
///
/// ```
/// Input: grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
/// Output: [5,8,1]
/// Explanation: The diagrams above show which cells we visit to get points for each query.
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/10/20/yetgriddrawio-2.png)
///
/// ```
/// Input: grid = [[5,2,1],[1,1,2]], queries = [3]
/// Output: [0]
/// Explanation: We can not get any points because the value of the top left cell is already greater than or equal to 3.
///
/// ```
///
/// **Constraints:**
///
/// * `m == grid.length`
/// * `n == grid[i].length`
/// * `2 <= m, n <= 1000`
/// * `4 <= m * n <= 10<sup>5</sup>`
/// * `k == queries.length`
/// * `1 <= k <= 10<sup>4</sup>`
/// * `1 <= grid[i][j], queries[i] <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/
// discuss: https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let (height, width) = (grid.len(), grid[0].len());
        let mut score_count = vec![(0, 0)];
        let mut value = grid[0][0];
        let mut score = 0;
        let mut visited = vec![vec![false; width]; height];
        visited[0][0] = true;

        let mut pq = BinaryHeap::from([(Reverse(grid[0][0]), (0_usize, 0_usize))]);

        while let Some((Reverse(v), (i, j))) = pq.pop() {
            if value < v {
                score_count.push((value, score));
                value = v;
            }

            score += 1;

            for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ci, cj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(ci), Some(cj)) if ci < height && cj < width => (ci, cj),
                    _ => continue,
                };

                if visited[ci][cj] {
                    continue;
                }

                visited[ci][cj] = true;

                pq.push((Reverse(grid[ci][cj]), (ci, cj)));
            }
        }

        score_count.push((value, score));

        queries
            .into_iter()
            .map(|query| {
                let idx = score_count.partition_point(|&x| x.0 < query) - 1;
                score_count[idx].1
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2503() {
        let grid = nd_vec![[1, 2, 3], [2, 5, 7], [3, 5, 1]];
        let queries = vec![5, 6, 2];
        let expected = vec![5, 8, 1];
        assert_eq!(Solution::max_points(grid, queries), expected);
        let grid = nd_vec![[5, 2, 1], [1, 1, 2]];
        let queries = vec![3];
        let expected = vec![0];
        assert_eq!(Solution::max_points(grid, queries), expected);
    }
}
