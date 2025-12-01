///
/// # 3607. Power Grid Maintenance
///
/// You are given an integer `c` representing `c` power stations, each with a unique identifier `id` from 1 to `c` (1‑based indexing).
///
/// These stations are interconnected via `n` **bidirectional** cables, represented by a 2D array `connections`, where each element `connections[i] = [u<sub>i</sub>, v<sub>i</sub>]` indicates a connection between station `u<sub>i</sub>` and station `v<sub>i</sub>`. Stations that are directly or indirectly connected form a **power grid**.
///
/// Initially, **all** stations are online (operational).
///
/// You are also given a 2D array `queries`, where each query is one of the following *two* types:
///
/// * `[1, x]`: A maintenance check is requested for station `x`. If station `x` is online, it resolves the check by itself. If station `x` is offline, the check is resolved by the operational station with the smallest `id` in the same **power grid** as `x`. If **no** **operational** station *exists* in that grid, return -1.
///
/// * `[2, x]`: Station `x` goes offline (i.e., it becomes non-operational).
///
/// Return an array of integers representing the results of each query of type `[1, x]` in the **order** they appear.
///
/// **Note:** The power grid preserves its structure; an offline (non‑operational) node remains part of its grid and taking it offline does not alter connectivity.
///
/// **Example 1:**
///
/// **Input:** c = 5, connections = [[1,2],[2,3],[3,4],[4,5]], queries = [[1,3],[2,1],[1,1],[2,2],[1,2]]
///
/// **Output:** [3,2,3]
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/04/15/powergrid.jpg)
///
/// * Initially, all stations `{1, 2, 3, 4, 5}` are online and form a single power grid.
/// * Query `[1,3]`: Station 3 is online, so the maintenance check is resolved by station 3.
/// * Query `[2,1]`: Station 1 goes offline. The remaining online stations are `{2, 3, 4, 5}`.
/// * Query `[1,1]`: Station 1 is offline, so the check is resolved by the operational station with the smallest `id` among `{2, 3, 4, 5}`, which is station 2.
/// * Query `[2,2]`: Station 2 goes offline. The remaining online stations are `{3, 4, 5}`.
/// * Query `[1,2]`: Station 2 is offline, so the check is resolved by the operational station with the smallest `id` among `{3, 4, 5}`, which is station 3.
///
/// **Example 2:**
///
/// **Input:** c = 3, connections = [], queries = [[1,1],[2,1],[1,1]]
///
/// **Output:** [1,-1]
///
/// **Explanation:**
///
/// * There are no connections, so each station is its own isolated grid.
/// * Query `[1,1]`: Station 1 is online in its isolated grid, so the maintenance check is resolved by station 1.
/// * Query `[2,1]`: Station 1 goes offline.
/// * Query `[1,1]`: Station 1 is offline and there are no other stations in its grid, so the result is -1.
///
/// **Constraints:**
///
/// * `1 <= c <= 10<sup>5</sup>`
/// * `0 <= n == connections.length <= min(10<sup>5</sup>, c * (c - 1) / 2)`
/// * `connections[i].length == 2`
/// * `1 <= u<sub>i</sub>, v<sub>i</sub> <= c`
/// * `u<sub>i</sub> != v<sub>i</sub>`
/// * `1 <= queries.length <= 2 * 10<sup>5</sup>`
/// * `queries[i].length == 2`
/// * `queries[i][0]` is either 1 or 2.
/// * `1 <= queries[i][1] <= c`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/power-grid-maintenance/
// discuss: https://leetcode.com/problems/power-grid-maintenance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::BTreeSet;

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

        self.data[b] = a;
        self.size[a] += self.size[b];
    }
}

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut uf = UnionFind::new(c + 1);

        for connection in connections {
            uf.union(connection[0] as usize, connection[1] as usize);
        }

        let mut online = vec![BTreeSet::new(); c + 1];

        for x in 1..=c {
            online[uf.find(x)].insert(x);
        }

        let mut result = vec![];

        for query in queries {
            let x = query[1] as usize;
            let root = uf.find(x);

            if query[0] == 1 {
                result.push(match online[root].get(&x).or(online[root].first()) {
                    Some(&x) => x as i32,
                    None => -1,
                });
            }

            if query[0] == 2 {
                online[root].remove(&x);
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
    fn test_3607() {
        let c = 5;
        let connections = nd_vec![[1, 2], [2, 3], [3, 4], [4, 5]];
        let queries = nd_vec![[1, 3], [2, 1], [1, 1], [2, 2], [1, 2]];
        let expected = vec![3, 2, 3];
        assert_eq!(Solution::process_queries(c, connections, queries), expected);
        let c = 3;
        let connections = nd_vec![];
        let queries = nd_vec![[1, 1], [2, 1], [1, 1]];
        let expected = vec![1, -1];
        assert_eq!(Solution::process_queries(c, connections, queries), expected);
    }
}
