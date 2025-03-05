use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 3342. Find Minimum Time to Reach Last Room II
///
/// There is a dungeon with `n x m` rooms arranged as a grid.
///
/// You are given a 2D array `moveTime` of size `n x m`, where `moveTime[i][j]` represents the **minimum** time in seconds when you can **start moving** to that room. You start from the room `(0, 0)` at time `t = 0` and can move to an **adjacent** room. Moving between **adjacent** rooms takes one second for one move and two seconds for the next, **alternating** between the two.
///
/// Return the **minimum** time to reach the room `(n - 1, m - 1)`.
///
/// Two rooms are **adjacent** if they share a common wall, either *horizontally* or *vertically*.
///
/// **Example 1:**
///
/// **Input:** moveTime = [[0,4],[4,4]]
///
/// **Output:** 7
///
/// **Explanation:**
///
/// The minimum time required is 7 seconds.
///
/// * At time `t == 4`, move from room `(0, 0)` to room `(1, 0)` in one second.
/// * At time `t == 5`, move from room `(1, 0)` to room `(1, 1)` in two seconds.
///
/// **Example 2:**
///
/// **Input:** moveTime = [[0,0,0,0],[0,0,0,0]]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// The minimum time required is 6 seconds.
///
/// * At time `t == 0`, move from room `(0, 0)` to room `(1, 0)` in one second.
/// * At time `t == 1`, move from room `(1, 0)` to room `(1, 1)` in two seconds.
/// * At time `t == 3`, move from room `(1, 1)` to room `(1, 2)` in one second.
/// * At time `t == 4`, move from room `(1, 2)` to room `(1, 3)` in two seconds.
///
/// **Example 3:**
///
/// **Input:** moveTime = [[0,1],[1,2]]
///
/// **Output:** 4
///
/// **Constraints:**
///
/// * `2 <= n == moveTime.length <= 750`
/// * `2 <= m == moveTime[i].length <= 750`
/// * `0 <= moveTime[i][j] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-minimum-time-to-reach-last-room-ii/
// discuss: https://leetcode.com/problems/find-minimum-time-to-reach-last-room-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let height = move_time.len();
        let width = move_time[0].len();

        const DIRECTION: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let mut cost_map = vec![vec![-1; width]; height];
        cost_map[0][0] = 0;

        let mut pq = BinaryHeap::from([(Reverse(0), (0, 0))]);

        while let Some((Reverse(cost), (i, j))) = pq.pop() {
            if cost > cost_map[i][j] {
                continue;
            }

            if i == height - 1 && j == width - 1 {
                return cost;
            }

            for (di, dj) in DIRECTION {
                let (ci, cj) = match (i.checked_add_signed(di), j.checked_add_signed(dj)) {
                    (Some(ci), Some(cj)) if ci < height && cj < width => (ci, cj),
                    _ => continue,
                };

                let next_cost = cost.max(move_time[ci][cj]) + if (i + j) % 2 == 0 { 1 } else { 2 };

                if cost_map[ci][cj] == -1 || next_cost < cost_map[ci][cj] {
                    cost_map[ci][cj] = next_cost;
                    pq.push((Reverse(next_cost), (ci, cj)));
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
    fn test_3342() {
        let move_time = nd_vec![[0, 4], [4, 4]];
        let expected = 7;
        assert_eq!(Solution::min_time_to_reach(move_time), expected);
        let move_time = nd_vec![[0, 0, 0, 0], [0, 0, 0, 0]];
        let expected = 6;
        assert_eq!(Solution::min_time_to_reach(move_time), expected);
        let move_time = nd_vec![[0, 1], [1, 2]];
        let expected = 4;
        assert_eq!(Solution::min_time_to_reach(move_time), expected);
    }
}
