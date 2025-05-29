///
/// # 2188. Minimum Time to Finish the Race
///
/// You are given a **0-indexed** 2D integer array `tires` where `tires[i] = [f<sub>i</sub>, r<sub>i</sub>]` indicates that the `i<sup>th</sup>` tire can finish its `x<sup>th</sup>` successive lap in `f<sub>i</sub> * r<sub>i</sub><sup>(x-1)</sup>` seconds.
///
/// * For example, if `f<sub>i</sub> = 3` and `r<sub>i</sub> = 2`, then the tire would finish its `1<sup>st</sup>` lap in `3` seconds, its `2<sup>nd</sup>` lap in `3 * 2 = 6` seconds, its `3<sup>rd</sup>` lap in `3 * 2<sup>2</sup> = 12` seconds, etc.
///
/// You are also given an integer `changeTime` and an integer `numLaps`.
///
/// The race consists of `numLaps` laps and you may start the race with **any** tire. You have an **unlimited** supply of each tire and after every lap, you may **change** to any given tire (including the current tire type) if you wait `changeTime` seconds.
///
/// Return *the **minimum** time to finish the race.*
///
/// **Example 1:**
///
/// ```
/// Input: tires = [[2,3],[3,4]], changeTime = 5, numLaps = 4
/// Output: 21
/// Explanation:
/// Lap 1: Start with tire 0 and finish the lap in 2 seconds.
/// Lap 2: Continue with tire 0 and finish the lap in 2 * 3 = 6 seconds.
/// Lap 3: Change tires to a new tire 0 for 5 seconds and then finish the lap in another 2 seconds.
/// Lap 4: Continue with tire 0 and finish the lap in 2 * 3 = 6 seconds.
/// Total time = 2 + 6 + 5 + 2 + 6 = 21 seconds.
/// The minimum time to complete the race is 21 seconds.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: tires = [[1,10],[2,2],[3,4]], changeTime = 6, numLaps = 5
/// Output: 25
/// Explanation:
/// Lap 1: Start with tire 1 and finish the lap in 2 seconds.
/// Lap 2: Continue with tire 1 and finish the lap in 2 * 2 = 4 seconds.
/// Lap 3: Change tires to a new tire 1 for 6 seconds and then finish the lap in another 2 seconds.
/// Lap 4: Continue with tire 1 and finish the lap in 2 * 2 = 4 seconds.
/// Lap 5: Change tires to tire 0 for 6 seconds then finish the lap in another 1 second.
/// Total time = 2 + 4 + 6 + 2 + 4 + 6 + 1 = 25 seconds.
/// The minimum time to complete the race is 25 seconds.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= tires.length <= 10<sup>5</sup>`
/// * `tires[i].length == 2`
/// * `1 <= f<sub>i</sub>, changeTime <= 10<sup>5</sup>`
/// * `2 <= r<sub>i</sub> <= 10<sup>5</sup>`
/// * `1 <= numLaps <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-to-finish-the-race/
// discuss: https://leetcode.com/problems/minimum-time-to-finish-the-race/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        let num_laps = num_laps as usize;

        let mut cache = vec![0; num_laps + 1];

        let mut tires_pow = tires.iter().map(|x| (x[0], x[0], x[1])).collect::<Vec<_>>();
        let mut min_time = tires_pow.iter().map(|x| x.0).min().unwrap_or(i32::MAX);

        for laps in 1..=num_laps {
            for laps_split in 1..=(laps / 2) {
                min_time = min_time.min(cache[laps_split] + cache[laps - laps_split] + change_time);
            }

            cache[laps] = min_time;

            min_time = i32::MAX;

            tires_pow.retain_mut(|x| {
                x.1 = x.1.saturating_mul(x.2);
                x.0 = x.0.saturating_add(x.1);
                min_time = min_time.min(x.0);

                x.0 != i32::MAX
            });
        }

        cache[num_laps]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2188() {
        let tires = nd_vec![[2, 3], [3, 4]];
        let change_time = 5;
        let num_laps = 4;
        let expected = 21;
        assert_eq!(
            Solution::minimum_finish_time(tires, change_time, num_laps),
            expected
        );
        let tires = nd_vec![[1, 10], [2, 2], [3, 4]];
        let change_time = 6;
        let num_laps = 5;
        let expected = 25;
        assert_eq!(
            Solution::minimum_finish_time(tires, change_time, num_laps),
            expected
        );
    }
}
