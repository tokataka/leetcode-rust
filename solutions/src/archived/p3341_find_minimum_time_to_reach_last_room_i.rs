use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 3341. Find Minimum Time to Reach Last Room I
///
/// There is a dungeon with `n x m` rooms arranged as a grid.
///
/// You are given a 2D array `moveTime` of size `n x m`, where `moveTime[i][j]` represents the **minimum** time in seconds when you can **start moving** to that room. You start from the room `(0, 0)` at time `t = 0` and can move to an **adjacent** room. Moving between adjacent rooms takes *exactly* one second.
///
/// Return the **minimum** time to reach the room `(n - 1, m - 1)`.
///
/// Two rooms are **adjacent** if they share a common wall, either *horizontally* or *vertically*.
///
/// **Example 1:**
///
/// **Input:** moveTime = [[0,4],[4,4]]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// The minimum time required is 6 seconds.
///
/// * At time `t == 4`, move from room `(0, 0)` to room `(1, 0)` in one second.
/// * At time `t == 5`, move from room `(1, 0)` to room `(1, 1)` in one second.
///
/// **Example 2:**
///
/// **Input:** moveTime = [[0,0,0],[0,0,0]]
///
/// **Output:** 3
///
/// **Explanation:**
///
/// The minimum time required is 3 seconds.
///
/// * At time `t == 0`, move from room `(0, 0)` to room `(1, 0)` in one second.
/// * At time `t == 1`, move from room `(1, 0)` to room `(1, 1)` in one second.
/// * At time `t == 2`, move from room `(1, 1)` to room `(1, 2)` in one second.
///
/// **Example 3:**
///
/// **Input:** moveTime = [[0,1],[1,2]]
///
/// **Output:** 3
///
/// **Constraints:**
///
/// * `2 <= n == moveTime.length <= 50`
/// * `2 <= m == moveTime[i].length <= 50`
/// * `0 <= moveTime[i][j] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i/
// discuss: https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();

        let mut time = vec![vec![i32::MAX; m]; n];
        let mut pq = BinaryHeap::from([(Reverse(0), (0, 0))]);

        while let Some((Reverse(t), (i, j))) = pq.pop() {
            if t >= time[i][j] {
                continue;
            }

            if i == n - 1 && j == m - 1 {
                return t;
            }

            time[i][j] = t;

            for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (ci, cj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(ci), Some(cj)) if ci < n && cj < m => (ci, cj),
                    _ => continue,
                };

                let next_t = t.max(move_time[ci][cj]) + 1;

                if next_t < time[ci][cj] {
                    pq.push((Reverse(next_t), (ci, cj)));
                }
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
    fn test_3341() {
        let move_time = nd_vec![[0, 4], [4, 4]];
        let expected = 6;
        assert_eq!(Solution::min_time_to_reach(move_time), expected);
        let move_time = nd_vec![[0, 0, 0], [0, 0, 0]];
        let expected = 3;
        assert_eq!(Solution::min_time_to_reach(move_time), expected);
        let move_time = nd_vec![[0, 1], [1, 2]];
        let expected = 3;
        assert_eq!(Solution::min_time_to_reach(move_time), expected);
    }
}
