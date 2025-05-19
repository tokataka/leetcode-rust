///
/// # 1751. Maximum Number of Events That Can Be Attended II
///
/// You are given an array of `events` where `events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>, value<sub>i</sub>]`. The `i<sup>th</sup>` event starts at `startDay<sub>i</sub>`<sub> </sub>and ends at `endDay<sub>i</sub>`, and if you attend this event, you will receive a value of `value<sub>i</sub>`. You are also given an integer `k` which represents the maximum number of events you can attend.
///
/// You can only attend one event at a time. If you choose to attend an event, you must attend the **entire** event. Note that the end day is **inclusive**: that is, you cannot attend two events where one of them starts and the other ends on the same day.
///
/// Return *the **maximum sum** of values that you can receive by attending events.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60048-pm.png)
///
/// ```
/// Input: events = [[1,2,4],[3,4,3],[2,3,1]], k = 2
/// Output: 7
/// Explanation: Choose the green events, 0 and 1 (0-indexed) for a total value of 4 + 3 = 7.
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60150-pm.png)
///
/// ```
/// Input: events = [[1,2,4],[3,4,3],[2,3,10]], k = 2
/// Output: 10
/// Explanation: Choose event 2 for a total value of 10.
/// Notice that you cannot attend any other event as they overlap, and that you do not have to attend k events.
/// ```
///
/// **Example 3:**
///
/// **![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60703-pm.png)**
///
/// ```
/// Input: events = [[1,1,1],[2,2,2],[3,3,3],[4,4,4]], k = 3
/// Output: 9
/// Explanation: Although the events do not overlap, you can only attend 3 events. Pick the highest valued three.
/// ```
///
/// **Constraints:**
///
/// * `1 <= k <= events.length`
/// * `1 <= k * events.length <= 10<sup>6</sup>`
/// * `1 <= startDay<sub>i</sub> <= endDay<sub>i</sub> <= 10<sup>9</sup>`
/// * `1 <= value<sub>i</sub> <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/
// discuss: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        events.sort_unstable();

        fn _max_value(
            idx: usize,
            k: usize,
            events: &Vec<Vec<i32>>,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if k == 0 || idx >= events.len() {
                return 0;
            }

            if cache[idx][k] != i32::MAX {
                return cache[idx][k];
            }

            let [_, end, value] = events[idx][..] else {
                unreachable!()
            };

            let next_idx = events.partition_point(|x| x[0] <= end);

            let result = _max_value(idx + 1, k, events, cache)
                .max(_max_value(next_idx, k - 1, events, cache) + value);

            cache[idx][k] = result;

            result
        }

        _max_value(
            0,
            k as usize,
            &events,
            &mut vec![vec![i32::MAX; k as usize + 1]; events.len()],
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1751() {
        let events = nd_vec![[1, 2, 4], [3, 4, 3], [2, 3, 1]];
        let k = 2;
        let expected = 7;
        assert_eq!(Solution::max_value(events, k), expected);
        let events = nd_vec![[1, 2, 4], [3, 4, 3], [2, 3, 10]];
        let k = 2;
        let expected = 10;
        assert_eq!(Solution::max_value(events, k), expected);
        let events = nd_vec![[1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4]];
        let k = 3;
        let expected = 9;
        assert_eq!(Solution::max_value(events, k), expected);
    }
}
