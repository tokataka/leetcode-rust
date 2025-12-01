///
/// # 3321. Find X-Sum of All K-Long Subarrays II
///
/// You are given an array `nums` of `n` integers and two integers `k` and `x`.
///
/// The **x-sum** of an array is calculated by the following procedure:
///
/// * Count the occurrences of all elements in the array.
/// * Keep only the occurrences of the top `x` most frequent elements. If two elements have the same number of occurrences, the element with the **bigger** value is considered more frequent.
/// * Calculate the sum of the resulting array.
///
/// **Note** that if an array has less than `x` distinct elements, its **x-sum** is the sum of the array.
///
/// Return an integer array `answer` of length `n - k + 1` where `answer[i]` is the **x-sum** of the subarray `nums[i..i + k - 1]`.
///
/// **Example 1:**
///
/// **Input:** nums = [1,1,2,2,3,4,2,3], k = 6, x = 2
///
/// **Output:** [6,10,12]
///
/// **Explanation:**
///
/// * For subarray `[1, 1, 2, 2, 3, 4]`, only elements 1 and 2 will be kept in the resulting array. Hence, `answer[0] = 1 + 1 + 2 + 2`.
/// * For subarray `[1, 2, 2, 3, 4, 2]`, only elements 2 and 4 will be kept in the resulting array. Hence, `answer[1] = 2 + 2 + 2 + 4`. Note that 4 is kept in the array since it is bigger than 3 and 1 which occur the same number of times.
/// * For subarray `[2, 2, 3, 4, 2, 3]`, only elements 2 and 3 are kept in the resulting array. Hence, `answer[2] = 2 + 2 + 2 + 3 + 3`.
///
/// **Example 2:**
///
/// **Input:** nums = [3,8,7,8,7,5], k = 2, x = 2
///
/// **Output:** [11,15,15,15,12]
///
/// **Explanation:**
///
/// Since `k == x`, `answer[i]` is equal to the sum of the subarray `nums[i..i + k - 1]`.
///
/// **Constraints:**
///
/// * `nums.length == n`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `1 <= x <= k <= nums.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-ii/
// discuss: https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let k = k as usize;
        let x = x as usize;

        let mut freq: HashMap<i64, i64> = HashMap::new();
        let mut upper = BTreeSet::new();
        let mut lower = BTreeSet::new();
        let mut sum = 0;

        for &num in nums.iter().take(k - 1) {
            let num = num as i64;
            let f = freq.entry(num).or_default();

            if upper.len() < x || upper.contains(&(*f, num)) {
                upper.remove(&(*f, num));
                *f += 1;
                upper.insert((*f, num));

                sum += num;
            } else {
                lower.remove(&(*f, num));
                *f += 1;
                lower.insert((*f, num));

                if lower.last() > upper.first() {
                    let (u, l) = (upper.pop_first().unwrap(), lower.pop_last().unwrap());

                    upper.insert(l);
                    lower.insert(u);

                    sum += l.0 * l.1 - u.0 * u.1;
                }
            }
        }

        let mut result = vec![];

        for (&cur, &prev) in nums.iter().skip(k - 1).zip(&nums) {
            let cur = cur as i64;
            let cur_f = freq.entry(cur).or_default();

            if upper.len() < x || upper.contains(&(*cur_f, cur)) {
                upper.remove(&(*cur_f, cur));
                *cur_f += 1;
                upper.insert((*cur_f, cur));

                sum += cur;
            } else {
                lower.remove(&(*cur_f, cur));
                *cur_f += 1;
                lower.insert((*cur_f, cur));

                if lower.last() > upper.first() {
                    let (u, l) = (upper.pop_first().unwrap(), lower.pop_last().unwrap());

                    upper.insert(l);
                    lower.insert(u);

                    sum += l.0 * l.1 - u.0 * u.1;
                }
            }

            result.push(sum);

            let prev = prev as i64;
            let prev_f = freq.entry(prev).or_default();

            if lower.contains(&(*prev_f, prev)) {
                lower.remove(&(*prev_f, prev));
                *prev_f -= 1;
                lower.insert((*prev_f, prev));
            } else {
                upper.remove(&(*prev_f, prev));
                *prev_f -= 1;
                upper.insert((*prev_f, prev));

                sum -= prev;

                if lower.last() > upper.first() {
                    let (u, l) = (upper.pop_first().unwrap(), lower.pop_last().unwrap());

                    upper.insert(l);
                    lower.insert(u);

                    sum += l.0 * l.1 - u.0 * u.1;
                }
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
    fn test_3321() {
        let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
        let k = 6;
        let x = 2;
        let expected = vec![6, 10, 12];
        assert_eq!(Solution::find_x_sum(nums, k, x), expected);
        let nums = vec![3, 8, 7, 8, 7, 5];
        let k = 2;
        let x = 2;
        let expected = vec![11, 15, 15, 15, 12];
        assert_eq!(Solution::find_x_sum(nums, k, x), expected);
    }
}
