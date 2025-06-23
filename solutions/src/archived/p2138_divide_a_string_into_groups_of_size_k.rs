///
/// # 2138. Divide a String Into Groups of Size k
///
/// A string `s` can be partitioned into groups of size `k` using the following procedure:
///
/// * The first group consists of the first `k` characters of the string, the second group consists of the next `k` characters of the string, and so on. Each element can be a part of **exactly one** group.
/// * For the last group, if the string **does not** have `k` characters remaining, a character `fill` is used to complete the group.
///
/// Note that the partition is done so that after removing the `fill` character from the last group (if it exists) and concatenating all the groups in order, the resultant string should be `s`.
///
/// Given the string `s`, the size of each group `k` and the character `fill`, return *a string array denoting the **composition of every group*** `s` *has been divided into, using the above procedure*.
///
/// **Example 1:**
///
/// ```
/// Input: s = "abcdefghi", k = 3, fill = "x"
/// Output: ["abc","def","ghi"]
/// Explanation:
/// The first 3 characters "abc" form the first group.
/// The next 3 characters "def" form the second group.
/// The last 3 characters "ghi" form the third group.
/// Since all groups can be completely filled by characters from the string, we do not need to use fill.
/// Thus, the groups formed are "abc", "def", and "ghi".
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "abcdefghij", k = 3, fill = "x"
/// Output: ["abc","def","ghi","jxx"]
/// Explanation:
/// Similar to the previous example, we are forming the first three groups "abc", "def", and "ghi".
/// For the last group, we can only use the character 'j' from the string. To complete this group, we add 'x' twice.
/// Thus, the 4 groups formed are "abc", "def", "ghi", and "jxx".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 100`
/// * `s` consists of lowercase English letters only.
/// * `1 <= k <= 100`
/// * `fill` is a lowercase English letter.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/
// discuss: https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        s.as_bytes()
            .chunks(k as usize)
            .map(|x| {
                let mut s = String::from_utf8_lossy(x).into_owned();
                s.push_str(&fill.to_string().repeat(k as usize - s.len()));
                s
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2138() {
        let s = "abcdefghi".to_owned();
        let k = 3;
        let fill = 'x';
        let expected = vec_string!["abc", "def", "ghi"];
        assert_eq!(Solution::divide_string(s, k, fill), expected);
        let s = "abcdefghij".to_owned();
        let k = 3;
        let fill = 'x';
        let expected = vec_string!["abc", "def", "ghi", "jxx"];
        assert_eq!(Solution::divide_string(s, k, fill), expected);
    }
}
