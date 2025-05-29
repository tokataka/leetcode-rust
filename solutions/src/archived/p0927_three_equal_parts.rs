///
/// # 927. Three Equal Parts
///
/// You are given an array `arr` which consists of only zeros and ones, divide the array into **three non-empty parts** such that all of these parts represent the same binary value.
///
/// If it is possible, return any `[i, j]` with `i + 1 < j`, such that:
///
/// * `arr[0], arr[1], ..., arr[i]` is the first part,
/// * `arr[i + 1], arr[i + 2], ..., arr[j - 1]` is the second part, and
/// * `arr[j], arr[j + 1], ..., arr[arr.length - 1]` is the third part.
/// * All three parts have equal binary values.
///
/// If it is not possible, return `[-1, -1]`.
///
/// Note that the entire part is used when considering what binary value it represents. For example, `[1,1,0]` represents `6` in decimal, not `3`. Also, leading zeros **are allowed**, so `[0,1,1]` and `[1,1]` represent the same value.
///
/// **Example 1:**
///
/// ```
/// Input: arr = [1,0,1,0,1]
/// Output: [0,3]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: arr = [1,1,0,1,1]
/// Output: [-1,-1]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: arr = [1,1,0,0,1]
/// Output: [0,2]
///
/// ```
///
/// **Constraints:**
///
/// * `3 <= arr.length <= 3 * 10<sup>4</sup>`
/// * `arr[i]` is `0` or `1`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/three-equal-parts/
// discuss: https://leetcode.com/problems/three-equal-parts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let one_positions = arr
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        if one_positions.is_empty() {
            return vec![0, 2];
        }

        if one_positions.len() % 3 != 0 {
            return vec![-1, -1];
        }

        let parts_count = one_positions.len() / 3;

        let parts = one_positions
            .chunks(parts_count)
            .map(|x| &arr[*x.first().unwrap()..=*x.last().unwrap()])
            .collect::<Vec<_>>();

        if parts.windows(2).any(|x| x[0] != x[1]) {
            return vec![-1, -1];
        }

        let trailing_zero_count = arr.len() - 1 - one_positions.last().unwrap();

        let i = one_positions[parts_count - 1] + trailing_zero_count;

        if i >= one_positions[parts_count] {
            return vec![-1, -1];
        }

        let j = one_positions[parts_count * 2 - 1] + 1 + trailing_zero_count;

        if j > one_positions[parts_count * 2] {
            return vec![-1, -1];
        }

        vec![i as i32, j as i32]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_927() {
        let arr = vec![1, 0, 1, 0, 1];
        let expected = vec![0, 3];
        assert_eq!(Solution::three_equal_parts(arr), expected);
        let arr = vec![1, 1, 0, 1, 1];
        let expected = vec![-1, -1];
        assert_eq!(Solution::three_equal_parts(arr), expected);
        let arr = vec![1, 1, 0, 0, 1];
        let expected = vec![0, 2];
        assert_eq!(Solution::three_equal_parts(arr), expected);
    }
}
