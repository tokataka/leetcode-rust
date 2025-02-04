///
/// # 321. Create Maximum Number
///
/// You are given two integer arrays `nums1` and `nums2` of lengths `m` and `n` respectively. `nums1` and `nums2` represent the digits of two numbers. You are also given an integer `k`.
///
/// Create the maximum number of length `k <= m + n` from digits of the two numbers. The relative order of the digits from the same array must be preserved.
///
/// Return an array of the `k` digits representing the answer.
///
/// **Example 1:**
///
/// ```
/// Input: nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
/// Output: [9,8,6,5,3]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums1 = [6,7], nums2 = [6,0,4], k = 5
/// Output: [6,7,6,0,4]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums1 = [3,9], nums2 = [8,9], k = 3
/// Output: [9,8,9]
///
/// ```
///
/// **Constraints:**
///
/// * `m == nums1.length`
/// * `n == nums2.length`
/// * `1 <= m, n <= 500`
/// * `0 <= nums1[i], nums2[i] <= 9`
/// * `1 <= k <= m + n`
/// * `nums1` and `nums2` do not have leading zeros.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/create-maximum-number/
// discuss: https://leetcode.com/problems/create-maximum-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let nums1_len = nums1.len();
        let nums2_len = nums2.len();

        let [nums1_range_max, nums2_range_max] = [&nums1, &nums2].map(|nums| {
            let mut result = vec![vec![0; nums.len()]; nums.len()];
            for j in 0..nums.len() {
                let mut max = (j, nums[j]);

                result[j][j] = max.0;

                for i in (0..j).rev() {
                    if nums[i] >= max.1 {
                        max = (i, nums[i]);
                    }

                    result[i][j] = max.0;
                }
            }
            result
        });

        let mut result = vec![];
        let mut st = vec![(0, 0)];

        for remain in (1..=k).rev() {
            let mut max = -1;
            let mut next_st = vec![];

            for (nums1_idx, nums2_idx) in st {
                let nums1_remain = nums1_len - nums1_idx;
                let nums2_remain = nums2_len - nums2_idx;

                let skip = nums1_remain + nums2_remain - remain;

                if nums1_idx < nums1_len {
                    let nums1_max_idx =
                        nums1_range_max[nums1_idx][(nums1_idx + skip).min(nums1_len - 1)];

                    let nums1_max = nums1[nums1_max_idx];

                    if nums1_max > max {
                        max = nums1_max;
                        next_st = vec![];
                    }

                    if nums1_max == max {
                        next_st.push((nums1_max_idx + 1, nums2_idx));
                    }
                }

                if nums2_idx < nums2_len {
                    let nums2_max_idx =
                        nums2_range_max[nums2_idx][(nums2_idx + skip).min(nums2_len - 1)];

                    let nums2_max = nums2[nums2_max_idx];

                    if nums2_max > max {
                        max = nums2_max;
                        next_st = vec![];
                    }

                    if nums2_max == max {
                        next_st.push((nums1_idx, nums2_max_idx + 1));
                    }
                }
            }

            result.push(max);

            next_st.sort();
            next_st.dedup();

            st = next_st;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_321() {
        let nums1 = vec![3, 4, 6, 5];
        let nums2 = vec![9, 1, 2, 5, 8, 3];
        let k = 5;
        let expected = vec![9, 8, 6, 5, 3];
        assert_eq!(Solution::max_number(nums1, nums2, k), expected);
        // let nums1 = vec![6, 7];
        // let nums2 = vec![6, 0, 4];
        // let k = 5;
        // let expected = vec![6, 7, 6, 0, 4];
        // assert_eq!(Solution::max_number(nums1, nums2, k), expected);
        let nums1 = vec![3, 9];
        let nums2 = vec![8, 9];
        let k = 3;
        let expected = vec![9, 8, 9];
        assert_eq!(Solution::max_number(nums1, nums2, k), expected);
        // let nums1 = vec![
        //     4, 6, 9, 1, 0, 6, 3, 1, 5, 2, 8, 3, 8, 8, 4, 7, 2, 0, 7, 1, 9, 9, 0, 1, 5, 9, 3, 9, 3,
        //     9, 7, 3, 0, 8, 1, 0, 9, 1, 6, 8, 8, 4, 4, 5, 7, 5, 2, 8, 2, 7, 7, 7, 4, 8, 5, 0, 9, 6,
        //     9, 2,
        // ];
        // let nums2 = vec![
        //     9, 9, 4, 5, 1, 2, 0, 9, 3, 4, 6, 3, 0, 9, 2, 8, 8, 2, 4, 8, 6, 5, 4, 4, 2, 9, 5, 0, 7,
        //     3, 7, 5, 9, 6, 6, 8, 8, 0, 2, 4, 2, 2, 1, 6, 6, 5, 3, 6, 2, 9, 6, 4, 5, 9, 7, 8, 0, 7,
        //     2, 3,
        // ];
        // let k = 60;
        // let expected = vec![
        //     9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8, 8, 6, 8, 8, 4, 4, 5, 7, 5, 2, 8, 2, 7, 7, 7,
        //     4, 8, 5, 0, 9, 6, 9, 2, 0, 2, 4, 2, 2, 1, 6, 6, 5, 3, 6, 2, 9, 6, 4, 5, 9, 7, 8, 0, 7,
        //     2, 3,
        // ];
        // assert_eq!(Solution::max_number(nums1, nums2, k), expected);
    }
}
