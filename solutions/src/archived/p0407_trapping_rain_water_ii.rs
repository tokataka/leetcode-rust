use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 407. Trapping Rain Water II
///
/// Given an `m x n` integer matrix `heightMap` representing the i_size of each unit cell in a 2D elevation map, return *the volume of water it can trap after raining*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/08/trap1-3d.jpg)
///
/// ```
/// Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
/// Output: 4
/// Explanation: After the rain, water is trapped between the blocks.
/// We have two small ponds 1 and 3 units trapped.
/// The total volume of water trapped is 4.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/08/trap2-3d.jpg)
///
/// ```
/// Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
/// Output: 10
///
/// ```
///
/// **Constraints:**
///
/// * `m == heightMap.length`
/// * `n == heightMap[i].length`
/// * `1 <= m, n <= 200`
/// * `0 <= heightMap[i][j] <= 2 * 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water-ii/
// discuss: https://leetcode.com/problems/trapping-rain-water-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        #[derive(Clone, Copy, PartialEq)]
        enum Tile {
            Block,
            Water,
            Empty,
        }

        use Tile::*;

        let i_size = height_map.len();
        let j_size = height_map[0].len();

        let mut pq = BinaryHeap::from_iter(height_map.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &height)| (Reverse(height), (i + 1, j + 1)))
        }));

        const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let mut tiles = vec![vec![Block; j_size + 2]; i_size + 2];

        tiles.iter_mut().for_each(|row| {
            row[0] = Empty;
            row[j_size + 1] = Empty;
        });

        tiles[0].iter_mut().for_each(|tile| *tile = Empty);
        tiles[i_size + 1].iter_mut().for_each(|tile| *tile = Empty);

        let mut last_height = 0;
        let mut water_count = 0;

        let mut result = 0;

        while let Some((Reverse(height), (i, j))) = pq.pop() {
            result += water_count * (height - last_height);
            last_height = height;

            if DIRECTIONS.iter().any(|&(di, dj)| {
                let (ci, cj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
                tiles[ci][cj] == Empty
            }) {
                tiles[i][j] = Empty;

                let mut st = vec![(i, j)];

                while let Some((i, j)) = st.pop() {
                    DIRECTIONS.iter().for_each(|&(di, dj)| {
                        let (ci, cj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));

                        if tiles[ci][cj] == Water {
                            tiles[ci][cj] = Empty;
                            water_count -= 1;
                            st.push((ci, cj));
                        }
                    });
                }
            } else {
                tiles[i][j] = Water;
                water_count += 1;
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
    fn test_407() {
        let height_map = nd_vec![[1, 4, 3, 1, 3, 2], [3, 2, 1, 3, 2, 4], [2, 3, 3, 2, 3, 1]];
        let expected = 4;
        assert_eq!(Solution::trap_rain_water(height_map), expected);
        let height_map = nd_vec![
            [3, 3, 3, 3, 3],
            [3, 2, 2, 2, 3],
            [3, 2, 1, 2, 3],
            [3, 2, 2, 2, 3],
            [3, 3, 3, 3, 3]
        ];
        let expected = 10;
        assert_eq!(Solution::trap_rain_water(height_map), expected);
    }
}
