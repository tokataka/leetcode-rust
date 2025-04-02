use std::collections::HashMap;

///
/// # 2122. Recover the Original Array
///
/// Alice had a **0-indexed** array `arr` consisting of `n` **positive** integers. She chose an arbitrary **positive integer** `k` and created two new **0-indexed** integer arrays `lower` and `higher` in the following manner:
///
/// 1. `lower[i] = arr[i] - k`, for every index `i` where `0 <= i < n`
/// 2. `higher[i] = arr[i] + k`, for every index `i` where `0 <= i < n`
///
/// Unfortunately, Alice lost all three arrays. However, she remembers the integers that were present in the arrays `lower` and `higher`, but not the array each integer belonged to. Help Alice and recover the original array.
///
/// Given an array `nums` consisting of `2n` integers, where **exactly** `n` of the integers were present in `lower` and the remaining in `higher`, return *the **original** array* `arr`. In case the answer is not unique, return ***any** valid array*.
///
/// **Note:** The test cases are generated such that there exists **at least one** valid array `arr`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,10,6,4,8,12]
/// Output: [3,7,11]
/// Explanation:
/// If arr = [3,7,11] and k = 1, we get lower = [2,6,10] and higher = [4,8,12].
/// Combining lower and higher gives us [2,6,10,4,8,12], which is a permutation of nums.
/// Another valid possibility is that arr = [5,7,9] and k = 3. In that case, lower = [2,4,6] and higher = [8,10,12].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,1,3,3]
/// Output: [2,2]
/// Explanation:
/// If arr = [2,2] and k = 1, we get lower = [1,1] and higher = [3,3].
/// Combining lower and higher gives us [1,1,3,3], which is equal to nums.
/// Note that arr cannot be [1,3] because in that case, the only possible way to obtain [1,1,3,3] is with k = 0.
/// This is invalid since k must be positive.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [5,435]
/// Output: [220]
/// Explanation:
/// The only possible combination is arr = [220] and k = 215. Using them, we get lower = [5] and higher = [435].
///
/// ```
///
/// **Constraints:**
///
/// * `2 * n == nums.length`
/// * `1 <= n <= 1000`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * The test cases are generated such that there exists **at least one** valid array `arr`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/recover-the-original-array/
// discuss: https://leetcode.com/problems/recover-the-original-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn recover_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let nums_map = nums
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, &x)| {
                acc.entry(x).or_insert(i);
                acc
            });

        for right_start in 1..=nums.len() / 2 {
            let mut result = vec![];
            let mut visited = vec![false; nums.len()];

            let num_diff = nums[right_start] - nums[0];

            if num_diff == 0 || num_diff & 1 == 1 {
                continue;
            }

            let k = num_diff / 2;

            let mut left = 0;

            while left < nums.len() {
                while left < nums.len() && visited[left] {
                    left += 1;
                }

                if left >= nums.len() {
                    break;
                }

                let mut right = match nums_map.get(&(nums[left] + 2 * k)) {
                    Some(&right) => right,
                    None => break,
                };

                while right < nums.len() && visited[right] {
                    right += 1;
                }

                if right >= nums.len() || nums[right] != nums[left] + 2 * k {
                    break;
                }

                result.push(nums[left] + k);

                visited[left] = true;
                visited[right] = true;
            }

            if result.len() * 2 == nums.len() {
                return result;
            }
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2122() {
        let nums = vec![2, 10, 6, 4, 8, 12];
        let expected = vec![3, 7, 11];
        assert_eq!(Solution::recover_array(nums), expected);
        let nums = vec![1, 1, 3, 3];
        let expected = vec![2, 2];
        assert_eq!(Solution::recover_array(nums), expected);
        let nums = vec![5, 435];
        let expected = vec![220];
        assert_eq!(Solution::recover_array(nums), expected);
    }
}
