///
/// # 4. Median of Two Sorted Arrays
///
/// Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return **the median** of the two sorted arrays.
///
/// The overall run time complexity should be `O(log (m+n))`.
///
/// **Example 1:**
///
/// ```
/// Input: nums1 = [1,3], nums2 = [2]
/// Output: 2.00000
/// Explanation: merged array = [1,2,3] and median is 2.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums1 = [1,2], nums2 = [3,4]
/// Output: 2.50000
/// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
///
/// ```
///
/// **Constraints:**
///
/// * `nums1.length == m`
/// * `nums2.length == n`
/// * `0 <= m <= 1000`
/// * `0 <= n <= 1000`
/// * `1 <= m + n <= 2000`
/// * `-10<sup>6</sup> <= nums1[i], nums2[i] <= 10<sup>6</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = match nums1.len() > nums2.len() {
            true => (nums2, nums1),
            false => (nums1, nums2),
        };

        let m = nums1.len();
        let n = nums2.len();

        let mut left = 0;
        let mut right = m;

        loop {
            let mid1 = (right + left) / 2;
            let mid2 = (m + n).div_ceil(2) - mid1;

            let max_left1 = mid1.checked_sub(1).map(|i| nums1[i]).unwrap_or(i32::MIN);
            let min_right1 = *nums1.get(mid1).unwrap_or(&i32::MAX);
            let max_left2 = mid2.checked_sub(1).map(|i| nums2[i]).unwrap_or(i32::MIN);
            let min_right2 = *nums2.get(mid2).unwrap_or(&i32::MAX);

            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                let mut result = max_left1.max(max_left2) as f64;

                if (m + n) % 2 == 0 {
                    result += min_right1.min(min_right2) as f64;
                    result /= 2.0;
                }

                return result;
            }

            if max_left1 > min_right2 {
                right = mid1 - 1;
            } else {
                left = mid1 + 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let expected = 2.00000;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let expected = 2.50000;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }
}
