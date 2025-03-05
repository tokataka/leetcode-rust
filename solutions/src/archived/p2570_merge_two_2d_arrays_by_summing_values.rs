///
/// # 2570. Merge Two 2D Arrays by Summing Values
///
/// You are given two **2D** integer arrays `nums1` and `nums2.`
///
/// * `nums1[i] = [id<sub>i</sub>, val<sub>i</sub>]` indicate that the number with the id `id<sub>i</sub>` has a value equal to `val<sub>i</sub>`.
/// * `nums2[i] = [id<sub>i</sub>, val<sub>i</sub>]` indicate that the number with the id `id<sub>i</sub>` has a value equal to `val<sub>i</sub>`.
///
/// Each array contains **unique** ids and is sorted in **ascending** order by id.
///
/// Merge the two arrays into one array that is sorted in ascending order by id, respecting the following conditions:
///
/// * Only ids that appear in at least one of the two arrays should be included in the resulting array.
/// * Each id should be included **only once** and its value should be the sum of the values of this id in the two arrays. If the id does not exist in one of the two arrays, then assume its value in that array to be `0`.
///
/// Return *the resulting array*. The returned array must be sorted in ascending order by id.
///
/// **Example 1:**
///
/// ```
/// Input: nums1 = [[1,2],[2,3],[4,5]], nums2 = [[1,4],[3,2],[4,1]]
/// Output: [[1,6],[2,3],[3,2],[4,6]]
/// Explanation: The resulting array contains the following:
/// - id = 1, the value of this id is 2 + 4 = 6.
/// - id = 2, the value of this id is 3.
/// - id = 3, the value of this id is 2.
/// - id = 4, the value of this id is 5 + 1 = 6.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums1 = [[2,4],[3,6],[5,5]], nums2 = [[1,3],[4,3]]
/// Output: [[1,3],[2,4],[3,6],[4,3],[5,5]]
/// Explanation: There are no common ids, so we just include each id with its value in the resulting list.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums1.length, nums2.length <= 200`
/// * `nums1[i].length == nums2[j].length == 2`
/// * `1 <= id<sub>i</sub>, val<sub>i</sub> <= 1000`
/// * Both arrays contain unique ids.
/// * Both arrays are in strictly ascending order by id.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/
// discuss: https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![0; 1001];

        for x in nums1.into_iter().chain(nums2) {
            result[x[0] as usize] += x[1];
        }

        result
            .into_iter()
            .enumerate()
            .filter(|&(_, x)| x != 0)
            .map(|(i, x)| vec![i as i32, x])
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2570() {
        let nums1 = nd_vec![[1, 2], [2, 3], [4, 5]];
        let nums2 = nd_vec![[1, 4], [3, 2], [4, 1]];
        let expected = nd_vec![[1, 6], [2, 3], [3, 2], [4, 6]];
        assert_eq!(Solution::merge_arrays(nums1, nums2), expected);
        let nums1 = nd_vec![[2, 4], [3, 6], [5, 5]];
        let nums2 = nd_vec![[1, 3], [4, 3]];
        let expected = nd_vec![[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]];
        assert_eq!(Solution::merge_arrays(nums1, nums2), expected);
    }
}
