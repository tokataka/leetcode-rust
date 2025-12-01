///
/// # 1345. Jump Game IV
///
/// Given an array of integers `arr`, you are initially positioned at the first index of the array.
///
/// In one step you can jump from index `i` to index:
///
/// * `i + 1` where: `i + 1 < arr.length`.
/// * `i - 1` where: `i - 1 >= 0`.
/// * `j` where: `arr[i] == arr[j]` and `i != j`.
///
/// Return *the minimum number of steps* to reach the **last index** of the array.
///
/// Notice that you can not jump outside of the array at any time.
///
/// **Example 1:**
///
/// ```
/// Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
/// Output: 3
/// Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: arr = [7]
/// Output: 0
/// Explanation: Start index is the last index. You do not need to jump.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: arr = [7,6,9,6,9,6,9,7]
/// Output: 1
/// Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= arr.length <= 5 * 10<sup>4</sup>`
/// * `-10<sup>8</sup> <= arr[i] <= 10<sup>8</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-iv/
// discuss: https://leetcode.com/problems/jump-game-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let mut num_idxs = HashMap::new();

        for (i, &num) in arr.iter().enumerate() {
            num_idxs.entry(num).or_insert(vec![]).push(i);
        }

        let mut visited = vec![false; n];
        visited[0] = true;

        let mut q = VecDeque::new();
        q.push_back((0, 0));

        while let Some((idx, jumps)) = q.pop_front() {
            if idx == n - 1 {
                return jumps;
            }

            if idx > 0 && !visited[idx - 1] {
                visited[idx - 1] = true;
                q.push_back((idx - 1, jumps + 1));
            }

            if idx < n - 1 && !visited[idx + 1] {
                visited[idx + 1] = true;
                q.push_back((idx + 1, jumps + 1));
            }

            if let Some(x) = num_idxs.remove(&arr[idx]) {
                for next in x {
                    if !visited[next] {
                        visited[next] = true;
                        q.push_back((next, jumps + 1));
                    }
                }
            }
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1345() {
        let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        let expected = 3;
        assert_eq!(Solution::min_jumps(arr), expected);
        let arr = vec![7];
        let expected = 0;
        assert_eq!(Solution::min_jumps(arr), expected);
        let arr = vec![7, 6, 9, 6, 9, 6, 9, 7];
        let expected = 1;
        assert_eq!(Solution::min_jumps(arr), expected);
    }
}
