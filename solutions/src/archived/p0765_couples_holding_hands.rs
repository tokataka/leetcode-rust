///
/// # 765. Couples Holding Hands
///
/// There are `n` couples sitting in `2n` seats arranged in a row and want to hold hands.
///
/// The people and seats are represented by an integer array `row` where `row[i]` is the ID of the person sitting in the `i<sup>th</sup>` seat. The couples are numbered in order, the first couple being `(0, 1)`, the second couple being `(2, 3)`, and so on with the last couple being `(2n - 2, 2n - 1)`.
///
/// Return *the minimum number of swaps so that every couple is sitting side by side*. A swap consists of choosing any two people, then they stand up and switch seats.
///
/// **Example 1:**
///
/// ```
/// Input: row = [0,2,1,3]
/// Output: 1
/// Explanation: We only need to swap the second (row[1]) and third (row[2]) person.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: row = [3,2,0,1]
/// Output: 0
/// Explanation: All couples are already seated side by side.
///
/// ```
///
/// **Constraints:**
///
/// * `2n == row.length`
/// * `2 <= n <= 30`
/// * `n` is even.
/// * `0 <= row[i] < 2n`
/// * All the elements of `row` are **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/couples-holding-hands/
// discuss: https://leetcode.com/problems/couples-holding-hands/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n = row.len();
        let mut map = vec![None; n / 2];

        let mut result = 0;

        for seat_idx in 0..n / 2 {
            let (a, b) = (
                row[seat_idx * 2] as usize / 2,
                row[seat_idx * 2 + 1] as usize / 2,
            );

            if a == b {
                continue;
            }

            match (map[a], map[b]) {
                (Some(x), Some(y)) => {
                    if x == b && y == a {
                        result += 1;
                    } else {
                        result += 2;

                        map[x] = Some(y);
                        map[y] = Some(x);
                    }
                }
                (Some(x), None) => {
                    result += 1;

                    map[b] = Some(x);
                    map[x] = Some(b);
                }
                (None, Some(x)) => {
                    result += 1;

                    map[a] = Some(x);
                    map[x] = Some(a);
                }
                _ => {
                    map[a] = Some(b);
                    map[b] = Some(a);
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_765() {
        // let row = vec![0, 2, 1, 3];
        // let expected = 1;
        // assert_eq!(Solution::min_swaps_couples(row), expected);
        // let row = vec![3, 2, 0, 1];
        // let expected = 0;
        // assert_eq!(Solution::min_swaps_couples(row), expected);
        let row = vec![0, 2, 4, 6, 7, 1, 3, 5];
        let expected = 3;
        assert_eq!(Solution::min_swaps_couples(row), expected);
    }
}
