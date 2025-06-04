use std::collections::BinaryHeap;

///
/// # 2931. Maximum Spending After Buying Items
///
/// You are given a **0-indexed** `m * n` integer matrix `values`, representing the values of `m * n` different items in `m` different shops. Each shop has `n` items where the `j<sup>th</sup>` item in the `i<sup>th</sup>` shop has a value of `values[i][j]`. Additionally, the items in the `i<sup>th</sup>` shop are sorted in non-increasing order of value. That is, `values[i][j] >= values[i][j + 1]` for all `0 <= j < n - 1`.
///
/// On each day, you would like to buy a single item from one of the shops. Specifically, On the `d<sup>th</sup>` day you can:
///
/// * Pick any shop `i`.
/// * Buy the rightmost available item `j` for the price of `values[i][j] * d`. That is, find the greatest index `j` such that item `j` was never bought before, and buy it for the price of `values[i][j] * d`.
///
/// **Note** that all items are pairwise different. For example, if you have bought item `0` from shop `1`, you can still buy item `0` from any other shop.
///
/// Return *the **maximum amount of money that can be spent** on buying all* `m * n` *products*.
///
/// **Example 1:**
///
/// ```
/// Input: values = [[8,5,2],[6,4,1],[9,7,3]]
/// Output: 285
/// Explanation: On the first day, we buy product 2 from shop 1 for a price of values[1][2] * 1 = 1.
/// On the second day, we buy product 2 from shop 0 for a price of values[0][2] * 2 = 4.
/// On the third day, we buy product 2 from shop 2 for a price of values[2][2] * 3 = 9.
/// On the fourth day, we buy product 1 from shop 1 for a price of values[1][1] * 4 = 16.
/// On the fifth day, we buy product 1 from shop 0 for a price of values[0][1] * 5 = 25.
/// On the sixth day, we buy product 0 from shop 1 for a price of values[1][0] * 6 = 36.
/// On the seventh day, we buy product 1 from shop 2 for a price of values[2][1] * 7 = 49.
/// On the eighth day, we buy product 0 from shop 0 for a price of values[0][0] * 8 = 64.
/// On the ninth day, we buy product 0 from shop 2 for a price of values[2][0] * 9 = 81.
/// Hence, our total spending is equal to 285.
/// It can be shown that 285 is the maximum amount of money that can be spent buying all m * n products.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: values = [[10,8,6,4,2],[9,7,5,3,2]]
/// Output: 386
/// Explanation: On the first day, we buy product 4 from shop 0 for a price of values[0][4] * 1 = 2.
/// On the second day, we buy product 4 from shop 1 for a price of values[1][4] * 2 = 4.
/// On the third day, we buy product 3 from shop 1 for a price of values[1][3] * 3 = 9.
/// On the fourth day, we buy product 3 from shop 0 for a price of values[0][3] * 4 = 16.
/// On the fifth day, we buy product 2 from shop 1 for a price of values[1][2] * 5 = 25.
/// On the sixth day, we buy product 2 from shop 0 for a price of values[0][2] * 6 = 36.
/// On the seventh day, we buy product 1 from shop 1 for a price of values[1][1] * 7 = 49.
/// On the eighth day, we buy product 1 from shop 0 for a price of values[0][1] * 8 = 64
/// On the ninth day, we buy product 0 from shop 1 for a price of values[1][0] * 9 = 81.
/// On the tenth day, we buy product 0 from shop 0 for a price of values[0][0] * 10 = 100.
/// Hence, our total spending is equal to 386.
/// It can be shown that 386 is the maximum amount of money that can be spent buying all m * n products.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= m == values.length <= 10`
/// * `1 <= n == values[i].length <= 10<sup>4</sup>`
/// * `1 <= values[i][j] <= 10<sup>6</sup>`
/// * `values[i]` are sorted in non-increasing order.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-spending-after-buying-items/
// discuss: https://leetcode.com/problems/maximum-spending-after-buying-items/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let m = values.len();
        let n = values[0].len();

        let mut iters = values.iter().map(|x| x.iter()).collect::<Vec<_>>();

        let mut pq = iters
            .iter_mut()
            .enumerate()
            .filter_map(|(i, iter)| iter.next().map(|&x| (x, i)))
            .collect::<BinaryHeap<_>>();

        let mut result = 0;
        let mut d = m as i64 * n as i64;

        while let Some((max, i)) = pq.pop() {
            result += max as i64 * d;
            d -= 1;

            let next_max = match pq.peek() {
                Some(&(next_max, _)) => next_max,
                None => -1,
            };

            for &next in iters[i].by_ref() {
                if next < next_max {
                    pq.push((next, i));
                    break;
                }

                result += next as i64 * d;
                d -= 1;
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
    fn test_2931() {
        let values = nd_vec![[8, 5, 2], [6, 4, 1], [9, 7, 3]];
        let expected = 285;
        assert_eq!(Solution::max_spending(values), expected);
        let values = nd_vec![[10, 8, 6, 4, 2], [9, 7, 5, 3, 2]];
        let expected = 386;
        assert_eq!(Solution::max_spending(values), expected);
    }
}
