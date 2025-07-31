///
/// # 1394. Find Lucky Integer in an Array
///
/// Given an array of integers `arr`, a **lucky integer** is an integer that has a frequency in the array equal to its value.
///
/// Return *the largest **lucky integer** in the array*. If there is no **lucky integer** return `-1`.
///
/// **Example 1:**
///
/// ```
/// Input: arr = [2,2,3,4]
/// Output: 2
/// Explanation: The only lucky number in the array is 2 because frequency[2] == 2.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: arr = [1,2,2,3,3,3]
/// Output: 3
/// Explanation: 1, 2 and 3 are all lucky numbers, return the largest of them.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: arr = [2,2,2,3,3]
/// Output: -1
/// Explanation: There are no lucky numbers in the array.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= arr.length <= 500`
/// * `1 <= arr[i] <= 500`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-lucky-integer-in-an-array/
// discuss: https://leetcode.com/problems/find-lucky-integer-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = vec![0; 501];

        for x in arr {
            freq[x as usize] += 1;
        }

        freq.iter()
            .enumerate()
            .skip(1)
            .rev()
            .find(|&(i, &x)| i == x)
            .map(|x| x.0 as i32)
            .unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1394() {
        let arr = vec![2, 2, 3, 4];
        let expected = 2;
        assert_eq!(Solution::find_lucky(arr), expected);
        let arr = vec![1, 2, 2, 3, 3, 3];
        let expected = 3;
        assert_eq!(Solution::find_lucky(arr), expected);
        let arr = vec![2, 2, 2, 3, 3];
        let expected = -1;
        assert_eq!(Solution::find_lucky(arr), expected);
    }
}
