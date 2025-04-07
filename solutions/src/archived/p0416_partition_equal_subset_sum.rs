///
/// # 416. Partition Equal Subset Sum
///
/// Given an integer array `nums`, return `true` *if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or* `false` *otherwise*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [1,5,11,5]
/// Output: true
/// Explanation: The array can be partitioned as [1, 5, 5] and [11].
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,3,5]
/// Output: false
/// Explanation: The array cannot be partitioned into equal sum subsets.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 200`
/// * `1 <= nums[i] <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-equal-subset-sum/
// discuss: https://leetcode.com/problems/partition-equal-subset-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let target = match nums.iter().sum::<i32>() {
            total if total & 1 != 0 => return false,
            total => total / 2,
        };

        let mut cur = vec![0];
        let mut visited = vec![false; 10100];

        for num in nums {
            for prev_sum in cur.clone() {
                let cur_sum = prev_sum + num;
                if !visited[cur_sum as usize] {
                    visited[cur_sum as usize] = true;

                    match cur_sum.cmp(&target) {
                        std::cmp::Ordering::Less => cur.push(cur_sum),
                        std::cmp::Ordering::Equal => return true,
                        std::cmp::Ordering::Greater => (),
                    };
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_416() {
        let nums = vec![1, 5, 11, 5];
        let expected = true;
        assert_eq!(Solution::can_partition(nums), expected);
        let nums = vec![1, 2, 3, 5];
        let expected = false;
        assert_eq!(Solution::can_partition(nums), expected);
    }
}
