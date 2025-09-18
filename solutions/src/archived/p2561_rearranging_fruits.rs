///
/// # 2561. Rearranging Fruits
///
/// You have two fruit baskets containing `n` fruits each. You are given two **0-indexed** integer arrays `basket1` and `basket2` representing the cost of fruit in each basket. You want to make both baskets **equal**. To do so, you can use the following operation as many times as you want:
///
/// * Chose two indices `i` and `j`, and swap the `ith `fruit of `basket1` with the `jth` fruit of `basket2`.
/// * The cost of the swap is `min(basket1[i],basket2[j])`.
///
/// Two baskets are considered equal if sorting them according to the fruit cost makes them exactly the same baskets.
///
/// Return *the minimum cost to make both the baskets equal or* `-1` *if impossible.*
///
/// **Example 1:**
///
/// ```
/// Input: basket1 = [4,2,2,2], basket2 = [1,4,1,2]
/// Output: 1
/// Explanation: Swap index 1 of basket1 with index 0 of basket2, which has cost 1. Now basket1 = [4,1,2,2] and basket2 = [2,4,1,2]. Rearranging both the arrays makes them equal.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: basket1 = [2,3,4,1], basket2 = [3,2,5,1]
/// Output: -1
/// Explanation: It can be shown that it is impossible to make both the baskets equal.
///
/// ```
///
/// **Constraints:**
///
/// * `basket1.length == basket2.length`
/// * `1 <= basket1.length <= 10<sup>5</sup>`
/// * `1 <= basket1[i],basket2[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/rearranging-fruits/
// discuss: https://leetcode.com/problems/rearranging-fruits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{cmp::Reverse, collections::HashMap};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut diff_map: HashMap<i64, i64> = HashMap::new();
        let mut min_fruit = i64::MAX;

        for x in basket1 {
            let x = x as i64;

            *diff_map.entry(x).or_default() += 1;
            min_fruit = min_fruit.min(x);
        }

        for x in basket2 {
            let x = x as i64;

            *diff_map.entry(x).or_default() -= 1;
            min_fruit = min_fruit.min(x);
        }

        let mut plus = vec![];
        let mut minus = vec![];

        for (value, count) in diff_map.into_iter().filter(|x| x.1 != 0) {
            if count % 2 == 1 {
                return -1;
            }

            if count > 0 {
                plus.push((value, count / 2));
            } else {
                minus.push((value, -count / 2));
            }
        }

        let mut result = 0;

        plus.sort_unstable();
        minus.sort_unstable_by_key(|&x| Reverse(x.0));

        while let (Some(&(p_value, p_count)), Some(&(m_value, m_count))) =
            (plus.last(), minus.last())
        {
            let value = p_value.min(m_value);
            let count = p_count.min(m_count);

            result += value.min(min_fruit * 2) * count;

            if p_count > count {
                plus.last_mut().unwrap().1 -= count;
            } else {
                plus.pop();
            }

            if m_count > count {
                minus.last_mut().unwrap().1 -= count;
            } else {
                minus.pop();
            }
        }

        if !plus.is_empty() || !minus.is_empty() {
            return -1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2561() {
        // let basket1 = vec![4, 2, 2, 2];
        // let basket2 = vec![1, 4, 1, 2];
        // let expected = 1;
        // assert_eq!(Solution::min_cost(basket1, basket2), expected);
        // let basket1 = vec![2, 3, 4, 1];
        // let basket2 = vec![3, 2, 5, 1];
        // let expected = -1;
        // assert_eq!(Solution::min_cost(basket1, basket2), expected);
        let basket1 = vec![84, 80, 43, 8, 80, 88, 43, 14, 100, 88];
        let basket2 = vec![32, 32, 42, 68, 68, 100, 42, 84, 14, 8];
        let expected = 48;
        assert_eq!(Solution::min_cost(basket1, basket2), expected);
    }
}
