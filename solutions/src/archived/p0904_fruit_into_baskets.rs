///
/// # 904. Fruit Into Baskets
///
/// You are visiting a farm that has a single row of fruit trees arranged from left to right. The trees are represented by an integer array `fruits` where `fruits[i]` is the **type** of fruit the `i<sup>th</sup>` tree produces.
///
/// You want to collect as much fruit as possible. However, the owner has some strict rules that you must follow:
///
/// * You only have **two** baskets, and each basket can only hold a **single type** of fruit. There is no limit on the amount of fruit each basket can hold.
/// * Starting from any tree of your choice, you must pick **exactly one fruit** from **every** tree (including the start tree) while moving to the right. The picked fruits must fit in one of your baskets.
/// * Once you reach a tree with fruit that cannot fit in your baskets, you must stop.
///
/// Given the integer array `fruits`, return *the **maximum** number of fruits you can pick*.
///
/// **Example 1:**
///
/// ```
/// Input: fruits = [1,2,1]
/// Output: 3
/// Explanation: We can pick from all 3 trees.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: fruits = [0,1,2,2]
/// Output: 3
/// Explanation: We can pick from trees [1,2,2].
/// If we had started at the first tree, we would only pick from trees [0,1].
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: fruits = [1,2,3,2,2]
/// Output: 4
/// Explanation: We can pick from trees [2,3,2,2].
/// If we had started at the first tree, we would only pick from trees [1,2].
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= fruits.length <= 10<sup>5</sup>`
/// * `0 <= fruits[i] < fruits.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/fruit-into-baskets/
// discuss: https://leetcode.com/problems/fruit-into-baskets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut cur = 0;
        let mut left = 0;
        let mut fruits_count = vec![0; 100001];
        let mut unique_fruits = 0;

        for &right_fruit in &fruits {
            let right_fruit = right_fruit as usize;

            if fruits_count[right_fruit] == 0 {
                unique_fruits += 1;
            }

            fruits_count[right_fruit] += 1;
            cur += 1;

            while unique_fruits > 2 {
                let left_fruit = fruits[left] as usize;

                fruits_count[left_fruit] -= 1;
                cur -= 1;

                if fruits_count[left_fruit] == 0 {
                    unique_fruits -= 1;
                }

                left += 1;
            }

            result = result.max(cur);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_904() {
        let fruits = vec![1, 2, 1];
        let expected = 3;
        assert_eq!(Solution::total_fruit(fruits), expected);
        let fruits = vec![0, 1, 2, 2];
        let expected = 3;
        assert_eq!(Solution::total_fruit(fruits), expected);
        let fruits = vec![1, 2, 3, 2, 2];
        let expected = 4;
        assert_eq!(Solution::total_fruit(fruits), expected);
    }
}
