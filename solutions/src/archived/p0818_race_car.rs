///
/// # 818. Race Car
///
/// Your car starts at position `0` and speed `+1` on an infinite number line. Your car can go into negative positions. Your car drives automatically according to a sequence of instructions `'A'` (accelerate) and `'R'` (reverse):
///
/// * When you get an instruction `'A'`, your car does the following:
///   * `position += speed`
///   * `speed *= 2`
///
/// * When you get an instruction `'R'`, your car does the following:
///   * If your speed is positive then `speed = -1`
///   * otherwise `speed = 1`
///
///    Your position stays the same.
///
/// For example, after commands `"AAR"`, your car goes to positions `0 --> 1 --> 3 --> 3`, and your speed goes to `1 --> 2 --> 4 --> -1`.
///
/// Given a target position `target`, return *the length of the shortest sequence of instructions to get there*.
///
/// **Example 1:**
///
/// ```
/// Input: target = 3
/// Output: 2
/// Explanation:
/// The shortest instruction sequence is "AA".
/// Your position goes from 0 --> 1 --> 3.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: target = 6
/// Output: 5
/// Explanation:
/// The shortest instruction sequence is "AAARA".
/// Your position goes from 0 --> 1 --> 3 --> 7 --> 7 --> 6.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= target <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/race-car/
// discuss: https://leetcode.com/problems/race-car/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 2];

        for i in 1..=target {
            let n = ((i + 1) as f64).log2().floor() as i32;
            if (1 << n) - 1 == i as i32 {
                dp[i] = n;
                continue;
            }

            let n_ceil = ((i + 1) as f64).log2().ceil() as i32;
            let pos_n_ceil = (1 << n_ceil) - 1;
            dp[i] = n_ceil + 1 + dp[pos_n_ceil as usize - i];

            let pos_n = (1 << n) - 1;
            for m in 0..n {
                let back_dist = (1 << m) - 1;
                let current_pos = pos_n - back_dist;
                let steps_so_far = n + 1 + m + 1;
                let remaining_dist = i - current_pos as usize;
                dp[i] = dp[i].min(steps_so_far + dp[remaining_dist]);
            }
        }
        dp[target]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_818() {
        assert_eq!(Solution::racecar(1), 1);
        assert_eq!(Solution::racecar(2), 4);
        assert_eq!(Solution::racecar(3), 2);
        assert_eq!(Solution::racecar(4), 5);
        assert_eq!(Solution::racecar(5), 7);
        assert_eq!(Solution::racecar(6), 5);
        let target = 20;
        let expected = 12;
        assert_eq!(Solution::racecar(target), expected);
        let target = 8681;
        let expected = 34;
        assert_eq!(Solution::racecar(target), expected);
    }
}
