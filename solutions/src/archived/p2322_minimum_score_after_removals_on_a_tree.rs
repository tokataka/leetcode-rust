///
/// # 2322. Minimum Score After Removals on a Tree
///
/// There is an undirected connected tree with `n` nodes labeled from `0` to `n - 1` and `n - 1` edges.
///
/// You are given a **0-indexed** integer array `nums` of length `n` where `nums[i]` represents the value of the `i<sup>th</sup>` node. You are also given a 2D integer array `edges` of length `n - 1` where `edges[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an edge between nodes `a<sub>i</sub>` and `b<sub>i</sub>` in the tree.
///
/// Remove two **distinct** edges of the tree to form three connected components. For a pair of removed edges, the following steps are defined:
///
/// 1. Get the XOR of all the values of the nodes for **each** of the three components respectively.
/// 2. The **difference** between the **largest** XOR value and the **smallest** XOR value is the **score** of the pair.
///
/// * For example, say the three components have the node values: `[4,5,7]`, `[1,9]`, and `[3,3,3]`. The three XOR values are `4 ^ 5 ^ 7 = **6**`, `1 ^ 9 = **8**`, and `3 ^ 3 ^ 3 = **3**`. The largest XOR value is `8` and the smallest XOR value is `3`. The score is then `8 - 3 = 5`.
///
/// Return *the **minimum** score of any possible pair of edge removals on the given tree*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/05/03/ex1drawio.png)
///
/// ```
/// Input: nums = [1,5,5,4,11], edges = [[0,1],[1,2],[1,3],[3,4]]
/// Output: 9
/// Explanation: The diagram above shows a way to make a pair of removals.
/// - The 1st component has nodes [1,3,4] with values [5,4,11]. Its XOR value is 5 ^ 4 ^ 11 = 10.
/// - The 2nd component has node [0] with value [1]. Its XOR value is 1 = 1.
/// - The 3rd component has node [2] with value [5]. Its XOR value is 5 = 5.
/// The score is the difference between the largest and smallest XOR value which is 10 - 1 = 9.
/// It can be shown that no other pair of removals will obtain a smaller score than 9.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/05/03/ex2drawio.png)
///
/// ```
/// Input: nums = [5,5,2,4,4,2], edges = [[0,1],[1,2],[5,2],[4,3],[1,3]]
/// Output: 0
/// Explanation: The diagram above shows a way to make a pair of removals.
/// - The 1st component has nodes [3,4] with values [4,4]. Its XOR value is 4 ^ 4 = 0.
/// - The 2nd component has nodes [1,0] with values [5,5]. Its XOR value is 5 ^ 5 = 0.
/// - The 3rd component has nodes [2,5] with values [2,2]. Its XOR value is 2 ^ 2 = 0.
/// The score is the difference between the largest and smallest XOR value which is 0 - 0 = 0.
/// We cannot obtain a smaller score than 0.
///
/// ```
///
/// **Constraints:**
///
/// * `n == nums.length`
/// * `3 <= n <= 1000`
/// * `1 <= nums[i] <= 10<sup>8</sup>`
/// * `edges.length == n - 1`
/// * `edges[i].length == 2`
/// * `0 <= a<sub>i</sub>, b<sub>i</sub> < n`
/// * `a<sub>i</sub> != b<sub>i</sub>`
/// * `edges` represents a valid tree.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/
// discuss: https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let total_xor = nums.iter().fold(0, |acc, x| acc ^ x);

        let mut graph = vec![vec![]; nums.len()];

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
        }

        fn _minimum_score(
            cur: usize,
            prev: usize,
            total_xor: i32,
            nums: &Vec<i32>,
            graph: &Vec<Vec<usize>>,
        ) -> (i32, i32, Vec<i32>) {
            let mut min_score = i32::MAX;
            let mut subtotal_xor = nums[cur];
            let mut found_xor: Vec<i32> = Vec::new();

            for &next in graph[cur].iter().filter(|&&x| x != prev) {
                let (next_min_score, next_subtotal_xor, next_found_xor) =
                    _minimum_score(next, cur, total_xor, nums, graph);

                min_score = min_score.min(next_min_score);
                subtotal_xor ^= next_subtotal_xor;

                for &a in &found_xor {
                    for &b in &next_found_xor {
                        let c = total_xor ^ a ^ b;
                        min_score = min_score.min(a.max(b).max(c) - a.min(b).min(c));
                    }
                }

                found_xor.extend(next_found_xor);
            }

            if cur != prev {
                let c = total_xor ^ subtotal_xor;

                for &a in &found_xor {
                    let b = subtotal_xor ^ a;
                    min_score = min_score.min(a.max(b).max(c) - a.min(b).min(c));
                }

                found_xor.push(subtotal_xor);
            }

            (min_score, subtotal_xor, found_xor)
        }

        _minimum_score(0, 0, total_xor, &nums, &graph).0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2322() {
        // let nums = vec![1, 5, 5, 4, 11];
        // let edges = nd_vec![[0, 1], [1, 2], [1, 3], [3, 4]];
        // let expected = 9;
        // assert_eq!(Solution::minimum_score(nums, edges), expected);
        // let nums = vec![5, 5, 2, 4, 4, 2];
        // let edges = nd_vec![[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]];
        // let expected = 0;
        // assert_eq!(Solution::minimum_score(nums, edges), expected);
        let nums = vec![9, 14, 2, 1];
        let edges = nd_vec![[2, 3], [3, 0], [3, 1]];
        let expected = 11;
        assert_eq!(Solution::minimum_score(nums, edges), expected);
    }
}
