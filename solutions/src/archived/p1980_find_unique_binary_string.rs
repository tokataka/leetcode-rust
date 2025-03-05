///
/// # 1980. Find Unique Binary String
///
/// Given an array of strings `nums` containing `n` **unique** binary strings each of length `n`, return *a binary string of length* `n` *that **does not appear** in* `nums`*. If there are multiple answers, you may return **any** of them*.
///
/// **Example 1:**
///
/// ```
/// Input: nums = ["01","10"]
/// Output: "11"
/// Explanation: "11" does not appear in nums. "00" would also be correct.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = ["00","01"]
/// Output: "11"
/// Explanation: "11" does not appear in nums. "10" would also be correct.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: nums = ["111","011","001"]
/// Output: "101"
/// Explanation: "101" does not appear in nums. "000", "010", "100", and "110" would also be correct.
///
/// ```
///
/// **Constraints:**
///
/// * `n == nums.length`
/// * `1 <= n <= 16`
/// * `nums[i].length == n`
/// * `nums[i] `is either `'0'` or `'1'`.
/// * All the strings of `nums` are **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-unique-binary-string/
// discuss: https://leetcode.com/problems/find-unique-binary-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_different_binary_string(mut nums: Vec<String>) -> String {
        nums.sort();
        let n = nums[0].len();

        let mut left = 0;
        let mut d = 2_usize.pow(n as u32 - 1);

        let mut prefix = String::with_capacity(n);

        for _ in 0..n {
            let middle = nums.partition_point(|x| x < &(prefix.clone() + "1"));

            if middle - left < d {
                prefix.push('0');
            } else {
                left = middle;
                prefix.push('1');
            }

            d /= 2;
        }

        prefix
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1980() {
        let nums = vec_string!["01", "10"];
        let expected = "11".to_owned();
        assert_eq!(Solution::find_different_binary_string(nums), expected);
        let nums = vec_string!["00", "01"];
        let expected = "11".to_owned();
        assert_eq!(Solution::find_different_binary_string(nums), expected);
        let nums = vec_string!["111", "011", "001"];
        let expected = "101".to_owned();
        assert_eq!(Solution::find_different_binary_string(nums), expected);
    }
}
