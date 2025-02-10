///
/// # 3151. Special Array I
///
/// An array is considered **special** if every pair of its adjacent elements contains two numbers with different parity.
///
/// You are given an array of integers `nums`. Return `true` if `nums` is a **special** array, otherwise, return `false`.
///
/// **Example 1:**
///
/// **Input:** nums = [1]
///
/// **Output:** true
///
/// **Explanation:**
///
/// There is only one element. So the answer is `true`.
///
/// **Example 2:**
///
/// **Input:** nums = [2,1,4]
///
/// **Output:** true
///
/// **Explanation:**
///
/// There is only two pairs: `(2,1)` and `(1,4)`, and both of them contain numbers with different parity. So the answer is `true`.
///
/// **Example 3:**
///
/// **Input:** nums = [4,3,1,6]
///
/// **Output:** false
///
/// **Explanation:**
///
/// `nums[1]` and `nums[2]` are both odd. So the answer is `false`.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 100`
/// * `1 <= nums[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/special-array-i/
// discuss: https://leetcode.com/problems/special-array-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|x| x[0] & 1 != x[1] & 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3151() {
        let nums = vec![1];
        let expected = true;
        assert_eq!(Solution::is_array_special(nums), expected);
        let nums = vec![2, 1, 4];
        let expected = true;
        assert_eq!(Solution::is_array_special(nums), expected);
        let nums = vec![4, 3, 1, 6];
        let expected = false;
        assert_eq!(Solution::is_array_special(nums), expected);
    }
}
