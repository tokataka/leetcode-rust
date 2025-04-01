///
/// # 3169. Count Days Without Meetings
///
/// You are given a positive integer `days` representing the total number of days an employee is available for work (starting from day 1). You are also given a 2D array `meetings` of size `n` where, `meetings[i] = [start_i, end_i]` represents the starting and ending days of meeting `i` (inclusive).
///
/// Return the count of days when the employee is available for work but no meetings are scheduled.
///
/// **Note:** The meetings may overlap.
///
/// **Example 1:**
///
/// **Input:** days = 10, meetings = [[5,7],[1,3],[9,10]]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// There is no meeting scheduled on the 4<sup>th</sup> and 8<sup>th</sup> days.
///
/// **Example 2:**
///
/// **Input:** days = 5, meetings = [[2,4],[1,3]]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// There is no meeting scheduled on the 5<sup>th </sup>day.
///
/// **Example 3:**
///
/// **Input:** days = 6, meetings = [[1,6]]
///
/// **Output:** 0
///
/// **Explanation:**
///
/// Meetings are scheduled for all working days.
///
/// **Constraints:**
///
/// * `1 <= days <= 10<sup>9</sup>`
/// * `1 <= meetings.length <= 10<sup>5</sup>`
/// * `meetings[i].length == 2`
/// * `1 <= meetings[i][0] <= meetings[i][1] <= days`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-days-without-meetings/
// discuss: https://leetcode.com/problems/count-days-without-meetings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        meetings.sort_unstable();

        let mut result = 0;
        let mut prev_end = 0;

        for meeting in meetings {
            let (start, end) = (meeting[0], meeting[1]);

            if start > prev_end + 1 {
                result += start - prev_end - 1;
            }

            prev_end = prev_end.max(end);
        }

        if days > prev_end {
            result += days - prev_end;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3169() {
        let days = 10;
        let meetings = nd_vec![[5, 7], [1, 3], [9, 10]];
        let expected = 2;
        assert_eq!(Solution::count_days(days, meetings), expected);
        let days = 5;
        let meetings = nd_vec![[2, 4], [1, 3]];
        let expected = 1;
        assert_eq!(Solution::count_days(days, meetings), expected);
        let days = 6;
        let meetings = nd_vec![[1, 6]];
        let expected = 0;
        assert_eq!(Solution::count_days(days, meetings), expected);
    }
}
