///
/// # 391. Perfect Rectangle
///
/// Given an array `rectangles` where `rectangles[i] = [x<sub>i</sub>, y<sub>i</sub>, a<sub>i</sub>, b<sub>i</sub>]` represents an axis-aligned rectangle. The bottom-left point of the rectangle is `(x<sub>i</sub>, y<sub>i</sub>)` and the top-right point of it is `(a<sub>i</sub>, b<sub>i</sub>)`.
///
/// Return `true` *if all the rectangles together form an exact cover of a rectangular region*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/03/27/perectrec1-plane.jpg)
///
/// ```
/// Input: rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
/// Output: true
/// Explanation: All 5 rectangles together form an exact cover of a rectangular region.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/03/27/perfectrec2-plane.jpg)
///
/// ```
/// Input: rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
/// Output: false
/// Explanation: Because there is a gap between the two rectangular regions.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/03/27/perfecrrec4-plane.jpg)
///
/// ```
/// Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
/// Output: false
/// Explanation: Because two of the rectangles overlap with each other.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= rectangles.length <= 2 * 10<sup>4</sup>`
/// * `rectangles[i].length == 4`
/// * `-10<sup>5</sup> <= x<sub>i</sub> < a<sub>i</sub> <= 10<sup>5</sup>`
/// * `-10<sup>5</sup> <= y<sub>i</sub> < b<sub>i</sub> <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-rectangle/
// discuss: https://leetcode.com/problems/perfect-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_rectangle_cover(mut rectangles: Vec<Vec<i32>>) -> bool {
        rectangles.sort_unstable_by_key(|r| r[0]);

        let (x_min, y_min, x_max, y_max) = rectangles.iter().fold(
            (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
            |(x_min, y_min, x_max, y_max), r| {
                (
                    x_min.min(r[0]),
                    y_min.min(r[1]),
                    x_max.max(r[2]),
                    y_max.max(r[3]),
                )
            },
        );

        let mut cur_x = vec![x_min; (y_max - y_min) as usize];

        for r in rectangles {
            let (x1, y1, x2, y2) = (r[0], r[1], r[2], r[3]);

            for y in y1..y2 {
                let y_idx = (y - y_min) as usize;
                if cur_x[y_idx] != x1 {
                    return false;
                }

                cur_x[y_idx] = x2;
            }
        }

        cur_x.iter().all(|&x| x == x_max)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_391() {
        let rectangles = nd_vec![
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [3, 2, 4, 4],
            [1, 3, 2, 4],
            [2, 3, 3, 4]
        ];
        let expected = true;
        assert_eq!(Solution::is_rectangle_cover(rectangles), expected);
        let rectangles = nd_vec![[1, 1, 2, 3], [1, 3, 2, 4], [3, 1, 4, 2], [3, 2, 4, 4]];
        let expected = false;
        assert_eq!(Solution::is_rectangle_cover(rectangles), expected);
        let rectangles = nd_vec![[1, 1, 3, 3], [3, 1, 4, 2], [1, 3, 2, 4], [2, 2, 4, 4]];
        let expected = false;
        assert_eq!(Solution::is_rectangle_cover(rectangles), expected);
    }
}
