///
/// # 803. Bricks Falling When Hit
///
/// You are given an `m x n` binary `grid`, where each `1` represents a brick and `0` represents an empty space. A brick is **stable** if:
///
/// * It is directly connected to the top of the grid, or
/// * At least one other brick in its four adjacent cells is **stable**.
///
/// You are also given an array `hits`, which is a sequence of erasures we want to apply. Each time we want to erase the brick at the location `hits[i] = (row<sub>i</sub>, col<sub>i</sub>)`. The brick on that location (if it exists) will disappear. Some other bricks may no longer be stable because of that erasure and will **fall**. Once a brick falls, it is **immediately** erased from the `grid` (i.e., it does not land on other stable bricks).
///
/// Return *an array* `result`*, where each* `result[i]` *is the number of bricks that will **fall** after the* `i<sup>th</sup>` *erasure is applied.*
///
/// **Note** that an erasure may refer to a location with no brick, and if it does, no bricks drop.
///
/// **Example 1:**
///
/// ```
/// Input: grid = [[1,0,0,0],[1,1,1,0]], hits = [[1,0]]
/// Output: [2]
/// Explanation: Starting with the grid:
/// [[1,0,0,0],
///  [1,1,1,0]]
/// We erase the underlined brick at (1,0), resulting in the grid:
/// [[1,0,0,0],
///  [0,1,1,0]]
/// The two underlined bricks are no longer stable as they are no longer connected to the top nor adjacent to another stable brick, so they will fall. The resulting grid is:
/// [[1,0,0,0],
///  [0,0,0,0]]
/// Hence the result is [2].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: grid = [[1,0,0,0],[1,1,0,0]], hits = [[1,1],[1,0]]
/// Output: [0,0]
/// Explanation: Starting with the grid:
/// [[1,0,0,0],
///  [1,1,0,0]]
/// We erase the underlined brick at (1,1), resulting in the grid:
/// [[1,0,0,0],
///  [1,0,0,0]]
/// All remaining bricks are still stable, so no bricks fall. The grid remains the same:
/// [[1,0,0,0],
///  [1,0,0,0]]
/// Next, we erase the underlined brick at (1,0), resulting in the grid:
/// [[1,0,0,0],
///  [0,0,0,0]]
/// Once again, all remaining bricks are still stable, so no bricks fall.
/// Hence the result is [0,0].
///
/// ```
///
/// **Constraints:**
///
/// * `m == grid.length`
/// * `n == grid[i].length`
/// * `1 <= m, n <= 200`
/// * `grid[i][j]` is `0` or `1`.
/// * `1 <= hits.length <= 4 * 10<sup>4</sup>`
/// * `hits[i].length == 2`
/// * `0 <= x<sub>i&nbsp;</sub><= m - 1`
/// * `0 <= y<sub>i</sub> <= n - 1`
/// * All `(x<sub>i</sub>, y<sub>i</sub>)` are unique.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/bricks-falling-when-hit/
// discuss: https://leetcode.com/problems/bricks-falling-when-hit/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

struct UnionFind {
    data: Vec<usize>,
    size: Vec<i32>,
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

        let (a, b) = match a > b {
            true => (b, a),
            false => (a, b),
        };

        self.size[a] += self.size[b];
        self.data[b] = a;
    }
}

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();

        let mut grid = grid;
        let mut miss_hits = Vec::with_capacity(hits.len());

        for hit in &hits {
            let (i, j) = (hit[0] as usize, hit[1] as usize);

            if grid[i][j] == 0 {
                miss_hits.push(true);
            } else {
                grid[i][j] = 0;
                miss_hits.push(false);
            }
        }

        let mut uf = UnionFind::new(n * m);

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    continue;
                }

                let idx = i * m + j;

                for (di, dj) in [(0, 1), (1, 0)] {
                    let (cur_i, cur_j) = (i + di, j + dj);
                    let cur_idx = cur_i * m + cur_j;

                    if cur_i < n && cur_j < m && grid[cur_i][cur_j] == 1 {
                        uf.union(idx, cur_idx);
                    }
                }
            }
        }

        let mut result = Vec::with_capacity(hits.len());

        for (hit, &miss_hit) in hits.iter().zip(&miss_hits).rev() {
            if miss_hit {
                result.push(0);
                continue;
            }

            let (i, j) = (hit[0] as usize, hit[1] as usize);
            grid[i][j] = 1;

            let mut idx = i * m + j;

            let mut adj_unstable_size = 0;

            for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (adj_i, adj_j) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(adj_i), Some(adj_j)) if adj_i < n && adj_j < m => (adj_i, adj_j),
                    _ => continue,
                };

                if grid[adj_i][adj_j] == 0 {
                    continue;
                }

                let adj_idx = uf.find(adj_i * m + adj_j);

                if idx != adj_idx && adj_idx >= m {
                    adj_unstable_size += uf.size[adj_idx];
                }

                uf.union(idx, adj_idx);

                idx = uf.find(idx);
            }

            if idx < m {
                result.push(adj_unstable_size);
            } else {
                result.push(0);
            }
        }

        result.reverse();
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_803() {
        // let grid = nd_vec![[1, 0, 0, 0], [1, 1, 1, 0]];
        // let hits = nd_vec![[1, 0]];
        // let expected = vec![2];
        // assert_eq!(Solution::hit_bricks(grid, hits), expected);
        // let grid = nd_vec![[1, 0, 0, 0], [1, 1, 0, 0]];
        // let hits = nd_vec![[1, 1], [1, 0]];
        // let expected = vec![0, 0];
        // assert_eq!(Solution::hit_bricks(grid, hits), expected);
        let grid = nd_vec![[1, 0, 1], [0, 0, 1]];
        let hits = nd_vec![[1, 0], [0, 0]];
        let expected = vec![0, 0];
        assert_eq!(Solution::hit_bricks(grid, hits), expected);
    }
}
