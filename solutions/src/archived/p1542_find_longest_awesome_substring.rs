///
/// # 1542. Find Longest Awesome Substring
///
/// You are given a string `s`. An **awesome** substring is a non-empty substring of `s` such that we can make any number of swaps in order to make it a palindrome.
///
/// Return *the length of the maximum length **awesome substring** of* `s`.
///
/// **Example 1:**
///
/// ```
/// Input: s = "3242415"
/// Output: 5
/// Explanation: "24241" is the longest awesome substring, we can form the palindrome "24142" with some swaps.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "12345678"
/// Output: 1
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "213123"
/// Output: 6
/// Explanation: "213123" is the longest awesome substring, we can form the palindrome "231132" with some swaps.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s` consists only of digits.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-longest-awesome-substring/
// discuss: https://leetcode.com/problems/find-longest-awesome-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut result = 1;

        let mut map = vec![usize::MAX; 1024];
        map[0] = 0;

        let mut cur = 0;

        for (i, ch) in s.bytes().enumerate() {
            cur ^= 1 << (ch - b'0');

            for x in 0..=9 {
                if map[cur ^ (1 << x)] != usize::MAX {
                    result = result.max(i + 1 - map[cur ^ (1 << x)]);
                }
            }

            if map[cur] != usize::MAX {
                result = result.max(i + 1 - map[cur]);
            } else {
                map[cur] = i + 1;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1542() {
        let s = "3242415".to_owned();
        let expected = 5;
        assert_eq!(Solution::longest_awesome(s), expected);
        let s = "12345678".to_owned();
        let expected = 1;
        assert_eq!(Solution::longest_awesome(s), expected);
        let s = "213123".to_owned();
        let expected = 6;
        assert_eq!(Solution::longest_awesome(s), expected);
    }
}
