///
/// # 2862. Maximum Element-Sum of a Complete Subset of Indices
///
/// You are given a **1****-indexed** array `nums`. Your task is to select a **complete subset** from `nums` where every pair of selected indices multiplied is a perfect square,. i. e. if you select `a<sub>i</sub>` and `a<sub>j</sub>`, `i * j` must be a perfect square.
///
/// Return the *sum* of the complete subset with the *maximum sum*.
///
/// **Example 1:**
///
/// **Input:** nums = [8,7,3,5,7,2,4,9]
///
/// **Output:** 16
///
/// **Explanation:**
///
/// We select elements at indices 2 and 8 and `2 * 8` is a perfect square.
///
/// **Example 2:**
///
/// **Input:** nums = [8,10,3,8,1,13,7,9,4]
///
/// **Output:** 20
///
/// **Explanation:**
///
/// We select elements at indices 1, 4, and 9. `1 * 4`, `1 * 9`, `4 * 9` are perfect squares.
///
/// **Constraints:**
///
/// * `1 <= n == nums.length <= 10<sup>4</sup>`
/// * `1 <= nums[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-element-sum-of-a-complete-subset-of-indices/
// discuss: https://leetcode.com/problems/maximum-element-sum-of-a-complete-subset-of-indices/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        let mut result = 0;

        for base in 1..=n {
            let mut sum = 0;

            for x in (1..).map(|i| base * i * i).take_while(|&x| x <= n) {
                sum += nums[x - 1] as i64;
            }

            result = result.max(sum);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2862() {
        let nums = vec![8, 7, 3, 5, 7, 2, 4, 9];
        let expected = 16;
        assert_eq!(Solution::maximum_sum(nums), expected);
        let nums = vec![8, 10, 3, 8, 1, 13, 7, 9, 4];
        let expected = 20;
        assert_eq!(Solution::maximum_sum(nums), expected);
    }
}
