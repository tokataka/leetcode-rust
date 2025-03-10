///
/// # 1718. Construct the Lexicographically Largest Valid Sequence
///
/// Given an integer `n`, find a sequence that satisfies all of the following:
///
/// * The integer `1` occurs once in the sequence.
/// * Each integer between `2` and `n` occurs twice in the sequence.
/// * For every integer `i` between `2` and `n`, the **distance** between the two occurrences of `i` is exactly `i`.
///
/// The **distance** between two numbers on the sequence, `a[i]` and `a[j]`, is the absolute difference of their indices, `|j - i|`.
///
/// Return *the **lexicographically largest** sequence**. It is guaranteed that under the given constraints, there is always a solution.*
///
/// A sequence `a` is lexicographically larger than a sequence `b` (of the same length) if in the first position where `a` and `b` differ, sequence `a` has a number greater than the corresponding number in `b`. For example, `[0,1,9,0]` is lexicographically larger than `[0,1,5,6]` because the first position they differ is at the third number, and `9` is greater than `5`.
///
/// **Example 1:**
///
/// ```
/// Input: n = 3
/// Output: [3,1,2,3,2]
/// Explanation: [2,3,2,1,3] is also a valid sequence, but [3,1,2,3,2] is the lexicographically largest valid sequence.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 5
/// Output: [5,3,1,4,3,5,2,4,2]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 20`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/
// discuss: https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        fn backtracking(result: &mut [i32], used: &mut [bool], n: usize, idx: usize) -> bool {
            let mut idx = idx;

            while idx < 2 * n - 1 && result[idx] > 0 {
                idx += 1;
            }

            if idx == 2 * n - 1 {
                return true;
            }

            for x in (1..=n).rev() {
                if used[x] {
                    continue;
                }

                if x > 1 && idx + x >= 2 * n - 1 {
                    return false;
                }

                if x == 1 {
                    result[idx] = x as i32;
                    used[x] = true;

                    if backtracking(result, used, n, idx + 1) {
                        return true;
                    }

                    result[idx] = 0;
                    used[x] = false;
                } else if result[idx] == 0 && result[idx + x] == 0 {
                    result[idx] = x as i32;
                    result[idx + x] = x as i32;
                    used[x] = true;

                    if backtracking(result, used, n, idx + 1) {
                        return true;
                    }

                    result[idx] = 0;
                    result[idx + x] = 0;
                    used[x] = false;
                }
            }

            false
        }

        let n = n as usize;

        let mut result = vec![0; 2 * n - 1];
        let mut used = vec![false; n + 1];

        backtracking(&mut result, &mut used, n, 0);

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1718() {
        // let n = 3;
        // let expected = vec![3, 1, 2, 3, 2];
        // assert_eq!(Solution::construct_distanced_sequence(n), expected);
        // let n = 5;
        // let expected = vec![5, 3, 1, 4, 3, 5, 2, 4, 2];
        // assert_eq!(Solution::construct_distanced_sequence(n), expected);
        let n = 4;
        let expected = vec![4, 2, 3, 2, 4, 3, 1];
        assert_eq!(Solution::construct_distanced_sequence(n), expected);
    }
}
