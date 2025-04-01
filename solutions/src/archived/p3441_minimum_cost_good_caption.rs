///
/// # 3441. Minimum Cost Good Caption
///
/// You are given a string `caption` of length `n`. A **good** caption is a string where **every** character appears in groups of **at least 3** consecutive occurrences.
///
/// For example:
///
/// * `"aaabbb"` and `"aaaaccc"` are **good** captions.
/// * `"aabbb"` and `"ccccd"` are **not** good captions.
///
/// You can perform the following operation **any** number of times:
///
/// Choose an index `i` (where `0 <= i < n`) and change the character at that index to either:
///
/// * The character immediately **before** it in the alphabet (if `caption[i] != 'a'`).
/// * The character immediately **after** it in the alphabet (if `caption[i] != 'z'`).
///
/// Your task is to convert the given `caption` into a **good** caption using the **minimum** number of operations, and return it. If there are **multiple** possible good captions, return the **lexicographically smallest** one among them. If it is **impossible** to create a good caption, return an empty string `""`.
///
/// **Example 1:**
///
/// **Input:** caption = "cdcd"
///
/// **Output:** "cccc"
///
/// **Explanation:**
///
/// It can be shown that the given caption cannot be transformed into a good caption with fewer than 2 operations. The possible good captions that can be created using exactly 2 operations are:
///
/// * `"dddd"`: Change `caption[0]` and `caption[2]` to their next character `'d'`.
/// * `"cccc"`: Change `caption[1]` and `caption[3]` to their previous character `'c'`.
///
/// Since `"cccc"` is lexicographically smaller than `"dddd"`, return `"cccc"`.
///
/// **Example 2:**
///
/// **Input:** caption = "aca"
///
/// **Output:** "aaa"
///
/// **Explanation:**
///
/// It can be proven that the given caption requires at least 2 operations to be transformed into a good caption. The only good caption that can be obtained with exactly 2 operations is as follows:
///
/// * Operation 1: Change `caption[1]` to `'b'`. `caption = "aba"`.
/// * Operation 2: Change `caption[1]` to `'a'`. `caption = "aaa"`.
///
/// Thus, return `"aaa"`.
///
/// **Example 3:**
///
/// **Input:** caption = "bc"
///
/// **Output:** ""
///
/// **Explanation:**
///
/// It can be shown that the given caption cannot be converted to a good caption by using any number of operations.
///
/// **Constraints:**
///
/// * `1 <= caption.length <= 5 * 10<sup>4</sup>`
/// * `caption` consists only of lowercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-good-caption/
// discuss: https://leetcode.com/problems/minimum-cost-good-caption/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost_good_caption(caption: String) -> String {
        if caption.len() < 3 {
            return "".to_string();
        }

        let mut dp = vec![[[usize::MAX; 4]; 26]; caption.len() + 1];

        for ch in 0..26 {
            dp[caption.len()][ch][3] = 0;
        }

        // fill dp table

        for (pos, cur_ch) in caption.chars().rev().enumerate() {
            let pos = caption.len() - pos;
            let cur_ch = cur_ch as usize - 'a' as usize;

            for ch in 0..26 {
                for len in 1..3 {
                    if dp[pos][ch][len] == usize::MAX {
                        continue;
                    }

                    let next_len = if len - 1 > 0 { len - 1 } else { 3 };

                    dp[pos - 1][ch][next_len] =
                        dp[pos - 1][ch][next_len].min(dp[pos][ch][len] + cur_ch.abs_diff(ch));
                }

                if dp[pos][ch][3] == usize::MAX {
                    continue;
                }

                for next_ch in 0..26 {
                    let next_len = if ch != next_ch || pos == caption.len() {
                        2
                    } else {
                        3
                    };
                    dp[pos - 1][next_ch][next_len] = dp[pos - 1][next_ch][next_len]
                        .min(dp[pos][ch][3] + cur_ch.abs_diff(next_ch));
                }
            }
        }

        let mut result = String::with_capacity(caption.len());

        let mut cur_len = 3;
        let mut cur_ch = 0;

        dp.iter()
            .enumerate()
            .take(caption.len())
            .for_each(|(i, dp_cur)| {
                if cur_len == 3 && i < caption.len() - 2 {
                    let (changed_cost, changed_ch) =
                        (0..26).map(|ch| (dp_cur[ch][3], ch)).min().unwrap();

                    let unchanged_cost = dp_cur[cur_ch][2].min(dp_cur[cur_ch][1]);

                    if i > 0
                        && (changed_cost > unchanged_cost
                            || changed_cost == unchanged_cost && changed_ch > cur_ch)
                    {
                        cur_len = 1;
                    } else {
                        cur_ch = changed_ch;
                    }
                }

                result.push((cur_ch as u8 + b'a') as char);

                if cur_len == 1 {
                    cur_len = 3;
                } else {
                    cur_len -= 1;
                }
            });

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3441() {
        // let caption = "cdcd".to_owned();
        // let expected = "cccc".to_owned();
        // assert_eq!(Solution::min_cost_good_caption(caption), expected);
        // let caption = "aca".to_owned();
        // let expected = "aaa".to_owned();
        // assert_eq!(Solution::min_cost_good_caption(caption), expected);
        // let caption = "bc".to_owned();
        // let expected = "".to_owned();
        // assert_eq!(Solution::min_cost_good_caption(caption), expected);
        let caption = "antwfdps".to_owned();
        let expected = "nnnnnppp".to_owned();
        assert_eq!(Solution::min_cost_good_caption(caption), expected);
    }
}
