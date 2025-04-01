///
/// # 15. 3Sum
///
/// Given an integer array nums, return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.
///
/// Notice that the solution set must not contain duplicate triplets.
///
/// **Example 1:**
///
/// ```
/// Input: nums = [-1,0,1,2,-1,-4]
/// Output: [[-1,-1,2],[-1,0,1]]
/// Explanation:
/// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
/// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
/// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
/// The distinct triplets are [-1,0,1] and [-1,-1,2].
/// Notice that the order of the output and the order of the triplets does not matter.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [0,1,1]
/// Output: []
/// Explanation: The only possible triplet does not sum up to 0.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = [0,0,0]
/// Output: [[0,0,0]]
/// Explanation: The only possible triplet sums up to 0.
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= nums.length <= 3000`
/// * `-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();

        let mut v1_last = i32::MIN;
        let mut result = vec![];

        for (i1, &v1) in nums.iter().enumerate() {
            if v1 > 0 {
                break;
            }

            if v1 == v1_last {
                continue;
            }

            v1_last = v1;

            let mut v2_last = i32::MIN;

            for (i2, &v2) in nums.iter().enumerate().skip(i1 + 1) {
                if v1 + v2 > 0 {
                    break;
                }

                if v2 == v2_last {
                    continue;
                }

                v2_last = v2;

                if nums[i2 + 1..].binary_search(&(-v1 - v2)).is_ok() {
                    result.push(vec![v1, v2, -v1 - v2]);
                };
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
    fn test_15() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = nd_vec![[-1, -1, 2], [-1, 0, 1]];
        assert_eq!(Solution::three_sum(nums), expected);
        let nums = vec![0, 1, 1];
        let expected: Vec<Vec<i32>> = nd_vec![];
        assert_eq!(Solution::three_sum(nums), expected);
        let nums = vec![0, 0, 0];
        let expected = nd_vec![[0, 0, 0]];
        assert_eq!(Solution::three_sum(nums), expected);
    }
}
