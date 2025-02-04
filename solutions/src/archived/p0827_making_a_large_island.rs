///
/// # 827. Making A Large Island
///
/// You are given an `n x n` binary matrix `grid`. You are allowed to change **at most one** `0` to be `1`.
///
/// Return *the size of the largest **island** in* `grid` *after applying this operation*.
///
/// An **island** is a 4-directionally connected group of `1`s.
///
/// **Example 1:**
///
/// ```
/// Input: grid = [[1,0],[0,1]]
/// Output: 3
/// Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: grid = [[1,1],[1,0]]
/// Output: 4
/// Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: grid = [[1,1],[1,1]]
/// Output: 4
/// Explanation: Can't change any 0 to 1, only one island with area = 4.
///
/// ```
///
/// **Constraints:**
///
/// * `n == grid.length`
/// * `n == grid[i].length`
/// * `1 <= n <= 500`
/// * `grid[i][j]` is either `0` or `1`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/making-a-large-island/
// discuss: https://leetcode.com/problems/making-a-large-island/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;

type Pos = (usize, usize);

#[derive(Debug)]
struct UnionFind {
    data: Vec<Vec<Option<(usize, usize)>>>,
}

impl UnionFind {
    fn new(height: usize, width: usize) -> Self {
        Self {
            data: vec![vec![None; width]; height],
        }
    }

    fn activate(&mut self, a: Pos) {
        if self.data[a.0][a.1].is_none() {
            self.data[a.0][a.1] = Some(a);
        }
    }

    fn find(&mut self, a: Pos) -> Option<Pos> {
        match self.data[a.0][a.1] {
            None => return None,
            Some(x) if x == a => return Some(x),
            _ => (),
        }

        self.data[a.0][a.1] = self.find(self.data[a.0][a.1].unwrap());

        self.data[a.0][a.1]
    }

    fn union(&mut self, a: Pos, b: Pos) {
        let (a, b) = match (self.find(a), self.find(b)) {
            (Some(a), Some(b)) => (a, b),
            _ => return,
        };

        self.data[b.0][b.1] = Some(a);
    }

    fn count(&mut self) -> (Vec<Vec<usize>>, usize) {
        let mut result = vec![vec![0; self.data[0].len()]; self.data.len()];
        let mut max = 0;

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                if let Some(x) = self.find((i, j)) {
                    result[x.0][x.1] += 1;
                    max = max.max(result[x.0][x.1]);
                }
            }
        }

        (result, max)
    }
}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTION: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let height = grid.len();
        let width = grid[0].len();

        let mut uf = UnionFind::new(height, width);
        let mut visited = vec![vec![false; width]; height];
        let mut zeros = vec![];

        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    zeros.push((i, j));
                    continue;
                }

                if visited[i][j] {
                    continue;
                }

                let mut st = vec![(i, j)];
                visited[i][j] = true;

                uf.activate((i, j));

                while let Some((i, j)) = st.pop() {
                    for (di, dj) in DIRECTION {
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

                        uf.activate((ci, cj));
                        uf.union((i, j), (ci, cj));

                        st.push((ci, cj));
                    }
                }
            }
        }

        let (counts, count_max) = uf.count();

        let mut result = count_max;
        let mut visited = HashSet::new();

        for (i, j) in zeros {
            let mut cur_count = 1;
            visited.clear();

            for (di, dj) in DIRECTION {
                let (ci, cj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(ci), Some(cj)) if ci < height && cj < width => (ci, cj),
                    _ => continue,
                };

                if let Some((ci, cj)) = uf.find((ci, cj)) {
                    if visited.contains(&(ci, cj)) {
                        continue;
                    }

                    visited.insert((ci, cj));

                    cur_count += counts[ci][cj];
                }
            }

            result = result.max(cur_count);
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_827() {
        let grid = nd_vec![[1, 0], [0, 1]];
        let expected = 3;
        assert_eq!(Solution::largest_island(grid), expected);
        let grid = nd_vec![[1, 1], [1, 0]];
        let expected = 4;
        assert_eq!(Solution::largest_island(grid), expected);
        let grid = nd_vec![[1, 1], [1, 1]];
        let expected = 4;
        assert_eq!(Solution::largest_island(grid), expected);
    }
}
