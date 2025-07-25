///
/// # 3440. Reschedule Meetings for Maximum Free Time II
///
/// You are given an integer `eventTime` denoting the duration of an event. You are also given two integer arrays `startTime` and `endTime`, each of length `n`.
///
/// These represent the start and end times of `n` **non-overlapping** meetings that occur during the event between time `t = 0` and time `t = eventTime`, where the `i<sup>th</sup>` meeting occurs during the time `[startTime[i], endTime[i]].`
///
/// You can reschedule **at most** one meeting by moving its start time while maintaining the **same duration**, such that the meetings remain non-overlapping, to **maximize** the **longest** *continuous period of free time* during the event.
///
/// Return the **maximum** amount of free time possible after rearranging the meetings.
///
/// **Note** that the meetings can **not** be rescheduled to a time outside the event and they should remain non-overlapping.
///
/// **Note:** *In this version*, it is **valid** for the relative ordering of the meetings to change after rescheduling one meeting.
///
/// **Example 1:**
///
/// **Input:** eventTime = 5, startTime = [1,3], endTime = [2,5]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/12/22/example0_rescheduled.png)
///
/// Reschedule the meeting at `[1, 2]` to `[2, 3]`, leaving no meetings during the time `[0, 2]`.
///
/// **Example 2:**
///
/// **Input:** eventTime = 10, startTime = [0,7,9], endTime = [1,8,10]
///
/// **Output:** 7
///
/// **Explanation:**
///
/// ![](https://assets.leetcode.com/uploads/2024/12/22/rescheduled_example0.png)
///
/// Reschedule the meeting at `[0, 1]` to `[8, 9]`, leaving no meetings during the time `[0, 7]`.
///
/// **Example 3:**
///
/// **Input:** eventTime = 10, startTime = [0,3,7,9], endTime = [1,4,8,10]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// **![](https://assets.leetcode.com/uploads/2025/01/28/image3.png)**
///
/// Reschedule the meeting at `[3, 4]` to `[8, 9]`, leaving no meetings during the time `[1, 7]`.
///
/// **Example 4:**
///
/// **Input:** eventTime = 5, startTime = [0,1,2,3,4], endTime = [1,2,3,4,5]
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
/// * `0 <= startTime[i] < endTime[i] <= eventTime`
/// * `endTime[i] <= startTime[i + 1]` where `i` lies in the range `[0, n - 2]`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii/
// discuss: https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut result = 0;

        let mut free_left = start_time[0];
        let mut max_free = 0;

        for i in 0..n {
            let event = end_time[i] - start_time[i];
            let free_right = *start_time.get(i + 1).unwrap_or(&event_time) - end_time[i];

            let mut cur = free_left + free_right;

            if max_free >= event {
                cur += event;
            }

            result = result.max(cur);

            max_free = max_free.max(free_left);
            free_left = free_right;
        }

        let mut free_right = event_time - *end_time.last().unwrap();
        let mut max_free = 0;

        for i in (0..n).rev() {
            let event = end_time[i] - start_time[i];
            let free_left = start_time[i] - if i > 0 { end_time[i - 1] } else { 0 };

            let mut cur = free_left + free_right;

            if max_free >= event {
                cur += event;
            }

            result = result.max(cur);

            max_free = max_free.max(free_right);
            free_right = free_left;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3440() {
        let event_time = 5;
        let start_time = vec![1, 3];
        let end_time = vec![2, 5];
        let expected = 2;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            expected
        );
        let event_time = 10;
        let start_time = vec![0, 7, 9];
        let end_time = vec![1, 8, 10];
        let expected = 7;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            expected
        );
        let event_time = 10;
        let start_time = vec![0, 3, 7, 9];
        let end_time = vec![1, 4, 8, 10];
        let expected = 6;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            expected
        );
        let event_time = 5;
        let start_time = vec![0, 1, 2, 3, 4];
        let end_time = vec![1, 2, 3, 4, 5];
        let expected = 0;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            expected
        );
    }
}
