///
/// # 1128. Number of Equivalent Domino Pairs
///
/// Given a list of `dominoes`, `dominoes[i] = [a, b]` is **equivalent to** `dominoes[j] = [c, d]` if and only if either (`a == c` and `b == d`), or (`a == d` and `b == c`) - that is, one domino can be rotated to be equal to another domino.
///
/// Return *the number of pairs* `(i, j)` *for which* `0 <= i < j < dominoes.length`*, and* `dominoes[i]` *is **equivalent to*** `dominoes[j]`.
///
/// **Example 1:**
///
/// ```
/// Input: dominoes = [[1,2],[2,1],[3,4],[5,6]]
/// Output: 1
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: dominoes = [[1,2],[1,2],[1,1],[1,2],[2,2]]
/// Output: 3
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= dominoes.length <= 4 * 10<sup>4</sup>`
/// * `dominoes[i].length == 2`
/// * `1 <= dominoes[i][j] <= 9`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-equivalent-domino-pairs/
// discuss: https://leetcode.com/problems/number-of-equivalent-domino-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        dominoes
            .into_iter()
            .fold(vec![vec![0; 10]; 10], |mut acc, x| {
                let (a, b) = (x[0].min(x[1]), x[0].max(x[1]));
                acc[a as usize][b as usize] += 1;
                acc
            })
            .into_iter()
            .flatten()
            .map(|x| (x * (x - 1)) / 2)
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1128() {
        let dominoes = nd_vec![[1, 2], [2, 1], [3, 4], [5, 6]];
        let expected = 1;
        assert_eq!(Solution::num_equiv_domino_pairs(dominoes), expected);
        let dominoes = nd_vec![[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]];
        let expected = 3;
        assert_eq!(Solution::num_equiv_domino_pairs(dominoes), expected);
    }
}
