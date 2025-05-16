///
/// # 3337. Total Characters in String After Transformations II
///
/// You are given a string `s` consisting of lowercase English letters, an integer `t` representing the number of **transformations** to perform, and an array `nums` of size 26. In one **transformation**, every character in `s` is replaced according to the following rules:
///
/// * Replace `s[i]` with the **next** `nums[s[i] - 'a']` consecutive characters in the alphabet. For example, if `s[i] = 'a'` and `nums[0] = 3`, the character `'a'` transforms into the next 3 consecutive characters ahead of it, which results in `"bcd"`.
/// * The transformation **wraps** around the alphabet if it exceeds `'z'`. For example, if `s[i] = 'y'` and `nums[24] = 3`, the character `'y'` transforms into the next 3 consecutive characters ahead of it, which results in `"zab"`.
///
/// Return the length of the resulting string after **exactly** `t` transformations.
///
/// Since the answer may be very large, return it **modulo** `10<sup>9</sup> + 7`.
///
/// **Example 1:**
///
/// **Input:** s = "abcyy", t = 2, nums = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2]
///
/// **Output:** 7
///
/// **Explanation:**
///
/// * **First Transformation (t = 1):**
///
///   * `'a'` becomes `'b'` as `nums[0] == 1`
///   * `'b'` becomes `'c'` as `nums[1] == 1`
///   * `'c'` becomes `'d'` as `nums[2] == 1`
///   * `'y'` becomes `'z'` as `nums[24] == 1`
///   * `'y'` becomes `'z'` as `nums[24] == 1`
///   * String after the first transformation: `"bcdzz"`
///
/// * **Second Transformation (t = 2):**
///
///   * `'b'` becomes `'c'` as `nums[1] == 1`
///   * `'c'` becomes `'d'` as `nums[2] == 1`
///   * `'d'` becomes `'e'` as `nums[3] == 1`
///   * `'z'` becomes `'ab'` as `nums[25] == 2`
///   * `'z'` becomes `'ab'` as `nums[25] == 2`
///   * String after the second transformation: `"cdeabab"`
///
/// * **Final Length of the string:** The string is `"cdeabab"`, which has 7 characters.
///
/// **Example 2:**
///
/// **Input:** s = "azbk", t = 1, nums = [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]
///
/// **Output:** 8
///
/// **Explanation:**
///
/// * **First Transformation (t = 1):**
///
///   * `'a'` becomes `'bc'` as `nums[0] == 2`
///   * `'z'` becomes `'ab'` as `nums[25] == 2`
///   * `'b'` becomes `'cd'` as `nums[1] == 2`
///   * `'k'` becomes `'lm'` as `nums[10] == 2`
///   * String after the first transformation: `"bcabcdlm"`
///
/// * **Final Length of the string:** The string is `"bcabcdlm"`, which has 8 characters.
///
/// **Constraints:**
///
/// * `1 <= s.length <= 10<sup>5</sup>`
/// * `s` consists only of lowercase English letters.
/// * `1 <= t <= 10<sup>9</sup>`
/// * `nums.length == 26`
/// * `1 <= nums[i] <= 25`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/total-characters-in-string-after-transformations-ii/
// discuss: https://leetcode.com/problems/total-characters-in-string-after-transformations-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i64 = 1_000_000_007;

struct Matrix(Vec<Vec<i64>>);

impl Matrix {
    pub fn build_freq(s: &str) -> Self {
        let mut res = vec![0; 26];

        for b in s.bytes() {
            res[(b - b'a') as usize] += 1;
        }

        Self(vec![res])
    }

    pub fn build_nums(nums: &[i32]) -> Self {
        let mut res = vec![vec![0; 26]; 26];

        for (i, &num) in nums.iter().enumerate() {
            for j in (i + 1..).take(num as usize) {
                res[i][j % 26] = 1;
            }
        }

        Self(res)
    }

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

        #[allow(clippy::needless_range_loop)]
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
    pub fn length_after_transformations(s: String, mut t: i32, nums: Vec<i32>) -> i32 {
        let mut freq_mat = Matrix::build_freq(&s);
        let mut nums_mat = Matrix::build_nums(&nums);

        while t > 0 {
            if t % 2 == 1 {
                freq_mat = &freq_mat * &nums_mat;
            }

            t /= 2;
            nums_mat = &nums_mat * &nums_mat;
        }

        freq_mat.sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3337() {
        // let s = "abcyy".to_owned();
        // let t = 2;
        // let nums = vec![
        //     1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
        // ];
        // let expected = 7;
        // assert_eq!(Solution::length_after_transformations(s, t, nums), expected);
        // let s = "azbk".to_owned();
        // let t = 1;
        // let nums = vec![
        //     2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        // ];
        // let expected = 8;
        // assert_eq!(Solution::length_after_transformations(s, t, nums), expected);
        let s = "k".to_owned();
        let t = 2;
        let nums = vec![
            2, 2, 1, 3, 1, 1, 2, 3, 3, 2, 1, 2, 2, 1, 1, 3, 1, 2, 2, 1, 3, 3, 3, 2, 2, 1,
        ];
        let expected = 2;
        assert_eq!(Solution::length_after_transformations(s, t, nums), expected);
    }
}
