///
/// # 2141. Maximum Running Time of N Computers
///
/// You have `n` computers. You are given the integer `n` and a **0-indexed** integer array `batteries` where the `i<sup>th</sup>` battery can **run** a computer for `batteries[i]` minutes. You are interested in running **all** `n` computers **simultaneously** using the given batteries.
///
/// Initially, you can insert **at most one battery** into each computer. After that and at any integer time moment, you can remove a battery from a computer and insert another battery **any number of times**. The inserted battery can be a totally new battery or a battery from another computer. You may assume that the removing and inserting processes take no time.
///
/// Note that the batteries cannot be recharged.
///
/// Return *the **maximum** number of minutes you can run all the* `n` *computers simultaneously.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/01/06/example1-fit.png)
///
/// ```
/// Input: n = 2, batteries = [3,3,3]
/// Output: 4
/// Explanation:
/// Initially, insert battery 0 into the first computer and battery 1 into the second computer.
/// After two minutes, remove battery 1 from the second computer and insert battery 2 instead. Note that battery 1 can still run for one minute.
/// At the end of the third minute, battery 0 is drained, and you need to remove it from the first computer and insert battery 1 instead.
/// By the end of the fourth minute, battery 1 is also drained, and the first computer is no longer running.
/// We can run the two computers simultaneously for at most 4 minutes, so we return 4.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/01/06/example2.png)
///
/// ```
/// Input: n = 2, batteries = [1,1,1,1]
/// Output: 2
/// Explanation:
/// Initially, insert battery 0 into the first computer and battery 2 into the second computer.
/// After one minute, battery 0 and battery 2 are drained so you need to remove them and insert battery 1 into the first computer and battery 3 into the second computer.
/// After another minute, battery 1 and battery 3 are also drained so the first and second computers are no longer running.
/// We can run the two computers simultaneously for at most 2 minutes, so we return 2.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= batteries.length <= 10<sup>5</sup>`
/// * `1 <= batteries[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-running-time-of-n-computers/
// discuss: https://leetcode.com/problems/maximum-running-time-of-n-computers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
        let n = n as i64;

        batteries.sort_unstable();

        let mut left = 1;
        let mut right = batteries.iter().map(|&x| x as i64).sum::<i64>() / n;

        while left < right {
            let mid = (left + right + 1) / 2;
            let mut extra = 0;

            for &battery in &batteries {
                extra += (battery as i64).min(mid);
            }

            if extra >= n * mid {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2141() {
        let n = 2;
        let batteries = vec![3, 3, 3];
        let expected = 4;
        assert_eq!(Solution::max_run_time(n, batteries), expected);
        let n = 2;
        let batteries = vec![1, 1, 1, 1];
        let expected = 2;
        assert_eq!(Solution::max_run_time(n, batteries), expected);
    }
}
