///
/// # 2280. Minimum Lines to Represent a Line Chart
///
/// You are given a 2D integer array `stockPrices` where `stockPrices[i] = [day<sub>i</sub>, price<sub>i</sub>]` indicates the price of the stock on day `day<sub>i</sub>` is `price<sub>i</sub>`. A **line chart** is created from the array by plotting the points on an XY plane with the X-axis representing the day and the Y-axis representing the price and connecting adjacent points. One such example is shown below:
///
/// ![](https://assets.leetcode.com/uploads/2022/03/30/1920px-pushkin_population_historysvg.png)
///
/// Return *the **minimum number of lines** needed to represent the line chart*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/30/ex0.png)
///
/// ```
/// Input: stockPrices = [[1,7],[2,6],[3,5],[4,4],[5,4],[6,3],[7,2],[8,1]]
/// Output: 3
/// Explanation:
/// The diagram above represents the input, with the X-axis representing the day and Y-axis representing the price.
/// The following 3 lines can be drawn to represent the line chart:
/// - Line 1 (in red) from (1,7) to (4,4) passing through (1,7), (2,6), (3,5), and (4,4).
/// - Line 2 (in blue) from (4,4) to (5,4).
/// - Line 3 (in green) from (5,4) to (8,1) passing through (5,4), (6,3), (7,2), and (8,1).
/// It can be shown that it is not possible to represent the line chart using less than 3 lines.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/03/30/ex1.png)
///
/// ```
/// Input: stockPrices = [[3,4],[1,2],[7,8],[2,3]]
/// Output: 1
/// Explanation:
/// As shown in the diagram above, the line chart can be represented with a single line.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= stockPrices.length <= 10<sup>5</sup>`
/// * `stockPrices[i].length == 2`
/// * `1 <= day<sub>i</sub>, price<sub>i</sub> <= 10<sup>9</sup>`
/// * All `day<sub>i</sub>` are **distinct**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-lines-to-represent-a-line-chart/
// discuss: https://leetcode.com/problems/minimum-lines-to-represent-a-line-chart/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
        fn calc_gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                calc_gcd(b, a % b)
            }
        }

        stock_prices.sort_unstable();

        stock_prices
            .windows(2)
            .fold((0, (0, 0)), |(result, (prev_dx, prev_dy)), x| {
                let (dx, dy) = (x[1][0] - x[0][0], x[1][1] - x[0][1]);
                let gcd = calc_gcd(dx, dy);

                let (dx, dy) = (dx / gcd, dy / gcd);

                if prev_dx == dx && prev_dy == dy {
                    (result, (dx, dy))
                } else {
                    (result + 1, (dx, dy))
                }
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2280() {
        let stock_prices = nd_vec![
            [1, 7],
            [2, 6],
            [3, 5],
            [4, 4],
            [5, 4],
            [6, 3],
            [7, 2],
            [8, 1]
        ];
        let expected = 3;
        assert_eq!(Solution::minimum_lines(stock_prices), expected);
        let stock_prices = nd_vec![[3, 4], [1, 2], [7, 8], [2, 3]];
        let expected = 1;
        assert_eq!(Solution::minimum_lines(stock_prices), expected);
    }
}
