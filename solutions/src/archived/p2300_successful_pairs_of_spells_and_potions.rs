///
/// # 2300. Successful Pairs of Spells and Potions
///
/// You are given two positive integer arrays `spells` and `potions`, of length `n` and `m` respectively, where `spells[i]` represents the strength of the `i<sup>th</sup>` spell and `potions[j]` represents the strength of the `j<sup>th</sup>` potion.
///
/// You are also given an integer `success`. A spell and potion pair is considered **successful** if the **product** of their strengths is **at least** `success`.
///
/// Return *an integer array* `pairs` *of length* `n` *where* `pairs[i]` *is the number of **potions** that will form a successful pair with the* `i<sup>th</sup>` *spell.*
///
/// **Example 1:**
///
/// ```
/// Input: spells = [5,1,3], potions = [1,2,3,4,5], success = 7
/// Output: [4,0,3]
/// Explanation:
/// - 0th spell: 5 * [1,2,3,4,5] = [5,10,15,20,25]. 4 pairs are successful.
/// - 1st spell: 1 * [1,2,3,4,5] = [1,2,3,4,5]. 0 pairs are successful.
/// - 2nd spell: 3 * [1,2,3,4,5] = [3,6,9,12,15]. 3 pairs are successful.
/// Thus, [4,0,3] is returned.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: spells = [3,1,2], potions = [8,5,8], success = 16
/// Output: [2,0,2]
/// Explanation:
/// - 0th spell: 3 * [8,5,8] = [24,15,24]. 2 pairs are successful.
/// - 1st spell: 1 * [8,5,8] = [8,5,8]. 0 pairs are successful.
/// - 2nd spell: 2 * [8,5,8] = [16,10,16]. 2 pairs are successful.
/// Thus, [2,0,2] is returned.
///
/// ```
///
/// **Constraints:**
///
/// * `n == spells.length`
/// * `m == potions.length`
/// * `1 <= n, m <= 10<sup>5</sup>`
/// * `1 <= spells[i], potions[i] <= 10<sup>5</sup>`
/// * `1 <= success <= 10<sup>10</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
// discuss: https://leetcode.com/problems/successful-pairs-of-spells-and-potions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut success_count = vec![0; 100001];

        for potion in potions {
            let min_spell = (success + potion as i64 - 1) / potion as i64;

            if min_spell <= 100000 {
                success_count[min_spell as usize] += 1;
            }
        }

        let mut sum = 0;

        for i in 1..=100000 {
            success_count[i] += sum;
            sum = success_count[i];
        }

        spells
            .into_iter()
            .map(|spell| success_count[spell as usize])
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2300() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;
        let expected = vec![4, 0, 3];
        assert_eq!(
            Solution::successful_pairs(spells, potions, success),
            expected
        );
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;
        let expected = vec![2, 0, 2];
        assert_eq!(
            Solution::successful_pairs(spells, potions, success),
            expected
        );
    }
}
