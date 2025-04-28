///
/// # 1444. Number of Ways of Cutting a Pizza
///
/// Given a rectangular pizza represented as a `rows x cols` matrix containing the following characters: `'A'` (an apple) and `'.'` (empty cell) and given the integer `k`. You have to cut the pizza into `k` pieces using `k-1` cuts.
///
/// For each cut you choose the direction: vertical or horizontal, then you choose a cut position at the cell boundary and cut the pizza into two pieces. If you cut the pizza vertically, give the left part of the pizza to a person. If you cut the pizza horizontally, give the upper part of the pizza to a person. Give the last piece of pizza to the last person.
///
/// *Return the number of ways of cutting the pizza such that each piece contains **at least** one apple.* Since the answer can be a huge number, return this modulo 10^9 + 7.
///
/// **Example 1:**
///
/// **![](https://assets.leetcode.com/uploads/2020/04/23/ways_to_cut_apple_1.png)**
///
/// ```
/// Input: pizza = ["A..","AAA","..."], k = 3
/// Output: 3
/// Explanation: The figure above shows the three ways to cut the pizza. Note that pieces must contain at least one apple.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: pizza = ["A..","AA.","..."], k = 3
/// Output: 1
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: pizza = ["A..","A..","..."], k = 1
/// Output: 1
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= rows, cols <= 50`
/// * `rows == pizza.length`
/// * `cols == pizza[i].length`
/// * `1 <= k <= 10`
/// * `pizza` consists of characters `'A'` and `'.'` only.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/
// discuss: https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        fn _ways(
            cache: &mut Vec<Vec<Vec<i32>>>,
            min_cuts: &mut Vec<Vec<Option<(usize, usize)>>>,
            i: usize,
            j: usize,
            pizza: &[String],
            k: usize,
        ) -> i32 {
            if cache[i][j][k] >= 0 {
                return cache[i][j][k];
            }

            let (min_cut_i, min_cut_j) = match min_cuts[i][j] {
                Some(x) => x,
                None => {
                    let mut min_cut_i = usize::MAX;
                    let mut min_cut_j = usize::MAX;

                    for (cut_i, row) in pizza.iter().enumerate().skip(i) {
                        for (cut_j, ch) in row.bytes().enumerate().skip(j) {
                            if ch == b'A' {
                                min_cut_i = min_cut_i.min(cut_i + 1);
                                min_cut_j = min_cut_j.min(cut_j + 1);
                            }
                        }
                    }

                    min_cuts[i][j] = Some((min_cut_i, min_cut_j));
                    (min_cut_i, min_cut_j)
                }
            };

            let mut result = 0;

            if k == 0 {
                if min_cut_i != usize::MAX {
                    result = 1;
                }
            } else {
                for cut_i in min_cut_i..pizza.len() {
                    result = (result + _ways(cache, min_cuts, cut_i, j, pizza, k - 1)) % MOD;
                }

                for cut_j in min_cut_j..pizza[0].len() {
                    result = (result + _ways(cache, min_cuts, i, cut_j, pizza, k - 1)) % MOD;
                }
            }

            cache[i][j][k] = result;
            result
        }

        _ways(
            &mut vec![vec![vec![-1; k as usize]; pizza[0].len()]; pizza.len()],
            &mut vec![vec![None; pizza[0].len()]; pizza.len()],
            0,
            0,
            &pizza,
            k as usize - 1,
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1444() {
        let pizza = vec_string!["A..", "AAA", "..."];
        let k = 3;
        let expected = 3;
        assert_eq!(Solution::ways(pizza, k), expected);
        let pizza = vec_string!["A..", "AA.", "..."];
        let k = 3;
        let expected = 1;
        assert_eq!(Solution::ways(pizza, k), expected);
        let pizza = vec_string!["A..", "A..", "..."];
        let k = 1;
        let expected = 1;
        assert_eq!(Solution::ways(pizza, k), expected);
    }
}
