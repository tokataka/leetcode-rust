///
/// # 3445. Maximum Difference Between Even and Odd Frequency II
///
/// You are given a string `s` and an integer `k`. Your task is to find the **maximum** difference between the frequency of **two** characters, `freq[a] - freq[b]`, in a substring `subs` of `s`, such that:
///
/// * `subs` has a size of **at least** `k`.
/// * Character `a` has an *odd frequency* in `subs`.
/// * Character `b` has an *even frequency* in `subs`.
///
/// Return the **maximum** difference.
///
/// **Note** that `subs` can contain more than 2 **distinct** characters.
///
/// **Example 1:**
///
/// **Input:** s = "12233", k = 4
///
/// **Output:** -1
///
/// **Explanation:**
///
/// For the substring `"12233"`, the frequency of `'1'` is 1 and the frequency of `'3'` is 2. The difference is `1 - 2 = -1`.
///
/// **Example 2:**
///
/// **Input:** s = "1122211", k = 3
///
/// **Output:** 1
///
/// **Explanation:**
///
/// For the substring `"11222"`, the frequency of `'2'` is 3 and the frequency of `'1'` is 2. The difference is `3 - 2 = 1`.
///
/// **Example 3:**
///
/// **Input:** s = "110", k = 3
///
/// **Output:** -1
///
/// **Constraints:**
///
/// * `3 <= s.length <= 3 * 10<sup>4</sup>`
/// * `s` consists only of digits `'0'` to `'4'`.
/// * The input is generated that at least one substring has a character with an even frequency and a character with an odd frequency.
/// * `1 <= k <= s.length`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-ii/
// discuss: https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let n = s.len();
        let mut ans = i32::MIN;
        let chars = s.as_bytes();

        fn get_status(cnt_a: i32, cnt_b: i32) -> i32 {
            ((cnt_a & 1) << 1) | (cnt_b & 1)
        }

        for a in b'0'..=b'5' {
            for b in b'0'..=b'5' {
                if a == b {
                    continue;
                }

                let mut best = [i32::MAX; 4];
                let (mut cnt_a, mut cnt_b) = (0, 0);
                let (mut prev_a, mut prev_b) = (0, 0);
                let mut left = -1;

                for right in 0..n {
                    cnt_a += if chars[right] == a { 1 } else { 0 };
                    cnt_b += if chars[right] == b { 1 } else { 0 };

                    while (right as i32 - left) >= k && (cnt_b - prev_b) >= 2 {
                        let left_status = get_status(prev_a, prev_b) as usize;
                        best[left_status] = best[left_status].min(prev_a - prev_b);
                        left += 1;
                        prev_a += if chars[left as usize] == a { 1 } else { 0 };
                        prev_b += if chars[left as usize] == b { 1 } else { 0 };
                    }

                    let right_status = get_status(cnt_a, cnt_b) as usize;
                    if best[right_status ^ 0b10] != i32::MAX {
                        ans = ans.max(cnt_a - cnt_b - best[right_status ^ 0b10]);
                    }
                }
            }
        }

        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3445() {
        let s = "12233".to_owned();
        let k = 4;
        let expected = -1;
        assert_eq!(Solution::max_difference(s, k), expected);
        let s = "1122211".to_owned();
        let k = 3;
        let expected = 1;
        assert_eq!(Solution::max_difference(s, k), expected);
        let s = "110".to_owned();
        let k = 3;
        let expected = -1;
        assert_eq!(Solution::max_difference(s, k), expected);
    }
}
