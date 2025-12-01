///
/// # 2435. Paths in Matrix Whose Sum Is Divisible by K
///
/// You are given a **0-indexed** `m x n` integer matrix `grid` and an integer `k`. You are currently at position `(0, 0)` and you want to reach position `(m - 1, n - 1)` moving only **down** or **right**.
///
/// Return *the number of paths where the sum of the elements on the path is divisible by* `k`. Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/08/13/image-20220813183124-1.png)
///
/// ```
/// Input: grid = [[5,2,4],[3,0,5],[0,7,2]], k = 3
/// Output: 2
/// Explanation: There are two paths where the sum of the elements on the path is divisible by k.
/// The first path highlighted in red has a sum of 5 + 2 + 4 + 5 + 2 = 18 which is divisible by 3.
/// The second path highlighted in blue has a sum of 5 + 3 + 0 + 5 + 2 = 15 which is divisible by 3.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/08/17/image-20220817112930-3.png)
///
/// ```
/// Input: grid = [[0,0]], k = 5
/// Output: 1
/// Explanation: The path highlighted in red has a sum of 0 + 0 = 0 which is divisible by 5.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2022/08/12/image-20220812224605-3.png)
///
/// ```
/// Input: grid = [[7,3,4,9],[2,3,6,2],[2,3,7,0]], k = 1
/// Output: 10
/// Explanation: Every integer is divisible by 1 so the sum of the elements on every possible path is divisible by k.
///
/// ```
///
/// **Constraints:**
///
/// * `m == grid.length`
/// * `n == grid[i].length`
/// * `1 <= m, n <= 5 * 10<sup>4</sup>`
/// * `1 <= m * n <= 5 * 10<sup>4</sup>`
/// * `0 <= grid[i][j] <= 100`
/// * `1 <= k <= 50`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/
// discuss: https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let n = grid.len();
        let m = grid[0].len();
        let k = k as usize;

        let mut dp = vec![[0; 50]; m + 1];
        dp[1][0] = 1;

        let mut temp = [0; 50];

        for i in 0..n {
            for j in 0..m {
                let num = grid[i][j] as usize;

                for sum in 0..k {
                    temp[(sum + num) % k] = (dp[j + 1][sum] + dp[j][sum]) % MOD;
                }

                dp[j + 1] = temp;
            }
        }

        dp[m][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2435() {
        assert_eq!(
            Solution::number_of_paths(nd_vec![[5, 2, 4], [3, 0, 5], [0, 7, 2]], 3),
            2
        );
        assert_eq!(Solution::number_of_paths(nd_vec![[0, 0]], 5), 1);
        assert_eq!(
            Solution::number_of_paths(nd_vec![[7, 3, 4, 9], [2, 3, 6, 2], [2, 3, 7, 0]], 1),
            10
        );
        assert_eq!(
            Solution::number_of_paths(nd_vec![[1], [5], [3], [7], [3], [2], [3], [5]], 29),
            1
        );
    }
}
