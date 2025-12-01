///
/// # 1578. Minimum Time to Make Rope Colorful
///
/// Alice has `n` balloons arranged on a rope. You are given a **0-indexed** string `colors` where `colors[i]` is the color of the `i<sup>th</sup>` balloon.
///
/// Alice wants the rope to be **colorful**. She does not want **two consecutive balloons** to be of the same color, so she asks Bob for help. Bob can remove some balloons from the rope to make it **colorful**. You are given a **0-indexed** integer array `neededTime` where `neededTime[i]` is the time (in seconds) that Bob needs to remove the `i<sup>th</sup>` balloon from the rope.
///
/// Return *the **minimum time** Bob needs to make the rope **colorful***.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/12/13/ballon1.jpg)
///
/// ```
/// Input: colors = "abaac", neededTime = [1,2,3,4,5]
/// Output: 3
/// Explanation: In the above image, 'a' is blue, 'b' is red, and 'c' is green.
/// Bob can remove the blue balloon at index 2. This takes 3 seconds.
/// There are no longer two consecutive balloons of the same color. Total time = 3.
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/12/13/balloon2.jpg)
///
/// ```
/// Input: colors = "abc", neededTime = [1,2,3]
/// Output: 0
/// Explanation: The rope is already colorful. Bob does not need to remove any balloons from the rope.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2021/12/13/balloon3.jpg)
///
/// ```
/// Input: colors = "aabaa", neededTime = [1,2,3,4,1]
/// Output: 2
/// Explanation: Bob will remove the balloons at indices 0 and 4. Each balloons takes 1 second to remove.
/// There are no longer two consecutive balloons of the same color. Total time = 1 + 1 = 2.
///
/// ```
///
/// **Constraints:**
///
/// * `n == colors.length == neededTime.length`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `1 <= neededTime[i] <= 10<sup>4</sup>`
/// * `colors` contains only lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-to-make-rope-colorful/
// discuss: https://leetcode.com/problems/minimum-time-to-make-rope-colorful/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        colors
            .bytes()
            .zip(needed_time)
            .chunk_by(|&(color, _)| color)
            .into_iter()
            .map(|(_, chunk)| {
                chunk.fold((0, 0), |(sum, max), (_, time)| (sum + time, max.max(time)))
            })
            .map(|(sum, max)| sum - max)
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1578() {
        let colors = "abaac".to_owned();
        let needed_time = vec![1, 2, 3, 4, 5];
        let expected = 3;
        assert_eq!(Solution::min_cost(colors, needed_time), expected);
        let colors = "abc".to_owned();
        let needed_time = vec![1, 2, 3];
        let expected = 0;
        assert_eq!(Solution::min_cost(colors, needed_time), expected);
        let colors = "aabaa".to_owned();
        let needed_time = vec![1, 2, 3, 4, 1];
        let expected = 2;
        assert_eq!(Solution::min_cost(colors, needed_time), expected);
    }
}
