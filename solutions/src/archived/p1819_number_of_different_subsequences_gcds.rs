///
/// # 1819. Number of Different Subsequences GCDs
///
/// You are given an array `nums` that consists of positive integers.
///
/// The **GCD** of a sequence of numbers is defined as the greatest integer that divides **all** the numbers in the sequence evenly.
///
/// * For example, the GCD of the sequence `[4,6,16]` is `2`.
///
/// A **subsequence** of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
///
/// * For example, `[2,5,10]` is a subsequence of `[1,2,1,**2**,4,1,**5**,**10**]`.
///
/// Return *the **number** of **different** GCDs among all **non-empty** subsequences of* `nums`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/03/17/image-1.png)
///
/// ```
/// Input: nums = [6,10,3]
/// Output: 5
/// Explanation: The figure shows all the non-empty subsequences and their GCDs.
/// The different GCDs are 6, 10, 3, 2, and 1.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [5,15,40,5,6]
/// Output: 7
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `1 <= nums[i] <= 2 * 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-different-subsequences-gcds/
// discuss: https://leetcode.com/problems/number-of-different-subsequences-gcds/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut occurence = vec![false; 200001];

        for num in nums {
            occurence[num as usize] = true;
        }

        for x in 1..=200000 {
            if let Some(total_gcd) = (x..=200000)
                .step_by(x)
                .filter(|&x| occurence[x])
                .reduce(gcd)
            {
                if total_gcd == x {
                    result += 1;
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1819() {
        let nums = vec![6, 10, 3];
        let expected = 5;
        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), expected);
        let nums = vec![5, 15, 40, 5, 6];
        let expected = 7;
        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), expected);
    }
}
