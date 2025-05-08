///
/// # 790. Domino and Tromino Tiling
///
/// You have two types of tiles: a `2 x 1` domino shape and a tromino shape. You may rotate these shapes.
///
/// ![](https://assets.leetcode.com/uploads/2021/07/15/lc-domino.jpg)
///
/// Given an integer n, return *the number of ways to tile an* `2 x n` *board*. Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// In a tiling, every square must be covered by a tile. Two tilings are different if and only if there are two 4-directionally adjacent cells on the board such that exactly one of the tilings has both squares occupied by a tile.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/07/15/lc-domino1.jpg)
///
/// ```
/// Input: n = 3
/// Output: 5
/// Explanation: The five different ways are show above.
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
/// * `1 <= n <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/domino-and-tromino-tiling/
// discuss: https://leetcode.com/problems/domino-and-tromino-tiling/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        fn _num_tilings(n: usize, t: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
            if cache[n][t] != i32::MAX {
                return cache[n][t];
            }

            if n == 0 {
                return 1;
            }

            let mut result = 0;

            if t == 0 {
                // vertical domino
                result += _num_tilings(n - 1, 0, cache);
            }

            if n >= 2 {
                if t == 0 {
                    result = (result + _num_tilings(n - 2, 0, cache)) % MOD;
                    result = (result + (2 * _num_tilings(n - 1, 1, cache) % MOD)) % MOD;
                } else {
                    result = (result + _num_tilings(n - 2, 0, cache)) % MOD;
                    result = (result + _num_tilings(n - 1, 1, cache)) % MOD;
                }
            }

            cache[n][t] = result;
            result
        }

        // cache[n][type], type 0: both avail, 1: one avail
        let mut cache = vec![vec![i32::MAX; 2]; n as usize + 1];

        _num_tilings(n as usize, 0, &mut cache)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_790() {
        let n = 3;
        let expected = 5;
        assert_eq!(Solution::num_tilings(n), expected);
        let n = 1;
        let expected = 1;
        assert_eq!(Solution::num_tilings(n), expected);
    }
}
