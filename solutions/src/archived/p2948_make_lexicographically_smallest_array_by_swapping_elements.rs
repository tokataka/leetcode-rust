///
/// # 2948. Make Lexicographically Smallest Array by Swapping Elements
///
/// You are given a **0-indexed** array of **positive** integers `nums` and a **positive** integer `limit`.
///
/// In one operation, you can choose any two indices `i` and `j` and swap `nums[i]` and `nums[j]` **if** `|nums[i] - nums[j]| <= limit`.
///
/// Return *the **lexicographically smallest array** that can be obtained by performing the operation any number of times*.
///
/// An array `a` is lexicographically smaller than an array `b` if in the first position where `a` and `b` differ, array `a` has an element that is less than the corresponding element in `b`. For example, the array `[2,10,3]` is lexicographically smaller than the array `[10,2,3]` because they differ at index `0` and `2 < 10`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,5,3,9,8], limit = 2
/// Output: [1,3,5,8,9]
/// Explanation: Apply the operation 2 times:
/// - Swap nums[1] with nums[2]. The array becomes [1,3,5,9,8]
/// - Swap nums[3] with nums[4]. The array becomes [1,3,5,8,9]
/// We cannot obtain a lexicographically smaller array by applying any more operations.
/// Note that it may be possible to get the same result by doing different operations.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,7,6,18,2,1], limit = 3
/// Output: [1,6,7,18,1,2]
/// Explanation: Apply the operation 3 times:
/// - Swap nums[1] with nums[2]. The array becomes [1,6,7,18,2,1]
/// - Swap nums[0] with nums[4]. The array becomes [2,6,7,18,1,1]
/// - Swap nums[0] with nums[5]. The array becomes [1,6,7,18,1,2]
/// We cannot obtain a lexicographically smaller array by applying any more operations.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [1,7,28,19,10], limit = 3
/// Output: [1,7,28,19,10]
/// Explanation: [1,7,28,19,10] is the lexicographically smallest array we can obtain because we cannot apply the operation on any two indices.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
/// * `1 <= limit <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/
// discuss: https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut nums_with_idx = nums
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect::<Vec<_>>();

        nums_with_idx.sort_unstable();

        let (v, i) = nums_with_idx[0];

        let mut indexs = vec![i];
        let mut values = vec![v];
        let mut min = v;
        let mut max = v;

        let mut result = vec![0; nums.len()];

        for &(v, i) in nums_with_idx
            .iter()
            .skip(1)
            .chain(std::iter::once(&(i32::MAX, usize::MAX)))
        {
            if v >= min - limit && v <= max + limit {
                indexs.push(i);
                values.push(v);
                min = min.min(v);
                max = max.max(v);
            } else {
                indexs.sort_unstable();

                for (ii, vv) in indexs.into_iter().zip(values) {
                    result[ii] = vv;
                }

                indexs = vec![i];
                values = vec![v];
                min = v;
                max = v;
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
    fn test_2948() {
        let nums = vec![1, 5, 3, 9, 8];
        let limit = 2;
        let expected = vec![1, 3, 5, 8, 9];
        assert_eq!(
            Solution::lexicographically_smallest_array(nums, limit),
            expected
        );
        let nums = vec![1, 7, 6, 18, 2, 1];
        let limit = 3;
        let expected = vec![1, 6, 7, 18, 1, 2];
        assert_eq!(
            Solution::lexicographically_smallest_array(nums, limit),
            expected
        );
        let nums = vec![1, 7, 28, 19, 10];
        let limit = 3;
        let expected = vec![1, 7, 28, 19, 10];
        assert_eq!(
            Solution::lexicographically_smallest_array(nums, limit),
            expected
        );
    }
}
