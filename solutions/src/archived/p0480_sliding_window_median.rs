use std::collections::BTreeSet;

///
/// # 480. Sliding Window Median
///
/// The **median** is the middle value in an ordered integer list. If the size of the list is even, there is no middle value. So the median is the mean of the two middle values.
///
/// * For examples, if `arr = [2,3,4]`, the median is `3`.
/// * For examples, if `arr = [1,2,3,4]`, the median is `(2 + 3) / 2 = 2.5`.
///
/// You are given an integer array `nums` and an integer `k`. There is a sliding window of size `k` which is moving from the very left of the array to the very right. You can only see the `k` numbers in the window. Each time the sliding window moves right by one position.
///
/// Return *the median array for each window in the original array*. Answers within `10<sup>-5</sup>` of the actual value will be accepted.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
/// Output: [1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000]
/// Explanation:
/// Window position                Median
/// ---------------                -----
/// [1  3  -1] -3  5  3  6  7        1
///  1 [3  -1  -3] 5  3  6  7       -1
///  1  3 [-1  -3  5] 3  6  7       -1
///  1  3  -1 [-3  5  3] 6  7        3
///  1  3  -1  -3 [5  3  6] 7        5
///  1  3  -1  -3  5 [3  6  7]       6
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,3,4,2,3,1,4,2], k = 3
/// Output: [2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= k <= nums.length <= 10<sup>5</sup>`
/// * `-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/sliding-window-median/
// discuss: https://leetcode.com/problems/sliding-window-median/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;

        let mut lower =
            BTreeSet::from_iter(nums.iter().enumerate().take(k).map(|(i, &x)| (x as i64, i)));
        let mut upper = BTreeSet::new();

        while lower.len() > upper.len() {
            upper.insert(lower.pop_last().unwrap());
        }

        let mut result = vec![match k % 2 {
            0 => (lower.last().unwrap().0 + upper.first().unwrap().0) as f64 / 2.0,
            _ => upper.first().unwrap().0 as f64,
        }];

        for (i, x) in nums.windows(k + 1).enumerate() {
            let (left, right) = (x[0] as i64, x[k] as i64);

            lower.remove(&(left, i));
            upper.remove(&(left, i));

            if !upper.is_empty() && right < upper.first().unwrap().0 {
                lower.insert((right, i + k));
            } else {
                upper.insert((right, i + k));
            }

            while lower.len() > upper.len() {
                upper.insert(lower.pop_last().unwrap());
            }

            while lower.len() + 1 < upper.len() {
                lower.insert(upper.pop_first().unwrap());
            }

            result.push(match k % 2 {
                0 => (lower.last().unwrap().0 + upper.first().unwrap().0) as f64 / 2.0,
                _ => upper.first().unwrap().0 as f64,
            });
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_480() {
        // let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        // let k = 3;
        // let expected = vec![1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000];
        // assert_eq!(Solution::median_sliding_window(nums, k), expected);
        // let nums = vec![1, 2, 3, 4, 2, 3, 1, 4, 2];
        // let k = 3;
        // let expected = vec![
        //     2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000,
        // ];
        // assert_eq!(Solution::median_sliding_window(nums, k), expected);
        // let nums = vec![1, 1, 1, 1];
        // let k = 2;
        // let expected = vec![1., 1., 1.];
        // assert_eq!(Solution::median_sliding_window(nums, k), expected);
        let nums = vec![4, 1, 7, 1, 8, 7, 8, 7, 7, 4];
        let k = 4;
        let expected = vec![
            2.50000, 4.00000, 7.00000, 7.50000, 7.50000, 7.00000, 7.00000,
        ];
        assert_eq!(Solution::median_sliding_window(nums, k), expected);
    }
}
