///
/// # 1301. Number of Paths with Max Score
///
/// You are given a square `board` of characters. You can move on the board starting at the bottom right square marked with the character `'S'`.
///
/// You need to reach the top left square marked with the character `'E'`. The rest of the squares are labeled either with a numeric character `1, 2, ..., 9` or with an obstacle `'X'`. In one move you can go up, left or up-left (diagonally) only if there is no obstacle there.
///
/// Return a list of two integers: the first integer is the maximum sum of numeric characters you can collect, and the second is the number of such paths that you can take to get that maximum sum, **taken modulo `10^9 + 7`**.
///
/// In case there is no path, return `[0, 0]`.
///
/// **Example 1:**
///
/// ```
/// Input: board = ["E23","2X2","12S"]
/// Output: [7,1]
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: board = ["E12","1X1","21S"]
/// Output: [4,2]
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: board = ["E11","XXX","11S"]
/// Output: [0,0]
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= board.length == board[i].length <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-paths-with-max-score/
// discuss: https://leetcode.com/problems/number-of-paths-with-max-score/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        const MOD: i32 = 1_000_000_007;
        let height = board.len();
        let width = board[0].len();
        let mut sum_counts = vec![vec![(0, 0); width]; height];
        sum_counts[height - 1][width - 1] = (0, 1);

        for (i, row) in board.iter().enumerate().rev() {
            for (j, ch) in row.bytes().enumerate().rev() {
                let score = match ch {
                    b'0'..=b'9' => ch as i32 - b'0' as i32,
                    b'E' => 0,
                    _ => continue,
                };

                let mut max_score = 0;
                let mut max_score_count = 0;

                for (di, dj) in [(1, 1), (1, 0), (0, 1)] {
                    let &(prev_score, count) = sum_counts
                        .get(i + di)
                        .and_then(|x| x.get(j + dj))
                        .unwrap_or(&(0, 0));

                    if count > 0 {
                        match max_score.cmp(&(prev_score)) {
                            std::cmp::Ordering::Less => {
                                max_score = prev_score;
                                max_score_count = count;
                            }
                            std::cmp::Ordering::Equal => max_score_count += count,
                            std::cmp::Ordering::Greater => (),
                        }
                    }
                }

                sum_counts[i][j] = (max_score + score, max_score_count % MOD);
            }
        }

        vec![sum_counts[0][0].0, sum_counts[0][0].1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1301() {
        let board = vec_string!["E23", "2X2", "12S"];
        let expected = vec![7, 1];
        assert_eq!(Solution::paths_with_max_score(board), expected);
        let board = vec_string!["E12", "1X1", "21S"];
        let expected = vec![4, 2];
        assert_eq!(Solution::paths_with_max_score(board), expected);
        let board = vec_string!["E11", "XXX", "11S"];
        let expected = vec![0, 0];
        assert_eq!(Solution::paths_with_max_score(board), expected);
    }
}
