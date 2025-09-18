///
/// # 3195. Find the Minimum Area to Cover All Ones I
///
/// You are given a 2D **binary** array `grid`. Find a rectangle with horizontal and vertical sides with the **smallest** area, such that all the 1's in `grid` lie inside this rectangle.
///
/// Return the **minimum** possible area of the rectangle.
///
/// **Example 1:**
///
/// **Input:** grid = [[0,1,0],[1,0,1]]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/05/08/examplerect0.png)
///
/// The smallest rectangle has a height of 2 and a width of 3, so it has an area of `2 * 3 = 6`.
///
/// **Example 2:**
///
/// **Input:** grid = [[1,0],[0,0]]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/05/08/examplerect1.png)
///
/// The smallest rectangle has both height and width 1, so its area is `1 * 1 = 1`.
///
/// **Constraints:**
///
/// * `1 <= grid.length, grid[i].length <= 1000`
/// * `grid[i][j]` is either 0 or 1.
/// * The input is generated such that there is at least one 1 in `grid`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-i/
// discuss: https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut l, mut r, mut t, mut b) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);

        for (i, row) in grid.iter().enumerate() {
            for (j, &el) in row.iter().enumerate() {
                if el == 1 {
                    l = l.min(j as i32);
                    r = r.max(j as i32);
                    t = t.min(i as i32);
                    b = b.max(i as i32);
                }
            }
        }

        (r + 1 - l) * (b + 1 - t)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3195() {
        let grid = nd_vec![[0, 1, 0], [1, 0, 1]];
        let expected = 6;
        assert_eq!(Solution::minimum_area(grid), expected);
        let grid = nd_vec![[1, 0], [0, 0]];
        let expected = 1;
        assert_eq!(Solution::minimum_area(grid), expected);
    }
}
