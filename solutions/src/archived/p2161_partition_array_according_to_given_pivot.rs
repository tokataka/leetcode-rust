///
/// # 2161. Partition Array According to Given Pivot
///
/// You are given a **0-indexed** integer array `nums` and an integer `pivot`. Rearrange `nums` such that the following conditions are satisfied:
///
/// * Every element less than `pivot` appears **before** every element greater than `pivot`.
/// * Every element equal to `pivot` appears **in between** the elements less than and greater than `pivot`.
/// * The **relative order** of the elements less than `pivot` and the elements greater than `pivot` is maintained.
///   * More formally, consider every `p<sub>i</sub>`, `p<sub>j</sub>` where `p<sub>i</sub>` is the new position of the `i<sup>th</sup>` element and `p<sub>j</sub>` is the new position of the `j<sup>th</sup>` element. If `i < j` and **both** elements are smaller (*or larger*) than `pivot`, then `p<sub>i</sub> < p<sub>j</sub>`.
///
/// Return `nums` *after the rearrangement.*
///
/// **Example 1:**
///
/// ```
/// Input: nums = [9,12,5,10,14,3,10], pivot = 10
/// Output: [9,5,3,10,10,12,14]
/// Explanation:
/// The elements 9, 5, and 3 are less than the pivot so they are on the left side of the array.
/// The elements 12 and 14 are greater than the pivot so they are on the right side of the array.
/// The relative ordering of the elements less than and greater than pivot is also maintained. [9, 5, 3] and [12, 14] are the respective orderings.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [-3,4,3,2], pivot = 2
/// Output: [-3,2,4,3]
/// Explanation:
/// The element -3 is less than the pivot so it is on the left side of the array.
/// The elements 4 and 3 are greater than the pivot so they are on the right side of the array.
/// The relative ordering of the elements less than and greater than pivot is also maintained. [-3] and [4, 3] are the respective orderings.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `-10<sup>6</sup> <= nums[i] <= 10<sup>6</sup>`
/// * `pivot` equals to an element of `nums`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-array-according-to-given-pivot/
// discuss: https://leetcode.com/problems/partition-array-according-to-given-pivot/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut lower = vec![];
        let mut greater = vec![];
        let mut equal_count = 0;

        for &num in &nums {
            match num.cmp(&pivot) {
                std::cmp::Ordering::Less => lower.push(num),
                std::cmp::Ordering::Greater => greater.push(num),
                std::cmp::Ordering::Equal => equal_count += 1,
            };
        }

        nums[0..lower.len()].copy_from_slice(&lower);
        nums[lower.len()..lower.len() + equal_count].fill(pivot);
        nums[lower.len() + equal_count..].copy_from_slice(&greater);

        nums
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2161() {
        let nums = vec![9, 12, 5, 10, 14, 3, 10];
        let pivot = 10;
        let expected = vec![9, 5, 3, 10, 10, 12, 14];
        assert_eq!(Solution::pivot_array(nums, pivot), expected);
        let nums = vec![-3, 4, 3, 2];
        let pivot = 2;
        let expected = vec![-3, 2, 4, 3];
        assert_eq!(Solution::pivot_array(nums, pivot), expected);
    }
}
