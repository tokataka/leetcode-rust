///
/// # 2106. Maximum Fruits Harvested After at Most K Steps
///
/// Fruits are available at some positions on an infinite x-axis. You are given a 2D integer array `fruits` where `fruits[i] = [position<sub>i</sub>, amount<sub>i</sub>]` depicts `amount<sub>i</sub>` fruits at the position `position<sub>i</sub>`. `fruits` is already **sorted** by `position<sub>i</sub>` in **ascending order**, and each `position<sub>i</sub>` is **unique**.
///
/// You are also given an integer `startPos` and an integer `k`. Initially, you are at the position `startPos`. From any position, you can either walk to the **left or right**. It takes **one step** to move **one unit** on the x-axis, and you can walk **at most** `k` steps in total. For every position you reach, you harvest all the fruits at that position, and the fruits will disappear from that position.
///
/// Return *the **maximum total number** of fruits you can harvest*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/11/21/1.png)
///
/// ```
/// Input: fruits = [[2,8],[6,3],[8,6]], startPos = 5, k = 4
/// Output: 9
/// Explanation:
/// The optimal way is to:
/// - Move right to position 6 and harvest 3 fruits
/// - Move right to position 8 and harvest 6 fruits
/// You moved 3 steps and harvested 3 + 6 = 9 fruits in total.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/11/21/2.png)
///
/// ```
/// Input: fruits = [[0,9],[4,1],[5,7],[6,2],[7,4],[10,9]], startPos = 5, k = 4
/// Output: 14
/// Explanation:
/// You can move at most k = 4 steps, so you cannot reach position 0 nor 10.
/// The optimal way is to:
/// - Harvest the 7 fruits at the starting position 5
/// - Move left to position 4 and harvest 1 fruit
/// - Move right to position 6 and harvest 2 fruits
/// - Move right to position 7 and harvest 4 fruits
/// You moved 1 + 3 = 4 steps and harvested 7 + 1 + 2 + 4 = 14 fruits in total.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/11/21/3.png)
///
/// ```
/// Input: fruits = [[0,3],[6,4],[8,5]], startPos = 3, k = 2
/// Output: 0
/// Explanation:
/// You can move at most k = 2 steps and cannot reach any position with fruits.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= fruits.length <= 10<sup>5</sup>`
/// * `fruits[i].length == 2`
/// * `0 <= startPos, position<sub>i</sub> <= 2 * 10<sup>5</sup>`
/// * `position<sub>i-1</sub> < position<sub>i</sub>` for any `i > 0` (**0-indexed**)
/// * `1 <= amount<sub>i</sub> <= 10<sup>4</sup>`
/// * `0 <= k <= 2 * 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps/
// discuss: https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut result = 0;

        let mut prefix_sum = vec![];
        let mut cur_sum = 0;

        for fruit in &fruits {
            let (pos, amount) = (fruit[0], fruit[1]);
            prefix_sum.push((pos, cur_sum, cur_sum + amount));
            cur_sum += amount;
        }

        for &(left_pos, left_amount, _) in &prefix_sum {
            if left_pos < start_pos - k || left_pos > start_pos {
                continue;
            }

            let max_right_pos = start_pos + (k + left_pos - start_pos) / 2;
            let right_idx = prefix_sum.partition_point(|&x| x.0 <= max_right_pos);
            let right_amount = prefix_sum[right_idx - 1].2;

            result = result.max(right_amount - left_amount);
        }

        for &(right_pos, _, right_amount) in prefix_sum.iter().rev() {
            if right_pos > start_pos + k || right_pos < start_pos {
                continue;
            }

            let max_left_pos = start_pos - (k + start_pos - right_pos) / 2;
            let left_idx = prefix_sum.partition_point(|&x| x.0 < max_left_pos);
            let left_amount = prefix_sum[left_idx].1;

            result = result.max(right_amount - left_amount);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2106() {
        let fruits = nd_vec![[2, 8], [6, 3], [8, 6]];
        let start_pos = 5;
        let k = 4;
        let expected = 9;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), expected);
        let fruits = nd_vec![[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]];
        let start_pos = 5;
        let k = 4;
        let expected = 14;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), expected);
        let fruits = nd_vec![[0, 3], [6, 4], [8, 5]];
        let start_pos = 3;
        let k = 2;
        let expected = 0;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), expected);
        let fruits = nd_vec![[0, 10000]];
        let start_pos = 200000;
        let k = 200000;
        let expected = 10000;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), expected);
    }
}
