///
/// # 3479. Fruits Into Baskets III
///
/// You are given two arrays of integers, `fruits` and `baskets`, each of length `n`, where `fruits[i]` represents the **quantity** of the `i<sup>th</sup>` type of fruit, and `baskets[j]` represents the **capacity** of the `j<sup>th</sup>` basket.
///
/// From left to right, place the fruits according to these rules:
///
/// * Each fruit type must be placed in the **leftmost available basket** with a capacity **greater than or equal** to the quantity of that fruit type.
/// * Each basket can hold **only one** type of fruit.
/// * If a fruit type **cannot be placed** in any basket, it remains **unplaced**.
///
/// Return the number of fruit types that remain unplaced after all possible allocations are made.
///
/// **Example 1:**
///
/// **Input:** fruits = [4,2,5], baskets = [3,5,4]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// * `fruits[0] = 4` is placed in `baskets[1] = 5`.
/// * `fruits[1] = 2` is placed in `baskets[0] = 3`.
/// * `fruits[2] = 5` cannot be placed in `baskets[2] = 4`.
///
/// Since one fruit type remains unplaced, we return 1.
///
/// **Example 2:**
///
/// **Input:** fruits = [3,6,1], baskets = [6,4,7]
///
/// **Output:** 0
///
/// **Explanation:**
///
/// * `fruits[0] = 3` is placed in `baskets[0] = 6`.
/// * `fruits[1] = 6` cannot be placed in `baskets[1] = 4` (insufficient capacity) but can be placed in the next available basket, `baskets[2] = 7`.
/// * `fruits[2] = 1` is placed in `baskets[1] = 4`.
///
/// Since all fruits are successfully placed, we return 0.
///
/// **Constraints:**
///
/// * `n == fruits.length == baskets.length`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `1 <= fruits[i], baskets[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/fruits-into-baskets-iii/
// discuss: https://leetcode.com/problems/fruits-into-baskets-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

struct SegTree {
    data: Vec<Option<i32>>,
}

impl SegTree {
    fn build(baskets: &[i32]) -> Self {
        let mut seg_tree = SegTree {
            data: vec![None; baskets.len() * 4],
        };

        seg_tree._build(1, 0, baskets.len() - 1, baskets);

        seg_tree
    }

    fn _build(&mut self, node: usize, left: usize, right: usize, baskets: &[i32]) {
        if left == right {
            self.data[node] = Some(baskets[left]);
            return;
        }

        let mid = (left + right) / 2;

        self._build(2 * node, left, mid, baskets);
        self._build(2 * node + 1, mid + 1, right, baskets);

        self.data[node] = self.data[2 * node].max(self.data[2 * node + 1]);
    }

    fn query_update(&mut self, node: usize, left: usize, right: usize, fruit: i32) -> bool {
        if self.data[node].is_none_or(|x| x < fruit) {
            return false;
        }

        if left == right {
            self.data[node] = None;
            return true;
        }

        let mid = (left + right) / 2;

        let res = if self.data[2 * node].is_some_and(|x| x >= fruit) {
            self.query_update(2 * node, left, mid, fruit)
        } else {
            self.query_update(2 * node + 1, mid + 1, right, fruit)
        };

        self.data[node] = self.data[2 * node].max(self.data[2 * node + 1]);

        res
    }
}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = baskets.len();

        let mut seg_tree = SegTree::build(&baskets);
        let mut result = n as i32;

        for fruit in fruits {
            if seg_tree.query_update(1, 0, n - 1, fruit) {
                result -= 1;
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
    fn test_3479() {
        let fruits = vec![4, 2, 5];
        let baskets = vec![3, 5, 4];
        let expected = 1;
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), expected);
        let fruits = vec![3, 6, 1];
        let baskets = vec![6, 4, 7];
        let expected = 0;
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), expected);
    }
}
