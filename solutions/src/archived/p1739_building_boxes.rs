///
/// # 1739. Building Boxes
///
/// You have a cubic storeroom where the width, length, and height of the room are all equal to `n` units. You are asked to place `n` boxes in this room where each box is a cube of unit side length. There are however some rules to placing the boxes:
///
/// * You can place the boxes anywhere on the floor.
/// * If box `x` is placed on top of the box `y`, then each side of the four vertical sides of the box `y` **must** either be adjacent to another box or to a wall.
///
/// Given an integer `n`, return *the **minimum** possible number of boxes touching the floor.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/04/3-boxes.png)
///
/// ```
/// Input: n = 3
/// Output: 3
/// Explanation: The figure above is for the placement of the three boxes.
/// These boxes are placed in the corner of the room, where the corner is on the left side.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/04/4-boxes.png)
///
/// ```
/// Input: n = 4
/// Output: 3
/// Explanation: The figure above is for the placement of the four boxes.
/// These boxes are placed in the corner of the room, where the corner is on the left side.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/04/10-boxes.png)
///
/// ```
/// Input: n = 10
/// Output: 6
/// Explanation: The figure above is for the placement of the ten boxes.
/// These boxes are placed in the corner of the room, where the corner is on the back side.
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/building-boxes/
// discuss: https://leetcode.com/problems/building-boxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let mut total = 0;
        let mut cur = 0;

        for i in 1.. {
            if n > total + cur + i {
                cur += i;
                total += cur;
                continue;
            }

            for j in 1..=i {
                total += j;

                if total >= n {
                    return cur + j;
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
    fn test_1739() {
        let n = 3;
        let expected = 3;
        assert_eq!(Solution::minimum_boxes(n), expected);
        let n = 4;
        let expected = 3;
        assert_eq!(Solution::minimum_boxes(n), expected);
        let n = 10;
        let expected = 6;
        assert_eq!(Solution::minimum_boxes(n), expected);
        let n = 126;
        let expected = 39;
        assert_eq!(Solution::minimum_boxes(n), expected);
    }
}
