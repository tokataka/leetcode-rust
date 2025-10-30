///
/// # 778. Swim in Rising Water
///
/// You are given an `n x n` integer matrix `grid` where each value `grid[i][j]` represents the elevation at that point `(i, j)`.
///
/// It starts raining, and water gradually rises over time. At time `t`, the water level is `t`, meaning **any** cell with elevation less than equal to `t` is submerged or reachable.
///
/// You can swim from a square to another 4-directionally adjacent square if and only if the elevation of both squares individually are at most `t`. You can swim infinite distances in zero time. Of course, you must stay within the boundaries of the grid during your swim.
///
/// Return *the minimum time until you can reach the bottom right square* `(n - 1, n - 1)` *if you start at the top left square* `(0, 0)`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/06/29/swim1-grid.jpg)
///
/// ```
/// Input: grid = [[0,2],[1,3]]
/// Output: 3
/// Explanation:
/// At time 0, you are in grid location (0, 0).
/// You cannot go anywhere else because 4-directionally adjacent neighbors have a higher elevation than t = 0.
/// You cannot reach point (1, 1) until time 3.
/// When the depth of water is 3, we can swim anywhere inside the grid.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/06/29/swim2-grid-1.jpg)
///
/// ```
/// Input: grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
/// Output: 16
/// Explanation: The final route is shown.
/// We need to wait until time 16 so that (0, 0) and (4, 4) are connected.
///
/// ```
///
/// **Constraints:**
///
/// * `n == grid.length`
/// * `n == grid[i].length`
/// * `1 <= n <= 50`
/// * `0 <= grid[i][j] < n<sup>2</sup>`
/// * Each value `grid[i][j]` is **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/swim-in-rising-water/
// discuss: https://leetcode.com/problems/swim-in-rising-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

struct UnionFind {
    data: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            data: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if a == self.data[a] {
            return a;
        }

        self.data[a] = self.find(self.data[a]);

        self.data[a]
    }

    fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return;
        }

        let (a, b) = match self.size[a] < self.size[b] {
            true => (b, a),
            false => (a, b),
        };

        self.size[a] += self.size[b];
        self.data[b] = a;
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let mut grid_map = (0..n * n).collect::<Vec<_>>();
        grid_map.sort_unstable_by_key(|ij| grid[ij / n][ij % n]);

        let mut uf = UnionFind::new(n * n);

        for (time, ij) in grid_map.into_iter().enumerate() {
            let (i, j) = (ij / n, ij % n);

            for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (ci, cj) = match (i.checked_add_signed(di), (j.checked_add_signed(dj))) {
                    (Some(ci), Some(cj)) if ci < n && cj < n => (ci, cj),
                    _ => continue,
                };

                if grid[i][j] < grid[ci][cj] {
                    continue;
                }

                uf.union(ij, ci * n + cj);
            }

            if uf.find(0) == uf.find(n * n - 1) {
                return time as i32;
            }
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_778() {
        let grid = nd_vec![[0, 2], [1, 3]];
        let expected = 3;
        assert_eq!(Solution::swim_in_water(grid), expected);
        let grid = nd_vec![
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6]
        ];
        let expected = 16;
        assert_eq!(Solution::swim_in_water(grid), expected);
    }
}
