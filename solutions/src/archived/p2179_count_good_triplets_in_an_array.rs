///
/// # 2179. Count Good Triplets in an Array
///
/// You are given two **0-indexed** arrays `nums1` and `nums2` of length `n`, both of which are **permutations** of `[0, 1, ..., n - 1]`.
///
/// A **good triplet** is a set of `3` **distinct** values which are present in **increasing order** by position both in `nums1` and `nums2`. In other words, if we consider `pos1<sub>v</sub>` as the index of the value `v` in `nums1` and `pos2<sub>v</sub>` as the index of the value `v` in `nums2`, then a good triplet will be a set `(x, y, z)` where `0 <= x, y, z <= n - 1`, such that `pos1<sub>x</sub> < pos1<sub>y</sub> < pos1<sub>z</sub>` and `pos2<sub>x</sub> < pos2<sub>y</sub> < pos2<sub>z</sub>`.
///
/// Return *the **total number** of good triplets*.
///
/// **Example 1:**
///
/// ```
/// Input: nums1 = [2,0,1,3], nums2 = [0,1,2,3]
/// Output: 1
/// Explanation:
/// There are 4 triplets (x,y,z) such that pos1x < pos1y < pos1z. They are (2,0,1), (2,0,3), (2,1,3), and (0,1,3).
/// Out of those triplets, only the triplet (0,1,3) satisfies pos2x < pos2y < pos2z. Hence, there is only 1 good triplet.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums1 = [4,0,1,3,2], nums2 = [4,1,0,2,3]
/// Output: 4
/// Explanation: The 4 good triplets are (4,0,3), (4,0,2), (4,1,3), and (4,1,2).
///
/// ```
///
/// **Constraints:**
///
/// * `n == nums1.length == nums2.length`
/// * `3 <= n <= 10<sup>5</sup>`
/// * `0 <= nums1[i], nums2[i] <= n - 1`
/// * `nums1` and `nums2` are permutations of `[0, 1, ..., n - 1]`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-good-triplets-in-an-array/
// discuss: https://leetcode.com/problems/count-good-triplets-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();

        let mut nums2_idx = vec![0; n];

        for (i, &x) in nums2.iter().enumerate() {
            nums2_idx[x as usize] = i;
        }

        let mut combined = nums1
            .iter()
            .map(|&x| nums2_idx[x as usize])
            .collect::<Vec<_>>();

        let mut lower_counts = vec![0; n];
        let mut upper_counts = vec![0; n];

        fn merge_sort(
            arr: &mut [usize],
            temp: &mut [usize],
            lower_counts: &mut [usize],
            upper_counts: &mut [usize],
            left: usize,
            right: usize,
        ) {
            let mid = (left + right) / 2;

            if mid - left >= 2 {
                merge_sort(arr, temp, lower_counts, upper_counts, left, mid);
            }
            if right - mid >= 2 {
                merge_sort(arr, temp, lower_counts, upper_counts, mid, right);
            }

            let n1 = mid - left;
            let n2 = right - mid;
            let mut idx1 = 0;
            let mut idx2 = 0;
            let mut idx_sorted = 0;

            while idx1 < n1 && idx2 < n2 {
                if arr[idx1 + left] > arr[idx2 + mid] {
                    lower_counts[arr[idx2 + mid]] += idx1;

                    temp[idx_sorted] = arr[idx2 + mid];
                    idx2 += 1;
                } else {
                    upper_counts[arr[idx1 + left]] += n2 - idx2;

                    temp[idx_sorted] = arr[idx1 + left];
                    idx1 += 1;
                }
                idx_sorted += 1;
            }

            if idx1 == n1 {
                for i in idx2 + mid..right {
                    lower_counts[arr[i]] += n1;
                }

                arr.copy_within(idx2 + mid..right, left + idx_sorted);
            } else if idx2 == n2 {
                arr.copy_within(idx1 + left..mid, left + idx_sorted);
            }

            arr[left..left + idx_sorted].copy_from_slice(&temp[..idx_sorted]);
        }

        merge_sort(
            &mut combined,
            &mut vec![0; n],
            &mut lower_counts,
            &mut upper_counts,
            0,
            n,
        );

        lower_counts
            .into_iter()
            .zip(upper_counts)
            .map(|(l, u)| (l * u) as i64)
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2179() {
        let nums1 = vec![2, 0, 1, 3];
        let nums2 = vec![0, 1, 2, 3];
        let expected = 1;
        assert_eq!(Solution::good_triplets(nums1, nums2), expected);
        let nums1 = vec![4, 0, 1, 3, 2];
        let nums2 = vec![4, 1, 0, 2, 3];
        let expected = 4;
        assert_eq!(Solution::good_triplets(nums1, nums2), expected);
    }
}
