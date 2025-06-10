use std::collections::BinaryHeap;

///
/// # 1354. Construct Target Array With Multiple Sums
///
/// You are given an array `target` of n integers. From a starting array `arr` consisting of `n` 1's, you may perform the following procedure :
///
/// * let `x` be the sum of all elements currently in your array.
/// * choose index `i`, such that `0 <= i < n` and set the value of `arr` at index `i` to `x`.
/// * You may repeat this procedure as many times as needed.
///
/// Return `true` *if it is possible to construct the* `target` *array from* `arr`*, otherwise, return* `false`.
///
/// **Example 1:**
///
/// ```
/// Input: target = [9,3,5]
/// Output: true
/// Explanation: Start with arr = [1, 1, 1]
/// [1, 1, 1], sum = 3 choose index 1
/// [1, 3, 1], sum = 5 choose index 2
/// [1, 3, 5], sum = 9 choose index 0
/// [9, 3, 5] Done
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: target = [1,1,1,2]
/// Output: false
/// Explanation: Impossible to create target array from [1,1,1,1].
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: target = [8,5]
/// Output: true
///
/// ```
///
/// **Constraints:**
///
/// * `n == target.length`
/// * `1 <= n <= 5 * 10<sup>4</sup>`
/// * `1 <= target[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/construct-target-array-with-multiple-sums/
// discuss: https://leetcode.com/problems/construct-target-array-with-multiple-sums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }

        let mut sum = target.iter().sum::<i32>();
        let mut target = BinaryHeap::from(target);

        while let Some(x) = target.pop() {
            if x == 1 {
                return true;
            }

            let sub = sum - x;
            let &next = target.peek().unwrap_or(&1);

            let next_x = x - (x - next + sub - 1) / sub * sub;

            if next_x < 1 || next_x == x {
                return false;
            }

            sum -= x - next_x;

            if next_x > 1 {
                target.push(next_x);
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1354() {
        // let target = vec![9, 3, 5];
        // let expected = true;
        // assert_eq!(Solution::is_possible(target), expected);
        // let target = vec![1, 1, 1, 2];
        // let expected = false;
        // assert_eq!(Solution::is_possible(target), expected);
        // let target = vec![8, 5];
        // let expected = true;
        // assert_eq!(Solution::is_possible(target), expected);
        let target = vec![1, 1000000000];
        let expected = true;
        assert_eq!(Solution::is_possible(target), expected);
    }
}
