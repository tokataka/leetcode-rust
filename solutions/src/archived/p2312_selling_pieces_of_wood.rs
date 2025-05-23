///
/// # 2312. Selling Pieces of Wood
///
/// You are given two integers `m` and `n` that represent the height and width of a rectangular piece of wood. You are also given a 2D integer array `prices`, where `prices[i] = [h<sub>i</sub>, w<sub>i</sub>, price<sub>i</sub>]` indicates you can sell a rectangular piece of wood of height `h<sub>i</sub>` and width `w<sub>i</sub>` for `price<sub>i</sub>` dollars.
///
/// To cut a piece of wood, you must make a vertical or horizontal cut across the **entire** height or width of the piece to split it into two smaller pieces. After cutting a piece of wood into some number of smaller pieces, you can sell pieces according to `prices`. You may sell multiple pieces of the same shape, and you do not have to sell all the shapes. The grain of the wood makes a difference, so you **cannot** rotate a piece to swap its height and width.
///
/// Return *the **maximum** money you can earn after cutting an* `m x n` *piece of wood*.
///
/// Note that you can cut the piece of wood as many times as you want.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/04/27/ex1.png)
///
/// ```
/// Input: m = 3, n = 5, prices = [[1,4,2],[2,2,7],[2,1,3]]
/// Output: 19
/// Explanation: The diagram above shows a possible scenario. It consists of:
/// - 2 pieces of wood shaped 2 x 2, selling for a price of 2 * 7 = 14.
/// - 1 piece of wood shaped 2 x 1, selling for a price of 1 * 3 = 3.
/// - 1 piece of wood shaped 1 x 4, selling for a price of 1 * 2 = 2.
/// This obtains a total of 14 + 3 + 2 = 19 money earned.
/// It can be shown that 19 is the maximum amount of money that can be earned.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/04/27/ex2new.png)
///
/// ```
/// Input: m = 4, n = 6, prices = [[3,2,10],[1,4,2],[4,1,3]]
/// Output: 32
/// Explanation: The diagram above shows a possible scenario. It consists of:
/// - 3 pieces of wood shaped 3 x 2, selling for a price of 3 * 10 = 30.
/// - 1 piece of wood shaped 1 x 4, selling for a price of 1 * 2 = 2.
/// This obtains a total of 30 + 2 = 32 money earned.
/// It can be shown that 32 is the maximum amount of money that can be earned.
/// Notice that we cannot rotate the 1 x 4 piece of wood to obtain a 4 x 1 piece of wood.
/// ```
///
/// **Constraints:**
///
/// * `1 <= m, n <= 200`
/// * `1 <= prices.length <= 2 * 10<sup>4</sup>`
/// * `prices[i].length == 3`
/// * `1 <= h<sub>i</sub> <= m`
/// * `1 <= w<sub>i</sub> <= n`
/// * `1 <= price<sub>i</sub> <= 10<sup>6</sup>`
/// * All the shapes of wood `(h<sub>i</sub>, w<sub>i</sub>)` are pairwise **distinct**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/selling-pieces-of-wood/
// discuss: https://leetcode.com/problems/selling-pieces-of-wood/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        fn _selling_wood(
            m: usize,
            n: usize,
            prices: &Vec<Vec<i64>>,
            cache: &mut Vec<Vec<i64>>,
        ) -> i64 {
            if cache[m][n] != i64::MAX {
                return cache[m][n];
            }

            let mut result = prices[m][n];

            for next_m in 1..=(m / 2) {
                result = result.max(
                    _selling_wood(next_m, n, prices, cache)
                        + _selling_wood(m - next_m, n, prices, cache),
                );
            }

            for next_n in 1..=(n / 2) {
                result = result.max(
                    _selling_wood(m, next_n, prices, cache)
                        + _selling_wood(m, n - next_n, prices, cache),
                );
            }

            cache[m][n] = result;
            result
        }

        let mut cache = vec![vec![i64::MAX; 201]; 201];
        let prices = prices
            .into_iter()
            .fold(vec![vec![0; 201]; 201], |mut acc, x| {
                acc[x[0] as usize][x[1] as usize] = x[2] as i64;
                acc
            });

        _selling_wood(m as usize, n as usize, &prices, &mut cache)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2312() {
        let m = 3;
        let n = 5;
        let prices = nd_vec![[1, 4, 2], [2, 2, 7], [2, 1, 3]];
        let expected = 19;
        assert_eq!(Solution::selling_wood(m, n, prices), expected);
        let m = 4;
        let n = 6;
        let prices = nd_vec![[3, 2, 10], [1, 4, 2], [4, 1, 3]];
        let expected = 32;
        assert_eq!(Solution::selling_wood(m, n, prices), expected);
    }
}
