///
/// # 3470. Permutations IV
///
/// Given two integers, `n` and `k`, an **alternating permutation** is a permutation of the first `n` positive integers such that no **two** adjacent elements are both odd or both even.
///
/// Return the **k-th** **alternating permutation** sorted in *lexicographical order*. If there are fewer than `k` valid **alternating permutations**, return an empty list.
///
/// **Example 1:**
///
/// **Input:** n = 4, k = 6
///
/// **Output:** [3,4,1,2]
///
/// **Explanation:**
///
/// The lexicographically-sorted alternating permutations of `[1, 2, 3, 4]` are:
///
/// 1. `[1, 2, 3, 4]`
/// 2. `[1, 4, 3, 2]`
/// 3. `[2, 1, 4, 3]`
/// 4. `[2, 3, 4, 1]`
/// 5. `[3, 2, 1, 4]`
/// 6. `[3, 4, 1, 2]` ← 6th permutation
/// 7. `[4, 1, 2, 3]`
/// 8. `[4, 3, 2, 1]`
///
/// Since `k = 6`, we return `[3, 4, 1, 2]`.
///
/// **Example 2:**
///
/// **Input:** n = 3, k = 2
///
/// **Output:** [3,2,1]
///
/// **Explanation:**
///
/// The lexicographically-sorted alternating permutations of `[1, 2, 3]` are:
///
/// 1. `[1, 2, 3]`
/// 2. `[3, 2, 1]` ← 2nd permutation
///
/// Since `k = 2`, we return `[3, 2, 1]`.
///
/// **Example 3:**
///
/// **Input:** n = 2, k = 3
///
/// **Output:** []
///
/// **Explanation:**
///
/// The lexicographically-sorted alternating permutations of `[1, 2]` are:
///
/// 1. `[1, 2]`
/// 2. `[2, 1]`
///
/// There are only 2 alternating permutations, but `k = 3`, which is out of range. Thus, we return an empty list `[]`.
///
/// **Constraints:**
///
/// * `1 <= n <= 100`
/// * `1 <= k <= 10<sup>15</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/permutations-iv/
// discuss: https://leetcode.com/problems/permutations-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn permute(n: i32, k: i64) -> Vec<i32> {
        let n = n as usize;

        let mut muls = (1..=n).map(|x| x.div_ceil(2)).collect::<Vec<_>>();

        if n % 2 == 0 {
            muls[n - 1] = n;
        }

        let mut idxs = vec![];

        let mut k = k as usize - 1;

        while k > 0 {
            let mut cur = 1;
            let mut i = 0;

            while k >= cur * muls[i] {
                cur *= muls[i];
                i += 1;

                if i >= muls.len() {
                    return vec![];
                }
            }

            idxs.resize(n - i - 1, 0);
            idxs.push(k / cur);

            k %= cur;
        }

        idxs.resize(n, 0);

        let mut result = vec![];

        let (evens, odds): (Vec<_>, Vec<_>) = (1..=n as i32).partition(|x| x % 2 == 0);

        let mut nums = [odds, evens];

        let mut cur = 0;

        if n % 2 == 0 {
            if idxs[0] % 2 == 1 {
                cur = 1;
            }

            idxs[0] /= 2;
        }

        for i in idxs {
            result.push(nums[cur].remove(i));
            cur ^= 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3470() {
        let n = 4;
        let k = 6;
        let expected = vec![3, 4, 1, 2];
        assert_eq!(Solution::permute(n, k), expected);
        let n = 3;
        let k = 2;
        let expected = vec![3, 2, 1];
        assert_eq!(Solution::permute(n, k), expected);
        // let n = 2;
        // let k = 3;
        // let expected = vec![];
        // assert_eq!(Solution::permute(n, k), expected);
    }
}
