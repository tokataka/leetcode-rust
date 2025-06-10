///
/// # 386. Lexicographical Numbers
///
/// Given an integer `n`, return all the numbers in the range `[1, n]` sorted in lexicographical order.
///
/// You must write an algorithm that runs in `O(n)` time and uses `O(1)` extra space.
///
/// **Example 1:**
///
/// ```
/// Input: n = 13
/// Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: n = 2
/// Output: [1,2]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 5 * 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/lexicographical-numbers/
// discuss: https://leetcode.com/problems/lexicographical-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);

        let mut cur = 1;

        loop {
            result.push(cur);

            while cur * 10 <= n {
                cur *= 10;

                result.push(cur);
            }

            if cur + 1 > n {
                cur /= 10;
            }

            while cur % 10 == 9 {
                cur /= 10;
            }

            if cur == 0 {
                break;
            }

            cur += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_386() {
        let n = 13;
        let expected = vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::lexical_order(n), expected);
        let n = 2;
        let expected = vec![1, 2];
        assert_eq!(Solution::lexical_order(n), expected);
    }
}
