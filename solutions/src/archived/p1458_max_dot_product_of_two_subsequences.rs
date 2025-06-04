///
/// # 1458. Max Dot Product of Two Subsequences
///
/// Given two arrays `nums1` and `nums2`.
///
/// Return the maximum dot product between **non-empty** subsequences of nums1 and nums2 with the same length.
///
/// A subsequence of a array is a new array which is formed from the original array by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, `[2,3,5]` is a subsequence of `[1,2,3,4,5]` while `[1,5,3]` is not).
///
/// **Example 1:**
///
/// ```
/// Input: nums1 = [2,1,-2,5], nums2 = [3,0,-6]
/// Output: 18
/// Explanation: Take subsequence [2,-2] from nums1 and subsequence [3,-6] from nums2.
/// Their dot product is (2*3 + (-2)*(-6)) = 18.
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums1 = [3,-2], nums2 = [2,-6,7]
/// Output: 21
/// Explanation: Take subsequence [3] from nums1 and subsequence [7] from nums2.
/// Their dot product is (3*7) = 21.
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums1 = [-1,-1], nums2 = [1,1]
/// Output: -1
/// Explanation: Take subsequence [-1] from nums1 and subsequence [1] from nums2.
/// Their dot product is -1.
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums1.length, nums2.length <= 500`
/// * `-1000 <= nums1[i], nums2[i] <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/max-dot-product-of-two-subsequences/
// discuss: https://leetcode.com/problems/max-dot-product-of-two-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut cache = vec![vec![i32::MAX; nums2.len() + 1]; nums1.len() + 1];

        fn _max_dot_product(
            idx1: usize,
            idx2: usize,
            nums1: &[i32],
            nums2: &[i32],
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if idx1 >= nums1.len() || idx2 >= nums2.len() {
                return i32::MIN;
            }

            if cache[idx1][idx2] != i32::MAX {
                return cache[idx1][idx2];
            }

            let mut result = nums1[idx1] * nums2[idx2]
                + 0.max(_max_dot_product(idx1 + 1, idx2 + 1, nums1, nums2, cache));

            result = result
                .max(_max_dot_product(idx1 + 1, idx2, nums1, nums2, cache))
                .max(_max_dot_product(idx1, idx2 + 1, nums1, nums2, cache));

            cache[idx1][idx2] = result;

            result
        }

        _max_dot_product(0, 0, &nums1, &nums2, &mut cache)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1458() {
        // let nums1 = vec![2, 1, -2, 5];
        // let nums2 = vec![3, 0, -6];
        // let expected = 18;
        // assert_eq!(Solution::max_dot_product(nums1, nums2), expected);
        // let nums1 = vec![3, -2];
        // let nums2 = vec![2, -6, 7];
        // let expected = 21;
        // assert_eq!(Solution::max_dot_product(nums1, nums2), expected);
        // let nums1 = vec![-1, -1];
        // let nums2 = vec![1, 1];
        // let expected = -1;
        // assert_eq!(Solution::max_dot_product(nums1, nums2), expected);
        let nums1 = vec![-5, -1, -2];
        let nums2 = vec![3, 3, 5, 5];
        let expected = -3;
        assert_eq!(Solution::max_dot_product(nums1, nums2), expected);
    }
}
