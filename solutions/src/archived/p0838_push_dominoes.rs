///
/// # 838. Push Dominoes
///
/// There are `n` dominoes in a line, and we place each domino vertically upright. In the beginning, we simultaneously push some of the dominoes either to the left or to the right.
///
/// After each second, each domino that is falling to the left pushes the adjacent domino on the left. Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.
///
/// When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.
///
/// For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.
///
/// You are given a string `dominoes` representing the initial state where:
///
/// * `dominoes[i] = 'L'`, if the `i<sup>th</sup>` domino has been pushed to the left,
/// * `dominoes[i] = 'R'`, if the `i<sup>th</sup>` domino has been pushed to the right, and
/// * `dominoes[i] = '.'`, if the `i<sup>th</sup>` domino has not been pushed.
///
/// Return *a string representing the final state*.
///
/// **Example 1:**
///
/// ```
/// Input: dominoes = "RR.L"
/// Output: "RR.L"
/// Explanation: The first domino expends no additional force on the second domino.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/05/18/domino.png)
///
/// ```
/// Input: dominoes = ".L.R...LR..L.."
/// Output: "LL.RR.LLRRLL.."
///
/// ```
///
/// **Constraints:**
///
/// * `n == dominoes.length`
/// * `1 <= n <= 10<sup>5</sup>`
/// * `dominoes[i]` is either `'L'`, `'R'`, or `'.'`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/push-dominoes/
// discuss: https://leetcode.com/problems/push-dominoes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.into_bytes();
        let mut lr_char = dominoes[0];
        let mut lr_idx = 0;

        for cur in 1..dominoes.len() {
            match dominoes[cur] {
                b'L' => {
                    if lr_char == b'R' {
                        let mid = (cur + lr_idx).div_ceil(2);
                        dominoes[lr_idx + 1..mid].fill(b'R');
                        dominoes[mid..cur].fill(b'L');
                        if (cur - (lr_idx + 1)) % 2 == 1 {
                            dominoes[mid] = b'.';
                        }
                    } else if lr_char == b'L' {
                        dominoes[lr_idx + 1..cur].fill(b'L');
                    } else {
                        dominoes[lr_idx..cur].fill(b'L');
                    }

                    lr_char = b'L';
                    lr_idx = cur;
                }
                b'R' => {
                    if lr_char == b'R' {
                        dominoes[lr_idx + 1..cur].fill(b'R');
                    }

                    lr_char = b'R';
                    lr_idx = cur;
                }
                _ => (),
            }
        }

        if lr_char == b'R' {
            dominoes[lr_idx + 1..].fill(b'R');
        }

        String::from_utf8(dominoes).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_838() {
        let dominoes = "RR.L".to_owned();
        let expected = "RR.L".to_owned();
        assert_eq!(Solution::push_dominoes(dominoes), expected);
        let dominoes = ".L.R...LR..L..".to_owned();
        let expected = "LL.RR.LLRRLL..".to_owned();
        assert_eq!(Solution::push_dominoes(dominoes), expected);
    }
}
