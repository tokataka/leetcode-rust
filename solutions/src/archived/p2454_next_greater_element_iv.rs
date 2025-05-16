///
/// # 2454. Next Greater Element IV
///
/// You are given a **0-indexed** array of non-negative integers `nums`. For each integer in `nums`, you must find its respective **second greater** integer.
///
/// The **second greater** integer of `nums[i]` is `nums[j]` such that:
///
/// * `j > i`
/// * `nums[j] > nums[i]`
/// * There exists **exactly one** index `k` such that `nums[k] > nums[i]` and `i < k < j`.
///
/// If there is no such `nums[j]`, the second greater integer is considered to be `-1`.
///
/// * For example, in the array `[1, 2, 4, 3]`, the second greater integer of `1` is `4`, `2` is `3`, and that of `3` and `4` is `-1`.
///
/// Return *an integer array* `answer`*, where* `answer[i]` *is the second greater integer of* `nums[i]`*.*
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,4,0,9,6]
/// Output: [9,6,6,-1,-1]
/// Explanation:
/// 0th index: 4 is the first integer greater than 2, and 9 is the second integer greater than 2, to the right of 2.
/// 1st index: 9 is the first, and 6 is the second integer greater than 4, to the right of 4.
/// 2nd index: 9 is the first, and 6 is the second integer greater than 0, to the right of 0.
/// 3rd index: There is no integer greater than 9 to its right, so the second greater integer is considered to be -1.
/// 4th index: There is no integer greater than 6 to its right, so the second greater integer is considered to be -1.
/// Thus, we return [9,6,6,-1,-1].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [3,3]
/// Output: [-1,-1]
/// Explanation:
/// We return [-1,-1] since neither integer has any integer greater than it.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `0 <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/next-greater-element-iv/
// discuss: https://leetcode.com/problems/next-greater-element-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut monotonic = vec![];
        let mut fallback = vec![];

        let mut result = vec![];

        for (i, num) in nums.into_iter().enumerate().rev() {
            let mut temp = vec![];

            while let Some(&(_, x)) = monotonic.last() {
                if x > num {
                    break;
                }

                temp.push(monotonic.pop().unwrap());
            }

            if let Some(&(_, temp_last)) = temp.last() {
                while let Some(&(_, x)) = fallback.last() {
                    if x > temp_last {
                        break;
                    }

                    fallback.pop();
                }

                fallback.extend(temp.into_iter().rev());
            }

            let mut candidates = vec![];

            if let Some(&x) = monotonic.iter().rev().nth(1) {
                candidates.push(x);
            }

            let fallback_idx = fallback.partition_point(|&p| p.1 > num);
            if fallback_idx > 0 {
                candidates.push(fallback[fallback_idx - 1]);
            }

            candidates.sort();

            result.push(match candidates.first() {
                Some(&(_, x)) => x,
                None => -1,
            });

            monotonic.push((i, num));
        }

        result.reverse();

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2454() {
        let nums = vec![2, 4, 0, 9, 6];
        let expected = vec![9, 6, 6, -1, -1];
        assert_eq!(Solution::second_greater_element(nums), expected);
        let nums = vec![3, 3];
        let expected = vec![-1, -1];
        assert_eq!(Solution::second_greater_element(nums), expected);
    }
}
