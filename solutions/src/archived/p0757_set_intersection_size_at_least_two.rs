///
/// # 757. Set Intersection Size At Least Two
///
/// You are given a 2D integer array `intervals` where `intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]` represents all the integers from `start<sub>i</sub>` to `end<sub>i</sub>` inclusively.
///
/// A **containing set** is an array `nums` where each interval from `intervals` has **at least two** integers in `nums`.
///
/// * For example, if `intervals = [[1,3], [3,7], [8,9]]`, then `[1,2,4,7,8,9]` and `[2,3,4,8,9]` are **containing sets**.
///
/// Return *the minimum possible size of a containing set*.
///
/// **Example 1:**
///
/// ```
/// Input: intervals = [[1,3],[3,7],[8,9]]
/// Output: 5
/// Explanation: let nums = [2, 3, 4, 8, 9].
/// It can be shown that there cannot be any containing array of size 4.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: intervals = [[1,3],[1,4],[2,5],[3,5]]
/// Output: 3
/// Explanation: let nums = [2, 3, 4].
/// It can be shown that there cannot be any containing array of size 2.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: intervals = [[1,2],[2,3],[2,4],[4,5]]
/// Output: 5
/// Explanation: let nums = [1, 2, 3, 4, 5].
/// It can be shown that there cannot be any containing array of size 4.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= intervals.length <= 3000`
/// * `intervals[i].length == 2`
/// * `0 <= start<sub>i</sub> < end<sub>i</sub> <= 10<sup>8</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/set-intersection-size-at-least-two/
// discuss: https://leetcode.com/problems/set-intersection-size-at-least-two/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::Reverse;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|x| (x[1], Reverse(x[0])));

        let mut result = 0;
        let mut last_two = [-1, -1];

        for interval in &intervals {
            let (start, end) = (interval[0], interval[1]);

            if last_two[0] < start {
                if last_two[1] >= start {
                    result += 1;
                    last_two = [last_two[1], end];
                } else {
                    result += 2;
                    last_two = [end - 1, end];
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
    fn test_757() {
        let intervals = nd_vec![[1, 3], [3, 7], [8, 9]];
        let expected = 5;
        assert_eq!(Solution::intersection_size_two(intervals), expected);
        let intervals = nd_vec![[1, 3], [1, 4], [2, 5], [3, 5]];
        let expected = 3;
        assert_eq!(Solution::intersection_size_two(intervals), expected);
        let intervals = nd_vec![[1, 2], [2, 3], [2, 4], [4, 5]];
        let expected = 5;
        assert_eq!(Solution::intersection_size_two(intervals), expected);
    }
}
