///
/// # 3439. Reschedule Meetings for Maximum Free Time I
///
/// You are given an integer `eventTime` denoting the duration of an event, where the event occurs from time `t = 0` to time `t = eventTime`.
///
/// You are also given two integer arrays `startTime` and `endTime`, each of length `n`. These represent the start and end time of `n` **non-overlapping** meetings, where the `i<sup>th</sup>` meeting occurs during the time `[startTime[i], endTime[i]]`.
///
/// You can reschedule **at most** `k` meetings by moving their start time while maintaining the **same duration**, to **maximize** the **longest** *continuous period of free time* during the event.
///
/// The **relative** order of all the meetings should stay the *same* and they should remain non-overlapping.
///
/// Return the **maximum** amount of free time possible after rearranging the meetings.
///
/// **Note** that the meetings can **not** be rescheduled to a time outside the event.
///
/// **Example 1:**
///
/// **Input:** eventTime = 5, k = 1, startTime = [1,3], endTime = [2,5]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/12/21/example0_rescheduled.png)
///
/// Reschedule the meeting at `[1, 2]` to `[2, 3]`, leaving no meetings during the time `[0, 2]`.
///
/// **Example 2:**
///
/// **Input:** eventTime = 10, k = 1, startTime = [0,2,9], endTime = [1,4,10]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/12/21/example1_rescheduled.png)
///
/// Reschedule the meeting at `[2, 4]` to `[1, 3]`, leaving no meetings during the time `[3, 9]`.
///
/// **Example 3:**
///
/// **Input:** eventTime = 5, k = 2, startTime = [0,1,2,3,4], endTime = [1,2,3,4,5]
///
/// **Output:** 0
///
/// **Explanation:**
///
/// There is no time during the event not occupied by meetings.
///
/// **Constraints:**
///
/// * `1 <= eventTime <= 10<sup>9</sup>`
/// * `n == startTime.length == endTime.length`
/// * `2 <= n <= 10<sup>5</sup>`
/// * `1 <= k <= n`
/// * `0 <= startTime[i] < endTime[i] <= eventTime`
/// * `endTime[i] <= startTime[i + 1]` where `i` lies in the range `[0, n - 2]`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-i/
// discuss: https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{collections::VecDeque, iter::once};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let k = k as usize;

        let mut result = 0;
        let mut left_end = 0;
        let mut window = VecDeque::new();
        let mut event_len = 0;

        for (start, end) in once((0, 0))
            .chain(start_time.into_iter().zip(end_time))
            .chain(once((event_time, event_time)))
        {
            result = result.max(start - left_end - event_len);

            event_len += end - start;
            window.push_back((start, end));

            if window.len() > k {
                let (pop_start, pop_end) = window.pop_front().unwrap();
                event_len -= pop_end - pop_start;
                left_end = pop_end;
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
    fn test_3439() {
        let event_time = 5;
        let k = 1;
        let start_time = vec![1, 3];
        let end_time = vec![2, 5];
        let expected = 2;
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            expected
        );
        let event_time = 10;
        let k = 1;
        let start_time = vec![0, 2, 9];
        let end_time = vec![1, 4, 10];
        let expected = 6;
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            expected
        );
        let event_time = 5;
        let k = 2;
        let start_time = vec![0, 1, 2, 3, 4];
        let end_time = vec![1, 2, 3, 4, 5];
        let expected = 0;
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            expected
        );
    }
}
