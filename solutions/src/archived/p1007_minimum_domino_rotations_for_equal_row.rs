///
/// # 1007. Minimum Domino Rotations For Equal Row
///
/// In a row of dominoes, `tops[i]` and `bottoms[i]` represent the top and bottom halves of the `i<sup>th</sup>` domino. (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)
///
/// We may rotate the `i<sup>th</sup>` domino, so that `tops[i]` and `bottoms[i]` swap values.
///
/// Return the minimum number of rotations so that all the values in `tops` are the same, or all the values in `bottoms` are the same.
///
/// If it cannot be done, return `-1`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/05/14/domino.png)
///
/// ```
/// Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
/// Output: 2
/// Explanation:
/// The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
/// If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
/// Output: -1
/// Explanation:
/// In this case, it is not possible to rotate the dominoes to make one row of values equal.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= tops.length <= 2 * 10<sup>4</sup>`
/// * `bottoms.length == tops.length`
/// * `1 <= tops[i], bottoms[i] <= 6`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/
// discuss: https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut top_count = [0; 6];
        let mut bottom_count = [0; 6];
        let mut remain = [true; 6];

        for (top, bottom) in tops.into_iter().zip(bottoms) {
            let top = top as usize - 1;
            let bottom = bottom as usize - 1;

            (0..6).filter(|&i| i != top && i != bottom).for_each(|i| {
                remain[i] = false;
            });

            if remain.iter().all(|&x| !x) {
                return -1;
            }

            if top != bottom {
                top_count[bottom] += 1;
                bottom_count[top] += 1;
            }
        }

        remain
            .into_iter()
            .enumerate()
            .filter(|&(_, x)| x)
            .map(|(i, _)| top_count[i].min(bottom_count[i]))
            .min()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1007() {
        let tops = vec![2, 1, 2, 4, 2, 2];
        let bottoms = vec![5, 2, 6, 2, 3, 2];
        let expected = 2;
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), expected);
        let tops = vec![3, 5, 1, 2, 3];
        let bottoms = vec![3, 6, 3, 3, 4];
        let expected = -1;
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), expected);
    }
}
