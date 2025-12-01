///
/// # 1840. Maximum Building Height
///
/// You want to build `n` new buildings in a city. The new buildings will be built in a line and are labeled from `1` to `n`.
///
/// However, there are city restrictions on the heights of the new buildings:
///
/// * The height of each building must be a non-negative integer.
/// * The height of the first building **must** be `0`.
/// * The height difference between any two adjacent buildings **cannot exceed** `1`.
///
/// Additionally, there are city restrictions on the maximum height of specific buildings. These restrictions are given as a 2D integer array `restrictions` where `restrictions[i] = [id<sub>i</sub>, maxHeight<sub>i</sub>]` indicates that building `id<sub>i</sub>` must have a height **less than or equal to** `maxHeight<sub>i</sub>`.
///
/// It is guaranteed that each building will appear **at most once** in `restrictions`, and building `1` will **not** be in `restrictions`.
///
/// Return *the **maximum possible height** of the **tallest** building*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex1-1.png)
///
/// ```
/// Input: n = 5, restrictions = [[2,1],[4,1]]
/// Output: 2
/// Explanation: The green area in the image indicates the maximum allowed height for each building.
/// We can build the buildings with heights [0,1,2,1,2], and the tallest building has a height of 2.
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex2.png)
///
/// ```
/// Input: n = 6, restrictions = []
/// Output: 5
/// Explanation: The green area in the image indicates the maximum allowed height for each building.
/// We can build the buildings with heights [0,1,2,3,4,5], and the tallest building has a height of 5.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex3.png)
///
/// ```
/// Input: n = 10, restrictions = [[5,3],[2,5],[7,4],[10,3]]
/// Output: 5
/// Explanation: The green area in the image indicates the maximum allowed height for each building.
/// We can build the buildings with heights [0,1,2,3,3,4,4,5,4,3], and the tallest building has a height of 5.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 10<sup>9</sup>`
/// * `0 <= restrictions.length <= min(n - 1, 10<sup>5</sup>)`
/// * `2 <= id<sub>i</sub> <= n`
/// * `id<sub>i</sub>` is **unique**.
/// * `0 <= maxHeight<sub>i</sub> <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-building-height/
// discuss: https://leetcode.com/problems/maximum-building-height/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut r = restrictions;
        r.push(vec![1, 0]);
        r.sort_unstable();

        for i in 0..r.len() - 1 {
            r[i + 1][1] = r[i + 1][1].min(r[i][1] + r[i + 1][0] - r[i][0]);
        }

        let mut result = r.last().map(|r| r[1] + n - r[0]).unwrap();

        for i in (0..r.len() - 1).rev() {
            r[i][1] = r[i][1].min(r[i + 1][1] + r[i + 1][0] - r[i][0]);

            result = result.max((r[i][1] + r[i + 1][1] + r[i + 1][0] - r[i][0]) / 2);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1840() {
        let n = 5;
        let restrictions = nd_vec![[2, 1], [4, 1]];
        let expected = 2;
        assert_eq!(Solution::max_building(n, restrictions), expected);
        let n = 6;
        let restrictions = nd_vec![];
        let expected = 5;
        assert_eq!(Solution::max_building(n, restrictions), expected);
        let n = 10;
        let restrictions = nd_vec![[5, 3], [2, 5], [7, 4], [10, 3]];
        let expected = 5;
        assert_eq!(Solution::max_building(n, restrictions), expected);
        let n = 10;
        let restrictions = nd_vec![
            [6, 0],
            [5, 2],
            [7, 0],
            [9, 1],
            [2, 4],
            [3, 4],
            [4, 0],
            [8, 2],
            [10, 0]
        ];
        let expected = 1;
        assert_eq!(Solution::max_building(n, restrictions), expected);
    }
}
