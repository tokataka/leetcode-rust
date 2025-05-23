///
/// # 38. Count and Say
///
/// The **count-and-say** sequence is a sequence of digit strings defined by the recursive formula:
///
/// * `countAndSay(1) = "1"`
/// * `countAndSay(n)` is the run-length encoding of `countAndSay(n - 1)`.
///
/// [Run-length encoding](http://en.wikipedia.org/wiki/Run-length_encoding) (RLE) is a string compression method that works by replacing consecutive identical characters (repeated 2 or more times) with the concatenation of the character and the number marking the count of the characters (length of the run). For example, to compress the string `"3322251"` we replace `"33"` with `"23"`, replace `"222"` with `"32"`, replace `"5"` with `"15"` and replace `"1"` with `"11"`. Thus the compressed string becomes `"23321511"`.
///
/// Given a positive integer `n`, return *the* `n<sup>th</sup>` *element of the **count-and-say** sequence*.
///
/// **Example 1:**
///
/// **Input:** n = 4
///
/// **Output:** "1211"
///
/// **Explanation:**
///
/// ```
/// countAndSay(1) = "1"
/// countAndSay(2) = RLE of "1" = "11"
/// countAndSay(3) = RLE of "11" = "21"
/// countAndSay(4) = RLE of "21" = "1211"
///
/// ```
///
/// **Example 2:**
///
/// **Input:** n = 1
///
/// **Output:** "1"
///
/// **Explanation:**
///
/// This is the base case.
///
/// **Constraints:**
///
/// * `1 <= n <= 30`
///
/// **Follow up:** Could you solve it iteratively?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-and-say/
// discuss: https://leetcode.com/problems/count-and-say/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut cur = "1".to_string();

        for _ in 1..n {
            let mut next = String::new();

            for chunk in cur.as_bytes().chunk_by(|a, b| a == b) {
                next.push((chunk.len() as u8 + b'0') as char);
                next.push(chunk[0] as char);
            }

            cur = next;
        }

        cur
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38() {
        let n = 4;
        let expected = "1211".to_owned();
        assert_eq!(Solution::count_and_say(n), expected);
        let n = 1;
        let expected = "1".to_owned();
        assert_eq!(Solution::count_and_say(n), expected);
    }
}
