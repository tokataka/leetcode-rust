///
/// # 3426. Manhattan Distances of All Arrangements of Pieces
///
/// You are given three integers `m`, `n`, and `k`.
///
/// There is a rectangular grid of size `m × n` containing `k` identical pieces. Return the sum of Manhattan distances between every pair of pieces over all **valid arrangements** of pieces.
///
/// A **valid arrangement** is a placement of all `k` pieces on the grid with **at most** one piece per cell.
///
/// Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// The Manhattan Distance between two cells `(x<sub>i</sub>, y<sub>i</sub>)` and `(x<sub>j</sub>, y<sub>j</sub>)` is `|x<sub>i</sub> - x<sub>j</sub>| + |y<sub>i</sub> - y<sub>j</sub>|`.
///
/// **Example 1:**
///
/// **Input:** m = 2, n = 2, k = 2
///
/// **Output:** 8
///
/// **Explanation:**
///
/// The valid arrangements of pieces on the board are:
///
/// ![](https://assets.leetcode.com/uploads/2024/12/25/4040example1.drawio)![](https://assets.leetcode.com/uploads/2024/12/25/untitled-diagramdrawio.png)
///
/// * In the first 4 arrangements, the Manhattan distance between the two pieces is 1.
/// * In the last 2 arrangements, the Manhattan distance between the two pieces is 2.
///
/// Thus, the total Manhattan distance across all valid arrangements is `1 + 1 + 1 + 1 + 2 + 2 = 8`.
///
/// **Example 2:**
///
/// **Input:** m = 1, n = 4, k = 3
///
/// **Output:** 20
///
/// **Explanation:**
///
/// The valid arrangements of pieces on the board are:
///
/// ![](https://assets.leetcode.com/uploads/2024/12/25/4040example2drawio.png)
///
/// * The first and last arrangements have a total Manhattan distance of `1 + 1 + 2 = 4`.
/// * The middle two arrangements have a total Manhattan distance of `1 + 2 + 3 = 6`.
///
/// The total Manhattan distance between all pairs of pieces across all arrangements is `4 + 6 + 6 + 4 = 20`.
///
/// **Constraints:**
///
/// * `1 <= m, n <= 10<sup>5</sup>`
/// * `2 <= m * n <= 10<sup>5</sup>`
/// * `2 <= k <= m * n`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/manhattan-distances-of-all-arrangements-of-pieces/
// discuss: https://leetcode.com/problems/manhattan-distances-of-all-arrangements-of-pieces/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

fn powmod(mut a: i64, mut b: i64, p: i64) -> i64 {
    a %= p;

    if a == 0 {
        return 0;
    }

    let mut product = 1;

    while b > 0 {
        if b & 1 == 1 {
            product = (product * a) % p;
            b -= 1;
        }

        a = (a * a) % p;
        b /= 2;
    }

    product
}

fn inv(a: i64, p: i64) -> i64 {
    powmod(a, p - 2, p)
}

impl Solution {
    pub fn distance_sum(m: i32, n: i32, k: i32) -> i32 {
        let m = m as i64;
        let n = n as i64;
        let k = k as i64;

        const MOD: i64 = 1_000_000_007;

        let mut numer = 1;
        let mut denom = 1;

        for i in 0..(k - 2).min(m * n - k) {
            numer = (numer * (m * n - 2 - i)) % MOD;
            denom = (denom * (i + 1)) % MOD;
        }

        let mult = (numer * inv(denom, MOD)) % MOD;

        let distances =
            (((n - 1) * n * (n + 1) * m * m / 6) + ((m - 1) * m * (m + 1) * n * n / 6)) % MOD;

        ((distances * mult) % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3426() {
        // let m = 2;
        // let n = 2;
        // let k = 2;
        // let expected = 8;
        // assert_eq!(Solution::distance_sum(m, n, k), expected);
        // let m = 1;
        // let n = 4;
        // let k = 3;
        // let expected = 20;
        // assert_eq!(Solution::distance_sum(m, n, k), expected);
        let m = 10;
        let n = 1500;
        let k = 5577;
        let expected = 917984626;
        assert_eq!(Solution::distance_sum(m, n, k), expected);
    }
}
