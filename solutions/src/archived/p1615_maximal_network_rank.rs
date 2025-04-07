///
/// # 1615. Maximal Network Rank
///
/// There is an infrastructure of `n` cities with some number of `roads` connecting these cities. Each `roads[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is a bidirectional road between cities `a<sub>i</sub>` and `b<sub>i</sub>`.
///
/// The **network rank** of **two different cities** is defined as the total number of **directly** connected roads to **either** city. If a road is directly connected to both cities, it is only counted **once**.
///
/// The **maximal network rank** of the infrastructure is the **maximum network rank** of all pairs of different cities.
///
/// Given the integer `n` and the array `roads`, return *the **maximal network rank** of the entire infrastructure*.
///
/// **Example 1:**
///
/// **![](https://assets.leetcode.com/uploads/2020/09/21/ex1.png)**
///
/// ```
/// Input: n = 4, roads = [[0,1],[0,3],[1,2],[1,3]]
/// Output: 4
/// Explanation: The network rank of cities 0 and 1 is 4 as there are 4 roads that are connected to either 0 or 1. The road between 0 and 1 is only counted once.
///
/// ```
///
/// **Example 2:**
///
/// **![](https://assets.leetcode.com/uploads/2020/09/21/ex2.png)**
///
/// ```
/// Input: n = 5, roads = [[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]]
/// Output: 5
/// Explanation: There are 5 roads that are connected to cities 1 or 2.
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: n = 8, roads = [[0,1],[1,2],[2,3],[2,4],[5,6],[5,7]]
/// Output: 5
/// Explanation: The network rank of 2 and 5 is 5. Notice that all the cities do not have to be connected.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 100`
/// * `0 <= roads.length <= n * (n - 1) / 2`
/// * `roads[i].length == 2`
/// * `0 <= a<sub>i</sub>, b<sub>i</sub> <= n-1`
/// * `a<sub>i</sub> != b<sub>i</sub>`
/// * Each pair of cities has **at most one** road connecting them.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-network-rank/
// discuss: https://leetcode.com/problems/maximal-network-rank/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut graph = vec![vec![]; n];
        let mut is_direct = vec![vec![false; n]; n];

        for road in roads {
            let (a, b) = (road[0] as usize, road[1] as usize);

            graph[a].push(b);
            graph[b].push(a);
            is_direct[a][b] = true;
            is_direct[b][a] = true;
        }

        let mut result = 0;

        for a in 0..n - 1 {
            for b in a + 1..n {
                result = result
                    .max(graph[a].len() + graph[b].len() - if is_direct[a][b] { 1 } else { 0 });
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1615() {
        let n = 4;
        let roads = nd_vec![[0, 1], [0, 3], [1, 2], [1, 3]];
        let expected = 4;
        assert_eq!(Solution::maximal_network_rank(n, roads), expected);
        let n = 5;
        let roads = nd_vec![[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]];
        let expected = 5;
        assert_eq!(Solution::maximal_network_rank(n, roads), expected);
        let n = 8;
        let roads = nd_vec![[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]];
        let expected = 5;
        assert_eq!(Solution::maximal_network_rank(n, roads), expected);
    }
}
