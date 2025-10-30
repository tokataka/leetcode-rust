///
/// # 1320. Minimum Distance to Type a Word Using Two Fingers
///
/// ![](https://assets.leetcode.com/uploads/2020/01/02/leetcode_keyboard.png)
///
/// You have a keyboard layout as shown above in the **X-Y** plane, where each English uppercase letter is located at some coordinate.
///
/// * For example, the letter `'A'` is located at coordinate `(0, 0)`, the letter `'B'` is located at coordinate `(0, 1)`, the letter `'P'` is located at coordinate `(2, 3)` and the letter `'Z'` is located at coordinate `(4, 1)`.
///
/// Given the string `word`, return *the minimum total **distance** to type such string using only two fingers*.
///
/// The **distance** between coordinates `(x<sub>1</sub>, y<sub>1</sub>)` and `(x<sub>2</sub>, y<sub>2</sub>)` is `|x<sub>1</sub> - x<sub>2</sub>| + |y<sub>1</sub> - y<sub>2</sub>|`.
///
/// **Note** that the initial positions of your two fingers are considered free so do not count towards your total distance, also your two fingers do not have to start at the first letter or the first two letters.
///
/// **Example 1:**
///
/// ```
/// Input: word = "CAKE"
/// Output: 3
/// Explanation: Using two fingers, one optimal way to type "CAKE" is:
/// Finger 1 on letter 'C' -> cost = 0
/// Finger 1 on letter 'A' -> cost = Distance from letter 'C' to letter 'A' = 2
/// Finger 2 on letter 'K' -> cost = 0
/// Finger 2 on letter 'E' -> cost = Distance from letter 'K' to letter 'E' = 1
/// Total distance = 3
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: word = "HAPPY"
/// Output: 6
/// Explanation: Using two fingers, one optimal way to type "HAPPY" is:
/// Finger 1 on letter 'H' -> cost = 0
/// Finger 1 on letter 'A' -> cost = Distance from letter 'H' to letter 'A' = 2
/// Finger 2 on letter 'P' -> cost = 0
/// Finger 2 on letter 'P' -> cost = Distance from letter 'P' to letter 'P' = 0
/// Finger 1 on letter 'Y' -> cost = Distance from letter 'A' to letter 'Y' = 4
/// Total distance = 6
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= word.length <= 300`
/// * `word` consists of uppercase English letters.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/
// discuss: https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let n = word.len();

        let mut distances = vec![vec![0; 27]; 27];

        for a in 0usize..26 {
            for b in a + 1..26 {
                let (ax, ay, bx, by) = (a % 6, a / 6, b % 6, b / 6);
                let d = (ax.abs_diff(bx) + ay.abs_diff(by)) as i32;
                distances[a][b] = d;
                distances[b][a] = d;
            }
        }

        let mut dp = vec![vec![vec![i32::MAX; 27]; 27]; n];

        for (i, ch) in word.bytes().map(|ch| (ch - b'A') as usize).enumerate() {
            if i == 0 {
                dp[i][ch][26] = 0;
                continue;
            }

            for first_ch in 0..26 {
                for second_ch in 0..=26 {
                    if dp[i - 1][first_ch][second_ch] != i32::MAX {
                        dp[i][ch][second_ch] = dp[i][ch][second_ch]
                            .min(dp[i - 1][first_ch][second_ch] + distances[first_ch][ch]);

                        dp[i][first_ch][ch] = dp[i][first_ch][ch]
                            .min(dp[i - 1][first_ch][second_ch] + distances[second_ch][ch]);
                    }
                }
            }
        }

        *dp[n - 1].iter().flat_map(|x| x.iter().min()).min().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1320() {
        let word = "CAKE".to_owned();
        let expected = 3;
        assert_eq!(Solution::minimum_distance(word), expected);
        let word = "HAPPY".to_owned();
        let expected = 6;
        assert_eq!(Solution::minimum_distance(word), expected);
    }
}
