///
/// # 2767. Partition String Into Minimum Beautiful Substrings
///
/// Given a binary string `s`, partition the string into one or more **substrings** such that each substring is **beautiful**.
///
/// A string is **beautiful** if:
///
/// * It doesn't contain leading zeros.
/// * It's the **binary** representation of a number that is a power of `5`.
///
/// Return *the **minimum** number of substrings in such partition.* If it is impossible to partition the string `s` into beautiful substrings, return `-1`.
///
/// A **substring** is a contiguous sequence of characters in a string.
///
/// **Example 1:**
///
/// ```
/// Input: s = "1011"
/// Output: 2
/// Explanation: We can paritition the given string into ["101", "1"].
/// - The string "101" does not contain leading zeros and is the binary representation of integer 51 = 5.
/// - The string "1" does not contain leading zeros and is the binary representation of integer 50 = 1.
/// It can be shown that 2 is the minimum number of beautiful substrings that s can be partitioned into.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "111"
/// Output: 3
/// Explanation: We can paritition the given string into ["1", "1", "1"].
/// - The string "1" does not contain leading zeros and is the binary representation of integer 50 = 1.
/// It can be shown that 3 is the minimum number of beautiful substrings that s can be partitioned into.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "0"
/// Output: -1
/// Explanation: We can not partition the given string into beautiful substrings.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 15`
/// * `s[i]` is either `'0'` or `'1'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-string-into-minimum-beautiful-substrings/
// discuss: https://leetcode.com/problems/partition-string-into-minimum-beautiful-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_beautiful_substrings(s: String) -> i32 {
        pub fn _minimum_beautiful_substrings(
            cache: &mut [i32; 15],
            s: &str,
            left: usize,
        ) -> Option<i32> {
            if left >= s.len() {
                return Some(0);
            }

            if &s[left..left + 1] == "0" {
                return None;
            }

            match cache[left] {
                i32::MAX => (),
                -1 => return None,
                x => return Some(x),
            }

            let min_count = (left + 1..=s.len())
                .filter_map(|right| {
                    let mut cur = i32::from_str_radix(&s[left..right], 2).unwrap();

                    while cur > 1 {
                        if cur % 5 != 0 {
                            return None;
                        }
                        cur /= 5;
                    }

                    _minimum_beautiful_substrings(cache, s, right).map(|x| x + 1)
                })
                .min();

            cache[left] = min_count.unwrap_or(-1);

            min_count
        }

        _minimum_beautiful_substrings(&mut [i32::MAX; 15], &s, 0).unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2767() {
        let s = "1011".to_owned();
        let expected = 2;
        assert_eq!(Solution::minimum_beautiful_substrings(s), expected);
        let s = "111".to_owned();
        let expected = 3;
        assert_eq!(Solution::minimum_beautiful_substrings(s), expected);
        let s = "0".to_owned();
        let expected = -1;
        assert_eq!(Solution::minimum_beautiful_substrings(s), expected);
    }
}
