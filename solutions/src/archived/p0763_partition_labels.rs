///
/// # 763. Partition Labels
///
/// You are given a string `s`. We want to partition the string into as many parts as possible so that each letter appears in at most one part. For example, the string `"ababcc"` can be partitioned into `["abab", "cc"]`, but partitions such as `["aba", "bcc"]` or `["ab", "ab", "cc"]` are invalid.
///
/// Note that the partition is done so that after concatenating all the parts in order, the resultant string should be `s`.
///
/// Return *a list of integers representing the size of these parts*.
///
/// **Example 1:**
///
/// ```
/// Input: s = "ababcbacadefegdehijhklij"
/// Output: [9,7,8]
/// Explanation:
/// The partition is "ababcbaca", "defegde", "hijhklij".
/// This is a partition so that each letter appears in at most one part.
/// A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: s = "eccbbbbdec"
/// Output: [10]
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= s.length <= 500`
/// * `s` consists of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-labels/
// discuss: https://leetcode.com/problems/partition-labels/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut total = [0; 26];

        for ch in s.bytes() {
            total[ch as usize - b'a' as usize] += 1;
        }

        let mut result = vec![];
        let mut cur = [0; 26];
        let mut count = 0;

        for ch in s.bytes() {
            let ch = ch as usize - b'a' as usize;

            if cur[ch] == 0 {
                cur[ch] = total[ch];
            }

            cur[ch] -= 1;
            count += 1;

            if cur.iter().all(|&c| c == 0) {
                result.push(count);
                count = 0;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_763() {
        let s = "ababcbacadefegdehijhklij".to_owned();
        let expected = vec![9, 7, 8];
        assert_eq!(Solution::partition_labels(s), expected);
        let s = "eccbbbbdec".to_owned();
        let expected = vec![10];
        assert_eq!(Solution::partition_labels(s), expected);
    }
}
