///
/// # 2434. Using a Robot to Print the Lexicographically Smallest String
///
/// You are given a string `s` and a robot that currently holds an empty string `t`. Apply one of the following operations until `s` and `t` **are both empty**:
///
/// * Remove the **first** character of a string `s` and give it to the robot. The robot will append this character to the string `t`.
/// * Remove the **last** character of a string `t` and give it to the robot. The robot will write this character on paper.
///
/// Return *the lexicographically smallest string that can be written on the paper.*
///
/// **Example 1:**
///
/// ```
/// Input: s = "zza"
/// Output: "azz"
/// Explanation: Let p denote the written string.
/// Initially p="", s="zza", t="".
/// Perform first operation three times p="", s="", t="zza".
/// Perform second operation three times p="azz", s="", t="".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "bac"
/// Output: "abc"
/// Explanation: Let p denote the written string.
/// Perform first operation twice p="", s="c", t="ba".
/// Perform second operation twice p="ab", s="c", t="".
/// Perform first operation p="ab", s="", t="c".
/// Perform second operation p="abc", s="", t="".
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: s = "bdda"
/// Output: "addb"
/// Explanation: Let p denote the written string.
/// Initially p="", s="bdda", t="".
/// Perform first operation four times p="", s="", t="bdda".
/// Perform second operation four times p="addb", s="", t="".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s` consists of only English lowercase letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/
// discuss: https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut p = String::with_capacity(s.len());
        let mut t = vec![];

        let mut freq = [0; 26];
        let mut min_ch = b'z';

        for ch in s.bytes() {
            freq[(ch - b'a') as usize] += 1;
            min_ch = min_ch.min(ch);
        }

        for ch in s.bytes() {
            t.push(ch);
            freq[(ch - b'a') as usize] -= 1;

            while min_ch != b'z' && freq[(min_ch - b'a') as usize] == 0 {
                min_ch += 1;
            }

            while !t.is_empty() && t.last().unwrap() <= &min_ch {
                p.push(t.pop().unwrap() as char);
            }
        }

        p
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2434() {
        let s = "zza".to_owned();
        let expected = "azz".to_owned();
        assert_eq!(Solution::robot_with_string(s), expected);
        let s = "bac".to_owned();
        let expected = "abc".to_owned();
        assert_eq!(Solution::robot_with_string(s), expected);
        let s = "bdda".to_owned();
        let expected = "addb".to_owned();
        assert_eq!(Solution::robot_with_string(s), expected);
    }
}
