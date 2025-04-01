///
/// # 2551. Put Marbles in Bags
///
/// You have `k` bags. You are given a **0-indexed** integer array `weights` where `weights[i]` is the weight of the `i<sup>th</sup>` marble. You are also given the integer `k.`
///
/// Divide the marbles into the `k` bags according to the following rules:
///
/// * No bag is empty.
/// * If the `i<sup>th</sup>` marble and `j<sup>th</sup>` marble are in a bag, then all marbles with an index between the `i<sup>th</sup>` and `j<sup>th</sup>` indices should also be in that same bag.
/// * If a bag consists of all the marbles with an index from `i` to `j` inclusively, then the cost of the bag is `weights[i] + weights[j]`.
///
/// The **score** after distributing the marbles is the sum of the costs of all the `k` bags.
///
/// Return *the **difference** between the **maximum** and **minimum** scores among marble distributions*.
///
/// **Example 1:**
///
/// ```
/// Input: weights = [1,3,5,1], k = 2
/// Output: 4
/// Explanation:
/// The distribution [1],[3,5,1] results in the minimal score of (1+1) + (3+1) = 6.
/// The distribution [1,3],[5,1], results in the maximal score of (1+3) + (5+1) = 10.
/// Thus, we return their difference 10 - 6 = 4.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: weights = [1, 3], k = 2
/// Output: 0
/// Explanation: The only distribution possible is [1],[3].
/// Since both the maximal and minimal score are the same, we return 0.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= k <= weights.length <= 10<sup>5</sup>`
/// * `1 <= weights[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/put-marbles-in-bags/
// discuss: https://leetcode.com/problems/put-marbles-in-bags/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        if k == 1 {
            return 0;
        }

        let mut borders = weights
            .windows(2)
            .map(|x| (x[0] + x[1]) as i64)
            .collect::<Vec<_>>();

        let t = (k as usize - 1).min(borders.len() - (k as usize - 1));

        let min = borders.select_nth_unstable(t).0.iter().sum::<i64>();
        let max = borders
            .select_nth_unstable_by(t, |a, b| b.cmp(a))
            .0
            .iter()
            .sum::<i64>();

        max - min
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2551() {
        let weights = vec![1, 4, 2, 5, 2];
        let k = 3;
        let expected = 3;
        assert_eq!(Solution::put_marbles(weights, k), expected);
        // let weights = vec![1, 3, 5, 1];
        // let k = 2;
        // let expected = 4;
        // assert_eq!(Solution::put_marbles(weights, k), expected);
        // let weights = vec![1, 3];
        // let k = 2;
        // let expected = 0;
        // assert_eq!(Solution::put_marbles(weights, k), expected);
    }
}
