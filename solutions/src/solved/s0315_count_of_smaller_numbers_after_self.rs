///
/// # 315. Count of Smaller Numbers After Self
///
/// Given an integer array `nums`, return *an integer array* `counts` *where* `counts[i]` *is the number of smaller elements to the right of* `nums[i]`.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [5,2,6,1]
/// Output: [2,1,1,0]
/// Explanation:
/// To the right of 5 there are 2 smaller elements (2 and 1).
/// To the right of 2 there is only 1 smaller element (1).
/// To the right of 6 there is 1 smaller element (1).
/// To the right of 1 there is 0 smaller element.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [-1]
/// Output: [0]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [-1,-1]
/// Output: [0,0]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-of-smaller-numbers-after-self/
// discuss: https://leetcode.com/problems/count-of-smaller-numbers-after-self/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        const NUM_MAX: i32 = 10000;
        const NUM_MIN: i32 = -10000;
        const NUM_SIZE: usize = (NUM_MAX - NUM_MIN) as usize + 1;

        let mut tree = vec![0; NUM_SIZE + 1];
        let mut result = Vec::with_capacity(nums.len());

        fn sum(tree: &[i32], mut i: usize) -> i32 {
            let mut result = 0;

            while i > 0 {
                result += tree[i];
                i -= i & (!i + 1);
            }

            result
        }

        fn add(tree: &mut [i32], mut i: usize) {
            while i <= NUM_SIZE {
                tree[i] += 1;
                i += i & (!i + 1);
            }
        }

        for num in nums.into_iter().rev() {
            let num = num - NUM_MIN + 1;

            result.push(if num > 1 {
                sum(&tree, num as usize - 1)
            } else {
                0
            });
            add(&mut tree, num as usize);
        }

        result.reverse();

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_315() {
        let nums = vec![5, 2, 6, 1];
        let expected = vec![2, 1, 1, 0];
        assert_eq!(Solution::count_smaller(nums), expected);
        let nums = vec![-1];
        let expected = vec![0];
        assert_eq!(Solution::count_smaller(nums), expected);
        let nums = vec![-1, -1];
        let expected = vec![0, 0];
        assert_eq!(Solution::count_smaller(nums), expected);
    }
}
