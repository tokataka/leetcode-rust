///
/// # 1267. Count Servers that Communicate
///
/// You are given a map of a server center, represented as a `m * n` integer matrix `grid`, where 1 means that on that cell there is a server and 0 means that it is no server. Two servers are said to communicate if they are on the same row or on the same column.
///
/// Return the number of servers that communicate with any other server.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2019/11/14/untitled-diagram-6.jpg)
///
/// ```
/// Input: grid = [[1,0],[0,1]]
/// Output: 0
/// Explanation: No servers can communicate with others.
/// ```
///
/// **Example 2:**
///
/// **![](https://assets.leetcode.com/uploads/2019/11/13/untitled-diagram-4.jpg)**
///
/// ```
/// Input: grid = [[1,0],[1,1]]
/// Output: 3
/// Explanation: All three servers can communicate with at least one other server.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2019/11/14/untitled-diagram-1-3.jpg)
///
/// ```
/// Input: grid = [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]
/// Output: 4
/// Explanation: The two servers in the first row can communicate with each other. The two servers in the third column can communicate with each other. The server at right bottom corner can't communicate with any other server.
///
/// ```
///
/// **Constraints:**
///
/// * `m == grid.length`
/// * `n == grid[i].length`
/// * `1 <= m <= 250`
/// * `1 <= n <= 250`
/// * `grid[i][j] == 0 or 1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-servers-that-communicate/
// discuss: https://leetcode.com/problems/count-servers-that-communicate/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct UnionFind {
    data: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            data: (0..size).collect(),
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        if self.data[a] == a {
            return a;
        }

        self.data[a] = self.find(self.data[a]);

        self.data[a]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);

        if a != b {
            self.data[b] = a;
        }
    }
}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(grid[0].len());
        let mut col_counts = vec![0; grid[0].len()];

        for row in &grid {
            let mut s = None;

            for (j, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    continue;
                }

                col_counts[j] += 1;

                if let Some(s) = s {
                    uf.union(s, j);
                } else {
                    s = Some(j);
                }
            }
        }

        let mut counts = vec![0; grid[0].len()];

        for j in 0..grid[0].len() {
            counts[uf.find(j)] += col_counts[j];
        }

        counts.into_iter().filter(|&count| count > 1).sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1267() {
        let grid = nd_vec![[1, 0], [0, 1]];
        let expected = 0;
        assert_eq!(Solution::count_servers(grid), expected);
        let grid = nd_vec![[1, 0], [1, 1]];
        let expected = 3;
        assert_eq!(Solution::count_servers(grid), expected);
        let grid = nd_vec![[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
        let expected = 4;
        assert_eq!(Solution::count_servers(grid), expected);
    }
}
