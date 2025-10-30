///
/// # 3100. Water Bottles II
///
/// You are given two integers `numBottles` and `numExchange`.
///
/// `numBottles` represents the number of full water bottles that you initially have. In one operation, you can perform one of the following operations:
///
/// * Drink any number of full water bottles turning them into empty bottles.
/// * Exchange `numExchange` empty bottles with one full water bottle. Then, increase `numExchange` by one.
///
/// Note that you cannot exchange multiple batches of empty bottles for the same value of `numExchange`. For example, if `numBottles == 3` and `numExchange == 1`, you cannot exchange `3` empty water bottles for `3` full bottles.
///
/// Return *the **maximum** number of water bottles you can drink*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2024/01/28/exampleone1.png)
///
/// ```
/// Input: numBottles = 13, numExchange = 6
/// Output: 15
/// Explanation: The table above shows the number of full water bottles, empty water bottles, the value of numExchange, and the number of bottles drunk.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2024/01/28/example231.png)
///
/// ```
/// Input: numBottles = 10, numExchange = 3
/// Output: 13
/// Explanation: The table above shows the number of full water bottles, empty water bottles, the value of numExchange, and the number of bottles drunk.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= numBottles <= 100 `
/// * `1 <= numExchange <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/water-bottles-ii/
// discuss: https://leetcode.com/problems/water-bottles-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut result = num_bottles;

        while num_bottles >= num_exchange {
            result += 1;
            num_bottles = num_bottles - num_exchange + 1;
            num_exchange += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3100() {
        let num_bottles = 13;
        let num_exchange = 6;
        let expected = 15;
        assert_eq!(
            Solution::max_bottles_drunk(num_bottles, num_exchange),
            expected
        );
        let num_bottles = 10;
        let num_exchange = 3;
        let expected = 13;
        assert_eq!(
            Solution::max_bottles_drunk(num_bottles, num_exchange),
            expected
        );
    }
}
