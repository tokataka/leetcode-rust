///
/// # 517. Super Washing Machines
///
/// You have `n` super washing machines on a line. Initially, each washing machine has some dresses or is empty.
///
/// For each move, you could choose any `m` (`1 <= m <= n`) washing machines, and pass one dress of each washing machine to one of its adjacent washing machines at the same time.
///
/// Given an integer array `machines` representing the number of dresses in each washing machine from left to right on the line, return *the minimum number of moves to make all the washing machines have the same number of dresses*. If it is not possible to do it, return `-1`.
///
/// **Example 1:**
///
/// ```
/// Input: machines = [1,0,5]
/// Output: 3
/// Explanation:
/// 1st move:    1     0 <-- 5    =>    1     1     4
/// 2nd move:    1 <-- 1 <-- 4    =>    2     1     3
/// 3rd move:    2     1 <-- 3    =>    2     2     2
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: machines = [0,3,0]
/// Output: 2
/// Explanation:
/// 1st move:    0 <-- 3     0    =>    1     2     0
/// 2nd move:    1     2 --> 0    =>    1     1     1
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: machines = [0,2,0]
/// Output: -1
/// Explanation:
/// It's impossible to make all three washing machines have the same number of dresses.
///
/// ```
///
/// **Constraints:**
///
/// * `n == machines.length`
/// * `1 <= n <= 10<sup>4</sup>`
/// * `0 <= machines[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/super-washing-machines/
// discuss: https://leetcode.com/problems/super-washing-machines/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len();

        let total = machines.iter().sum::<i32>();

        if total == 0 {
            return 0;
        }

        if total % n as i32 != 0 {
            return -1;
        }

        let avg = total / n as i32;

        let mut moves = vec![0; n];

        let mut diff = 0;

        for (i, &machine) in machines.iter().enumerate() {
            diff += machine - avg;

            moves[i] += 0.max(diff);
        }

        for (i, &machine) in machines.iter().enumerate().rev() {
            diff += machine - avg;

            moves[i] += 0.max(diff);
        }

        moves.into_iter().max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_517() {
        let machines = vec![1, 0, 5];
        let expected = 3;
        assert_eq!(Solution::find_min_moves(machines), expected);
        let machines = vec![0, 3, 0];
        let expected = 2;
        assert_eq!(Solution::find_min_moves(machines), expected);
        let machines = vec![0, 2, 0];
        let expected = -1;
        assert_eq!(Solution::find_min_moves(machines), expected);
        let machines = vec![0, 0, 4, 0, 5, 0, 5, 0, 4];
        let expected = 4;
        assert_eq!(Solution::find_min_moves(machines), expected);
    }
}
