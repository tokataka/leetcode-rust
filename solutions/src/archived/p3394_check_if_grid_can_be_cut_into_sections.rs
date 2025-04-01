///
/// # 3394. Check if Grid can be Cut into Sections
///
/// You are given an integer `n` representing the dimensions of an `n x n` grid, with the origin at the bottom-left corner of the grid. You are also given a 2D array of coordinates `rectangles`, where `rectangles[i]` is in the form `[start<sub>x</sub>, start<sub>y</sub>, end<sub>x</sub>, end<sub>y</sub>]`, representing a rectangle on the grid. Each rectangle is defined as follows:
///
/// * `(start<sub>x</sub>, start<sub>y</sub>)`: The bottom-left corner of the rectangle.
/// * `(end<sub>x</sub>, end<sub>y</sub>)`: The top-right corner of the rectangle.
///
/// **Note** that the rectangles do not overlap. Your task is to determine if it is possible to make **either two horizontal or two vertical cuts** on the grid such that:
///
/// * Each of the three resulting sections formed by the cuts contains **at least** one rectangle.
/// * Every rectangle belongs to **exactly** one section.
///
/// Return `true` if such cuts can be made; otherwise, return `false`.
///
/// **Example 1:**
///
/// **Input:** n = 5, rectangles = [[1,0,5,2],[0,2,2,4],[3,2,5,3],[0,4,4,5]]
///
/// **Output:** true
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/10/23/tt1drawio.png)
///
/// The grid is shown in the diagram. We can make horizontal cuts at `y = 2` and `y = 4`. Hence, output is true.
///
/// **Example 2:**
///
/// **Input:** n = 4, rectangles = [[0,0,1,1],[2,0,3,4],[0,2,2,3],[3,0,4,3]]
///
/// **Output:** true
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/10/23/tc2drawio.png)
///
/// We can make vertical cuts at `x = 2` and `x = 3`. Hence, output is true.
///
/// **Example 3:**
///
/// **Input:** n = 4, rectangles = [[0,2,2,4],[1,0,3,2],[2,2,3,4],[3,0,4,2],[3,2,4,4]]
///
/// **Output:** false
///
/// **Explanation:**
///
/// We cannot make two horizontal or two vertical cuts that satisfy the conditions. Hence, output is false.
///
/// **Constraints:**
///
/// * `3 <= n <= 10<sup>9</sup>`
/// * `3 <= rectangles.length <= 10<sup>5</sup>`
/// * `0 <= rectangles[i][0] < rectangles[i][2] <= n`
/// * `0 <= rectangles[i][1] < rectangles[i][3] <= n`
/// * No two rectangles overlap.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/
// discuss: https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut axis_x = vec![];
        let mut axis_y = vec![];

        for rectangle in rectangles {
            axis_x.push((rectangle[0], rectangle[2]));
            axis_y.push((rectangle[1], rectangle[3]));
        }

        for mut axis in [axis_x, axis_y] {
            axis.sort_unstable();

            let mut cut_count = -1;
            let mut prev_end = 0;

            for (start, end) in axis {
                if start >= prev_end {
                    cut_count += 1;

                    if cut_count >= 2 {
                        return true;
                    }
                }

                prev_end = prev_end.max(end);
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3394() {
        let n = 5;
        let rectangles = nd_vec![[1, 0, 5, 2], [0, 2, 2, 4], [3, 2, 5, 3], [0, 4, 4, 5]];
        let expected = true;
        assert_eq!(Solution::check_valid_cuts(n, rectangles), expected);
        let n = 4;
        let rectangles = nd_vec![[0, 0, 1, 1], [2, 0, 3, 4], [0, 2, 2, 3], [3, 0, 4, 3]];
        let expected = true;
        assert_eq!(Solution::check_valid_cuts(n, rectangles), expected);
        let n = 4;
        let rectangles = nd_vec![
            [0, 2, 2, 4],
            [1, 0, 3, 2],
            [2, 2, 3, 4],
            [3, 0, 4, 2],
            [3, 2, 4, 4]
        ];
        let expected = false;
        assert_eq!(Solution::check_valid_cuts(n, rectangles), expected);
    }
}
