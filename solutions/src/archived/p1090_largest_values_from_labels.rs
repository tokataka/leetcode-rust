use std::collections::HashMap;

///
/// # 1090. Largest Values From Labels
///
/// You are given `n` item's value and label as two integer arrays `values` and `labels`. You are also given two integers `numWanted` and `useLimit`.
///
/// Your task is to find a subset of items with the **maximum sum** of their values such that:
///
/// * The number of items is **at most** `numWanted`.
/// * The number of items with the same label is **at most** `useLimit`.
///
/// Return the maximum sum.
///
/// **Example 1:**
///
/// **Input:** values = [5,4,3,2,1], labels = [1,1,2,2,3], numWanted = 3, useLimit = 1
///
/// **Output:** 9
///
/// **Explanation:**
///
/// The subset chosen is the first, third, and fifth items with the sum of values 5 + 3 + 1.
///
/// **Example 2:**
///
/// **Input:** values = [5,4,3,2,1], labels = [1,3,3,3,2], numWanted = 3, useLimit = 2
///
/// **Output:** 12
///
/// **Explanation:**
///
/// The subset chosen is the first, second, and third items with the sum of values 5 + 4 + 3.
///
/// **Example 3:**
///
/// **Input:** values = [9,8,8,7,6], labels = [0,0,0,1,1], numWanted = 3, useLimit = 1
///
/// **Output:** 16
///
/// **Explanation:**
///
/// The subset chosen is the first and fourth items with the sum of values 9 + 7.
///
/// **Constraints:**
///
/// * `n == values.length == labels.length`
/// * `1 <= n <= 2 * 10<sup>4</sup>`
/// * `0 <= values[i], labels[i] <= 2 * 10<sup>4</sup>`
/// * `1 <= numWanted, useLimit <= n`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-values-from-labels/
// discuss: https://leetcode.com/problems/largest-values-from-labels/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut value_labels = values.into_iter().zip(labels).collect::<Vec<_>>();
        value_labels.sort_unstable_by(|a, b| b.cmp(a));

        let mut result = 0;
        let mut label_count = HashMap::new();
        let mut total_count = 0;

        for (value, label) in value_labels {
            let count = label_count.entry(label).or_insert(0);
            if *count == use_limit {
                continue;
            }

            *count += 1;
            total_count += 1;
            result += value;

            if total_count == num_wanted {
                break;
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
    fn test_1090() {
        let values = vec![5, 4, 3, 2, 1];
        let labels = vec![1, 1, 2, 2, 3];
        let num_wanted = 3;
        let use_limit = 1;
        let expected = 9;
        assert_eq!(
            Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
            expected
        );
        let values = vec![5, 4, 3, 2, 1];
        let labels = vec![1, 3, 3, 3, 2];
        let num_wanted = 3;
        let use_limit = 2;
        let expected = 12;
        assert_eq!(
            Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
            expected
        );
        let values = vec![9, 8, 8, 7, 6];
        let labels = vec![0, 0, 0, 1, 1];
        let num_wanted = 3;
        let use_limit = 1;
        let expected = 16;
        assert_eq!(
            Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
            expected
        );
    }
}
