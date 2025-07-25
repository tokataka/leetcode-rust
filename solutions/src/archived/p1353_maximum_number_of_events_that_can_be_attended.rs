///
/// # 1353. Maximum Number of Events That Can Be Attended
///
/// You are given an array of `events` where `events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>]`. Every event `i` starts at `startDay<sub>i</sub>`<sub> </sub>and ends at `endDay<sub>i</sub>`.
///
/// You can attend an event `i` at any day `d` where `startTime<sub>i</sub> <= d <= endTime<sub>i</sub>`. You can only attend one event at any time `d`.
///
/// Return *the maximum number of events you can attend*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/02/05/e1.png)
///
/// ```
/// Input: events = [[1,2],[2,3],[3,4]]
/// Output: 3
/// Explanation: You can attend all the three events.
/// One way to attend them all is as shown.
/// Attend the first event on day 1.
/// Attend the second event on day 2.
/// Attend the third event on day 3.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: events= [[1,2],[2,3],[3,4],[1,2]]
/// Output: 4
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= events.length <= 10<sup>5</sup>`
/// * `events[i].length == 2`
/// * `1 <= startDay<sub>i</sub> <= endDay<sub>i</sub> <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/
// discuss: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable();

        let mut result = 0;
        let mut cur = 1;
        let mut i = 0;

        let mut pq = BinaryHeap::new();

        while i < events.len() || !pq.is_empty() {
            if pq.is_empty() && cur > events[i][0] {
                cur = events[i][0];
            }

            while i < events.len() && events[i][0] <= cur {
                pq.push(Reverse(events[i][1]));
                i += 1;
            }

            while let Some(Reverse(t)) = pq.pop() {
                if t >= cur {
                    result += 1;
                    break;
                }
            }

            cur += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1353() {
        // let events = nd_vec![[1, 2], [2, 3], [3, 4]];
        // let expected = 3;
        // assert_eq!(Solution::max_events(events), expected);
        // let events = nd_vec![[1, 2], [2, 3], [3, 4], [1, 2]];
        // let expected = 4;
        // assert_eq!(Solution::max_events(events), expected);
        // let events = nd_vec![[1, 10], [2, 2], [2, 2], [2, 2], [2, 2]];
        // let expected = 2;
        // assert_eq!(Solution::max_events(events), expected);
        let events = nd_vec![[1, 2], [1, 2], [3, 3], [1, 5], [1, 5]];
        let expected = 5;
        assert_eq!(Solution::max_events(events), expected);
    }
}
