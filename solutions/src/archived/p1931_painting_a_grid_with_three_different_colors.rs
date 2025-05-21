///
/// # 1931. Painting a Grid With Three Different Colors
///
/// You are given two integers `m` and `n`. Consider an `m x n` grid where each cell is initially white. You can paint each cell **red**, **green**, or **blue**. All cells **must** be painted.
///
/// Return *the number of ways to color the grid with **no two adjacent cells having the same color***. Since the answer can be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/06/22/colorthegrid.png)
///
/// ```
/// Input: m = 1, n = 1
/// Output: 3
/// Explanation: The three possible colorings are shown in the image above.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/06/22/copy-of-colorthegrid.png)
///
/// ```
/// Input: m = 1, n = 2
/// Output: 6
/// Explanation: The six possible colorings are shown in the image above.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: m = 5, n = 5
/// Output: 580986
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= m <= 5`
/// * `1 <= n <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/painting-a-grid-with-three-different-colors/
// discuss: https://leetcode.com/problems/painting-a-grid-with-three-different-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i64 = 1_000_000_007;

struct Matrix(Vec<Vec<i64>>);

impl Matrix {
    pub fn sum(&self) -> i32 {
        (self.0.iter().flatten().sum::<i64>() % MOD) as i32
    }
}

impl std::ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        if self.0[0].len() != rhs.0.len() {
            panic!("invalid multiplication");
        }

        let mut res = vec![vec![0; rhs.0[0].len()]; self.0.len()];

        for i in 0..self.0.len() {
            for j in 0..rhs.0[0].len() {
                for k in 0..self.0[0].len() {
                    res[i][j] += self.0[i][k] * rhs.0[k][j] % MOD;
                }

                res[i][j] %= MOD;
            }
        }

        Matrix(res)
    }
}

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let m = m as usize;

        let mut states = vec![];

        'outer: for state in 0..3usize.pow(m as u32) {
            let mut cur = state;
            let mut prev = 3;

            for _ in 0..m {
                if prev == cur % 3 {
                    continue 'outer;
                }

                prev = cur % 3;
                cur /= 3;
            }

            states.push(state);
        }

        let states_len = states.len();

        let mut transition = Matrix(vec![vec![0; states_len]; states_len]);

        for (i, &state_i) in states.iter().enumerate() {
            'outer: for (j, &state_j) in states.iter().enumerate().skip(i + 1) {
                let mut state_i = state_i;
                let mut state_j = state_j;

                for _ in 0..m {
                    if state_i % 3 == state_j % 3 {
                        continue 'outer;
                    }

                    state_i /= 3;
                    state_j /= 3;
                }

                transition.0[i][j] = 1;
                transition.0[j][i] = 1;
            }
        }

        let mut result = Matrix(vec![vec![1; states_len]]);

        let mut n = n as usize - 1;

        while n > 0 {
            if n % 2 == 1 {
                result = &result * &transition;
            }

            n /= 2;
            transition = &transition * &transition;
        }

        result.sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1931() {
        // let m = 1;
        // let n = 1;
        // let expected = 3;
        // assert_eq!(Solution::color_the_grid(m, n), expected);
        // let m = 1;
        // let n = 2;
        // let expected = 6;
        // assert_eq!(Solution::color_the_grid(m, n), expected);
        let m = 2;
        let n = 2;
        let expected = 18;
        assert_eq!(Solution::color_the_grid(m, n), expected);
        let m = 5;
        let n = 5;
        let expected = 580986;
        assert_eq!(Solution::color_the_grid(m, n), expected);
    }
}
