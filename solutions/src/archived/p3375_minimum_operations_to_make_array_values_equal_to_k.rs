///
/// # 3375. Minimum Operations to Make Array Values Equal to K
///
/// You are given an integer array `nums` and an integer `k`.
///
/// An integer `h` is called **valid** if all values in the array that are **strictly greater** than `h` are *identical*.
///
/// For example, if `nums = [10, 8, 10, 8]`, a **valid** integer is `h = 9` because all `nums[i] > 9` are equal to 10, but 5 is not a **valid** integer.
///
/// You are allowed to perform the following operation on `nums`:
///
/// * Select an integer `h` that is *valid* for the **current** values in `nums`.
/// * For each index `i` where `nums[i] > h`, set `nums[i]` to `h`.
///
/// Return the **minimum** number of operations required to make every element in `nums` **equal** to `k`. If it is impossible to make all elements equal to `k`, return -1.
///
/// **Example 1:**
///
/// **Input:** nums = [5,2,5,4,5], k = 2
///
/// **Output:** 2
///
/// **Explanation:**
///
/// The operations can be performed in order using valid integers 4 and then 2.
///
/// **Example 2:**
///
/// **Input:** nums = [2,1,2], k = 2
///
/// **Output:** -1
///
/// **Explanation:**
///
/// It is impossible to make all the values equal to 2.
///
/// **Example 3:**
///
/// **Input:** nums = [9,7,5,3], k = 1
///
/// **Output:** 4
///
/// **Explanation:**
///
/// The operations can be performed using valid integers in the order 7, 5, 3, and 1.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 100 `
/// * `1 <= nums[i] <= 100`
/// * `1 <= k <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/
// discuss: https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums.dedup();

        match nums[0].cmp(&k) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => nums.len() as i32 - 1,
            std::cmp::Ordering::Greater => nums.len() as i32,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3375() {
        let nums = vec![5, 2, 5, 4, 5];
        let k = 2;
        let expected = 2;
        assert_eq!(Solution::min_operations(nums, k), expected);
        let nums = vec![2, 1, 2];
        let k = 2;
        let expected = -1;
        assert_eq!(Solution::min_operations(nums, k), expected);
        let nums = vec![9, 7, 5, 3];
        let k = 1;
        let expected = 4;
        assert_eq!(Solution::min_operations(nums, k), expected);
    }
}
