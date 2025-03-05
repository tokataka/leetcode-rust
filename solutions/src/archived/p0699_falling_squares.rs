use std::collections::HashMap;

///
/// # 699. Falling Squares
///
/// There are several squares being dropped onto the X-axis of a 2D plane.
///
/// You are given a 2D integer array `positions` where `positions[i] = [left<sub>i</sub>, sideLength<sub>i</sub>]` represents the `i<sup>th</sup>` square with a side length of `sideLength<sub>i</sub>` that is dropped with its left edge aligned with X-coordinate `left<sub>i</sub>`.
///
/// Each square is dropped one at a time from a height above any landed squares. It then falls downward (negative Y direction) until it either lands **on the top side of another square** or **on the X-axis**. A square brushing the left/right side of another square does not count as landing on it. Once it lands, it freezes in place and cannot be moved.
///
/// After each square is dropped, you must record the **height of the current tallest stack of squares**.
///
/// Return *an integer array* `ans` *where* `ans[i]` *represents the height described above after dropping the* `i<sup>th</sup>` *square*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/28/fallingsq1-plane.jpg)
///
/// ```
/// Input: positions = [[1,2],[2,3],[6,1]]
/// Output: [2,5,5]
/// Explanation:
/// After the first drop, the tallest stack is square 1 with a height of 2.
/// After the second drop, the tallest stack is squares 1 and 2 with a height of 5.
/// After the third drop, the tallest stack is still squares 1 and 2 with a height of 5.
/// Thus, we return an answer of [2, 5, 5].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: positions = [[100,100],[200,100]]
/// Output: [100,100]
/// Explanation:
/// After the first drop, the tallest stack is square 1 with a height of 100.
/// After the second drop, the tallest stack is either square 1 or square 2, both with heights of 100.
/// Thus, we return an answer of [100, 100].
/// Note that square 2 only brushes the right side of square 1, which does not count as landing on it.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= positions.length <= 1000`
/// * `1 <= left<sub>i</sub> <= 10<sup>8</sup>`
/// * `1 <= sideLength<sub>i</sub> <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/falling-squares/
// discuss: https://leetcode.com/problems/falling-squares/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct SegmentTree {
    data: Vec<i32>,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    fn query(&self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i32 {
        if left > end || right < start {
            return -1;
        }

        if left <= start && end <= right {
            return self.data[node];
        }

        let left_max = self.query(node * 2, start, (start + end) / 2, left, right);
        let right_max = self.query(node * 2 + 1, (start + end) / 2 + 1, end, left, right);

        left_max.max(right_max)
    }

    fn update(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
        height: i32,
    ) {
        if left > end || right < start {
            return;
        }

        if start == end {
            self.data[node] = height;
            return;
        }

        self.update(node * 2, start, (start + end) / 2, left, right, height);
        self.update(
            node * 2 + 1,
            (start + end) / 2 + 1,
            end,
            left,
            right,
            height,
        );

        self.data[node] = self.data[node * 2].max(self.data[node * 2 + 1]);
    }
}


impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut idx_to_x = positions
            .iter()
            .flat_map(|x| [x[0], x[0] + x[1] - 1])
            .collect::<Vec<_>>();

        idx_to_x.sort();
        idx_to_x.dedup();

        let x_to_idx = idx_to_x
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect::<HashMap<_, _>>();

        let n = idx_to_x.len();

        let mut tree = SegmentTree::new(4 * n);
        let mut result = vec![];

        for position in positions {
            let (x, side_length) = (position[0], position[1]);
            let (left, right) = (x_to_idx[&x], x_to_idx[&(x + side_length - 1)]);

            let max_height = tree.query(1, 0, n - 1, left, right);

            tree.update(1, 0, n - 1, left, right, side_length + max_height);

            result.push(tree.query(1, 0, n - 1, 0, n - 1));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_699() {
        let positions = nd_vec![[1, 2], [2, 3], [6, 1]];
        let expected = vec![2, 5, 5];
        assert_eq!(Solution::falling_squares(positions), expected);
        let positions = nd_vec![[100, 100], [200, 100]];
        let expected = vec![100, 100];
        assert_eq!(Solution::falling_squares(positions), expected);
    }
}
