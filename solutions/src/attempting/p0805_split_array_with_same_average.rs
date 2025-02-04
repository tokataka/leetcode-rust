///
/// # 805. Split Array With Same Average
///
/// You are given an integer array `nums`.
///
/// You should move each element of `nums` into one of the two arrays `A` and `B` such that `A` and `B` are non-empty, and `average(A) == average(B)`.
///
/// Return `true` if it is possible to achieve that and `false` otherwise.
///
/// **Note** that for an array `arr`, `average(arr)` is the total_sum of all the elements of `arr` over the length of `arr`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,2,3,4,5,6,7,8]
/// Output: true
/// Explanation: We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [3,1]
/// Output: false
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 30`
/// * `0 <= nums[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/split-array-with-same-average/
// discuss: https://leetcode.com/problems/split-array-with-same-average/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let total_sum: i32 = nums.iter().cloned().sum();
        let total_count = nums.len() as i32;

        fn _gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                return a;
            }

            _gcd(b, a % b)
        }

        let gcd = _gcd(total_sum, total_count);

        if gcd == 1 {
            return false;
        }

        let targets = (1..=(gcd / 2))
            .map(|x| (total_sum * x / gcd, total_count * x / gcd))
            .fold(vec![None; 31], |mut acc, x| {
                acc[x.1 as usize] = Some(x.0);
                acc
            });

        let mut st = vec![(0, 0)];
        let mut visited = HashSet::new();

        for len in 1..=(nums.len() / 2) {
            let mut next = vec![];

            for (sum, bitmask) in st {
                for idx in 0..nums.len() {
                    if bitmask & (1 << idx) == 0 {
                        let next_sum = sum + nums[idx];
                        let next_bitmask = bitmask | (1 << idx);

                        if visited.contains(&(next_sum, len)) {
                            continue;
                        }

                        visited.insert((next_sum, len));

                        if let Some(x) = targets[len] {
                            if x == next_sum {
                                return true;
                            }
                        }
                        next.push((next_sum, next_bitmask));
                    }
                }
            }

            st = next;
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_805() {
        // let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // let expected = true;
        // assert_eq!(Solution::split_array_same_average(nums), expected);
        // let nums = vec![3, 1];
        // let expected = false;
        // assert_eq!(Solution::split_array_same_average(nums), expected);
        let nums = vec![5,3,11,19,2];
        let expected = true;
        assert_eq!(Solution::split_array_same_average(nums), expected);
    }
}
