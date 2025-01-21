use std::collections::VecDeque;

///
/// # 220. Contains Duplicate III
///
/// You are given an integer array `nums` and two integers `indexDiff` and `valueDiff`.
///
/// Find a pair of indices `(i, j)` such that:
///
/// * `i != j`,
/// * `abs(i - j) <= indexDiff`.
/// * `abs(nums[i] - nums[j]) <= valueDiff`, and
///
/// Return `true` *if such pair exists or* `false` *otherwise*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,3,1], indexDiff = 3, valueDiff = 0
/// Output: true
/// Explanation: We can choose (i, j) = (0, 3).
/// We satisfy the three conditions:
/// i != j --> 0 != 3
/// abs(i - j) <= indexDiff --> abs(0 - 3) <= 3
/// abs(nums[i] - nums[j]) <= valueDiff --> abs(1 - 1) <= 0
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,5,9,1,5,9], indexDiff = 2, valueDiff = 3
/// Output: false
/// Explanation: After trying all the possible pairs (i, j), we cannot satisfy the three conditions, so we return false.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= nums.length <= 10<sup>5</sup>`
/// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
/// * `1 <= indexDiff <= nums.length`
/// * `0 <= valueDiff <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate-iii/
// discuss: https://leetcode.com/problems/contains-duplicate-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let mut q = VecDeque::new();
        let mut sorted = vec![];

        for &num in nums.iter().take(index_diff as usize + 1) {
            q.push_back(num);
            sorted.push(num);
        }

        sorted.sort_unstable();

        if sorted.windows(2).any(|x| x[1] - x[0] <= value_diff) {
            return true;
        }

        for &num in nums.iter().skip(index_diff as usize + 1) {
            let pop_num = q.pop_front().unwrap();
            q.push_back(num);

            sorted.remove(sorted.binary_search(&pop_num).unwrap());

            let insert_idx = sorted.partition_point(|&x| x < num);
            sorted.insert(insert_idx, num);

            if insert_idx > 0 && sorted[insert_idx] - sorted[insert_idx - 1] <= value_diff
                || insert_idx < sorted.len() - 1
                    && sorted[insert_idx + 1] - sorted[insert_idx] <= value_diff
            {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_220() {
        let nums = vec![1, 2, 3, 1];
        let index_diff = 3;
        let value_diff = 0;
        let expected = true;
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, index_diff, value_diff),
            expected
        );
        let nums = vec![1, 5, 9, 1, 5, 9];
        let index_diff = 2;
        let value_diff = 3;
        let expected = false;
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, index_diff, value_diff),
            expected
        );
    }
}
