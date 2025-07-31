///
/// # 1865. Finding Pairs With a Certain Sum
///
/// You are given two integer arrays `nums1` and `nums2`. You are tasked to implement a data structure that supports queries of two types:
///
/// 1. **Add** a positive integer to an element of a given index in the array `nums2`.
/// 2. **Count** the number of pairs `(i, j)` such that `nums1[i] + nums2[j]` equals a given value (`0 <= i < nums1.length` and `0 <= j < nums2.length`).
///
/// Implement the `FindSumPairs` class:
///
/// * `FindSumPairs(int[] nums1, int[] nums2)` Initializes the `FindSumPairs` object with two integer arrays `nums1` and `nums2`.
/// * `void add(int index, int val)` Adds `val` to `nums2[index]`, i.e., apply `nums2[index] += val`.
/// * `int count(int tot)` Returns the number of pairs `(i, j)` such that `nums1[i] + nums2[j] == tot`.
///
/// **Example 1:**
///
/// ```
/// Input
/// ["FindSumPairs", "count", "add", "count", "count", "add", "add", "count"]
/// [[[1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]], [7], [3, 2], [8], [4], [0, 1], [1, 1], [7]]
/// Output
/// [null, 8, null, 2, 1, null, null, 11]
///
/// Explanation
/// FindSumPairs findSumPairs = new FindSumPairs([1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]);
/// findSumPairs.count(7);  // return 8; pairs (2,2), (3,2), (4,2), (2,4), (3,4), (4,4) make 2 + 5 and pairs (5,1), (5,5) make 3 + 4
/// findSumPairs.add(3, 2); // now nums2 = [1,4,5,4,5,4]
/// findSumPairs.count(8);  // return 2; pairs (5,2), (5,4) make 3 + 5
/// findSumPairs.count(4);  // return 1; pair (5,0) makes 3 + 1
/// findSumPairs.add(0, 1); // now nums2 = [2,4,5,4,5,4]
/// findSumPairs.add(1, 1); // now nums2 = [2,5,5,4,5,4]
/// findSumPairs.count(7);  // return 11; pairs (2,1), (2,2), (2,4), (3,1), (3,2), (3,4), (4,1), (4,2), (4,4) make 2 + 5 and pairs (5,3), (5,5) make 3 + 4
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums1.length <= 1000`
/// * `1 <= nums2.length <= 10<sup>5</sup>`
/// * `1 <= nums1[i] <= 10<sup>9</sup>`
/// * `1 <= nums2[i] <= 10<sup>5</sup>`
/// * `0 <= index < nums2.length`
/// * `1 <= val <= 10<sup>5</sup>`
/// * `1 <= tot <= 10<sup>9</sup>`
/// * At most `1000` calls are made to `add` and `count` **each**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/finding-pairs-with-a-certain-sum/
// discuss: https://leetcode.com/problems/finding-pairs-with-a-certain-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

struct FindSumPairs {
    nums1_count: HashMap<i32, i32>,
    nums2: Vec<i32>,
    nums2_count: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut nums1_count = HashMap::new();
        let mut nums2_count = HashMap::new();

        for &num1 in &nums1 {
            *nums1_count.entry(num1).or_default() += 1;
        }

        for &num2 in &nums2 {
            *nums2_count.entry(num2).or_default() += 1;
        }

        Self {
            nums1_count,
            nums2,
            nums2_count,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        *self
            .nums2_count
            .get_mut(&self.nums2[index as usize])
            .unwrap() -= 1;

        self.nums2[index as usize] += val;

        *self
            .nums2_count
            .entry(self.nums2[index as usize])
            .or_default() += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut result = 0;

        for (&num1, &num1_count) in &self.nums1_count {
            if num1 >= tot {
                continue;
            }

            let num2 = tot - num1;
            let num2_count = self.nums2_count.get(&num2).unwrap_or(&0);

            result += num1_count * num2_count;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1865() {
        let mut obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        assert_eq!(obj.count(7), 8);
        obj.add(3, 2);
        assert_eq!(obj.count(8), 2);
        assert_eq!(obj.count(4), 1);
        obj.add(0, 1);
        obj.add(1, 1);
        assert_eq!(obj.count(7), 11);
    }
}
