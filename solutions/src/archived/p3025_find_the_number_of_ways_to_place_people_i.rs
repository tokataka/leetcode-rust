///
/// # 3025. Find the Number of Ways to Place People I
///
/// You are given a 2D array `points` of size `n x 2` representing integer coordinates of some points on a 2D plane, where `points[i] = [x<sub>i</sub>, y<sub>i</sub>]`.
///
/// Count the number of pairs of points `(A, B)`, where
///
/// * `A` is on the **upper left** side of `B`, and
/// * there are no other points in the rectangle (or line) they make (**including the border**).
///
/// Return the count.
///
/// **Example 1:**
///
/// **Input:** points = [[1,1],[2,2],[3,3]]
///
/// **Output:** 0
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/01/04/example1alicebob.png)
///
/// There is no way to choose `A` and `B` so `A` is on the upper left side of `B`.
///
/// **Example 2:**
///
/// **Input:** points = [[6,2],[4,4],[2,6]]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// <img height="365" src="https://assets.leetcode.com/uploads/2024/06/25/t2.jpg" width="1321">
///
/// * The left one is the pair `(points[1], points[0])`, where `points[1]` is on the upper left side of `points[0]` and the rectangle is empty.
/// * The middle one is the pair `(points[2], points[1])`, same as the left one it is a valid pair.
/// * The right one is the pair `(points[2], points[0])`, where `points[2]` is on the upper left side of `points[0]`, but `points[1]` is inside the rectangle so it's not a valid pair.
///
/// **Example 3:**
///
/// **Input:** points = [[3,1],[1,3],[1,1]]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/06/25/t3.jpg)
///
/// * The left one is the pair `(points[2], points[0])`, where `points[2]` is on the upper left side of `points[0]` and there are no other points on the line they form. Note that it is a valid state when the two points form a line.
/// * The middle one is the pair `(points[1], points[2])`, it is a valid pair same as the left one.
/// * The right one is the pair `(points[1], points[0])`, it is not a valid pair as `points[2]` is on the border of the rectangle.
///
/// **Constraints:**
///
/// * `2 <= n <= 50`
/// * `points[i].length == 2`
/// * `0 <= points[i][0], points[i][1] <= 50`
/// * All `points[i]` are distinct.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-number-of-ways-to-place-people-i/
// discuss: https://leetcode.com/problems/find-the-number-of-ways-to-place-people-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::Reverse;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        points.sort_unstable_by_key(|p| (p[0], Reverse(p[1])));

        for (i, point1) in points.iter().enumerate() {
            let height1 = point1[1];

            let mut max_height2 = -1;

            for point2 in points.iter().skip(i + 1) {
                let height2 = point2[1];

                if height2 > height1 {
                    continue;
                }

                if height2 > max_height2 {
                    max_height2 = height2;
                    result += 1;
                }
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
    fn test_3025() {
        let points = nd_vec![[1, 1], [2, 2], [3, 3]];
        let expected = 0;
        assert_eq!(Solution::number_of_pairs(points), expected);
        let points = nd_vec![[6, 2], [4, 4], [2, 6]];
        let expected = 2;
        assert_eq!(Solution::number_of_pairs(points), expected);
        let points = nd_vec![[3, 1], [1, 3], [1, 1]];
        let expected = 2;
        assert_eq!(Solution::number_of_pairs(points), expected);
    }
}
