///
/// # 390. Elimination Game
///
/// You have a list `arr` of all integers in the range `[1, n]` sorted in a strictly increasing order. Apply the following algorithm on `arr`:
///
/// * Starting from left to right, remove the first number and every other number afterward until you reach the end of the list.
/// * Repeat the previous step again, but this time from right to left, remove the rightmost number and every other number from the remaining numbers.
/// * Keep repeating the steps again, alternating left to right and right to left, until a single number remains.
///
/// Given the integer `n`, return *the last number that remains in* `arr`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 9
/// Output: 6
/// Explanation:
/// arr = [1, 2, 3, 4, 5, 6, 7, 8, 9]
/// arr = [2, 4, 6, 8]
/// arr = [2, 6]
/// arr = [6]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 1
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/elimination-game/
// discuss: https://leetcode.com/problems/elimination-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut base = 1;
        let mut offset = 0;
        let mut n = n;

        let mut is_left_to_right = true;

        while n > 1 {
            if !is_left_to_right && n % 2 == 0 {
                offset -= base;
            }

            base *= 2;
            n /= 2;
            is_left_to_right = !is_left_to_right;
        }

        base + offset
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_390() {
        let n = 9;
        let expected = 6;
        assert_eq!(Solution::last_remaining(n), expected);
        let n = 1;
        let expected = 1;
        assert_eq!(Solution::last_remaining(n), expected);
    }
}
