use std::collections::{BTreeMap, HashMap};

///
/// # 3454. Separate Squares II
///
/// You are given a 2D integer array `squares`. Each `squares[i] = [x<sub>i</sub>, y<sub>i</sub>, l<sub>i</sub>]` represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
///
/// Find the **minimum** y-coordinate value of a horizontal line such that the total area covered by squares above the line *equals* the total area covered by squares below the line.
///
/// Answers within `10<sup>-5</sup>` of the actual answer will be accepted.
///
/// **Note**: Squares **may** overlap. Overlapping areas should be counted **only once** in this version.
///
/// **Example 1:**
///
/// **Input:** squares = [[0,0,1],[2,2,1]]
///
/// **Output:** 1.00000
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/01/15/4065example1drawio.png)
///
/// Any horizontal line between `y = 1` and `y = 2` results in an equal split, with 1 square unit above and 1 square unit below. The minimum y-value is 1.
///
/// **Example 2:**
///
/// **Input:** squares = [[0,0,2],[1,1,1]]
///
/// **Output:** 1.00000
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2025/01/15/4065example2drawio.png)
///
/// Since the blue square overlaps with the red square, it will not be counted again. Thus, the line `y = 1` splits the squares into two equal parts.
///
/// **Constraints:**
///
/// * `1 <= squares.length <= 5 * 10<sup>4</sup>`
/// * `squares[i] = [x<sub>i</sub>, y<sub>i</sub>, l<sub>i</sub>]`
/// * `squares[i].length == 3`
/// * `0 <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>9</sup>`
/// * `1 <= l<sub>i</sub> <= 10<sup>9</sup>`
/// * The total area of all the squares will not exceed `10<sup>15</sup>`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/separate-squares-ii/
// discuss: https://leetcode.com/problems/separate-squares-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct SegmentTree {
    data: Vec<(i32, i32)>,
    idx_to_x: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize, idx_to_x: Vec<i32>) -> Self {
        Self {
            data: vec![(0, 0); 4 * n],
            idx_to_x,
        }
    }

    fn query(&self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i32 {
        if left > end || right < start {
            return 0;
        }

        if left <= start && end <= right {
            return self.data[node].0;
        }

        let left_sum = self.query(node * 2, start, (start + end) / 2, left, right);
        let right_sum = self.query(node * 2 + 1, (start + end) / 2 + 1, end, left, right);

        left_sum + right_sum
    }

    fn update(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
        diff: i32,
    ) {
        if left > end || right < start {
            return;
        }

        if left <= start && end <= right {
            let next_count = self.data[node].1 + diff;

            if next_count > 0 {
                self.data[node] = (self.idx_to_x[end + 1] - self.idx_to_x[start], next_count);
            } else {
                self.data[node] = (0, 0);
            }
            return;

        }

        self.update(node * 2, start, (start + end) / 2, left, right, diff);
        self.update(node * 2 + 1, (start + end) / 2 + 1, end, left, right, diff);

        let width = self.data[node * 2].0 + self.data[node * 2 + 1].0;
        self.data[node] = (width, 0);
    }
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut idx_to_x = squares
            .iter()
            .flat_map(|square| [square[0], square[0] + square[2]])
            .collect::<Vec<_>>();

        idx_to_x.sort();
        idx_to_x.dedup();

        let n = idx_to_x.len() - 1;

        let x_to_idx = idx_to_x
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect::<HashMap<_, _>>();

        let lines_map: BTreeMap<_, Vec<_>> = squares
            .iter()
            .flat_map(|square| {
                let (x, y, l) = (square[0], square[1], square[2]);
                [
                    (y, (x_to_idx[&x], x_to_idx[&(x + l)]), 1),
                    (y + l, (x_to_idx[&x], x_to_idx[&(x + l)]), -1),
                ]
            })
            .fold(BTreeMap::new(), |mut acc, line| {
                acc.entry(line.0).or_default().push((line.1, line.2));
                acc
            });

        let mut tree = SegmentTree::new(n, idx_to_x);

        let mut prefix_sum = vec![];
        let mut lower_area = 0;
        let mut prev_y = 0;

        for (y, lines) in lines_map {
            let prev_width = tree.query(1, 0, n - 1, 0, n - 1);
            lower_area += prev_width as i64 * (y - prev_y) as i64;
            prefix_sum.push((lower_area, y, prev_width));
            prev_y = y;

            for ((left, right), diff) in lines {
                tree.update(1, 0, n - 1, left, right - 1, diff);
            }
        }

        let total_area = lower_area;

        let center = prefix_sum.partition_point(|&x| x.0 < (total_area + 1) / 2);

        if prefix_sum[center].0 * 2 == total_area {
            return prefix_sum[center].1 as f64;
        }

        let exceed_area = prefix_sum[center].0 as f64 - total_area as f64 / 2.;
        let exceed_height = exceed_area / prefix_sum[center].2 as f64;

        prefix_sum[center].1 as f64 - exceed_height
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3454() {
        // let squares = nd_vec![[0, 0, 1], [2, 2, 1]];
        // let expected = 1.00000;
        // assert_eq!(Solution::separate_squares(squares), expected);
        // let squares = nd_vec![[0, 0, 2], [1, 1, 1]];
        // let expected = 1.00000;
        // assert_eq!(Solution::separate_squares(squares), expected);
        let squares = nd_vec![[1,1,50000],[2,1,50000],[3,1,50000],[4,1,50000],[5,1,50000],[6,1,50000],[7,1,50000],[8,1,50000],[9,1,50000],[10,1,50000]];
        let expected = 25001.;
        assert_eq!(Solution::separate_squares(squares), expected);
    }
}
