///
/// # 2538. Difference Between Maximum and Minimum Price Sum
///
/// There exists an undirected and initially unrooted tree with `n` nodes indexed from `0` to `n - 1`. You are given the integer `n` and a 2D integer array `edges` of length `n - 1`, where `edges[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an edge between nodes `a<sub>i</sub>` and `b<sub>i</sub>` in the tree.
///
/// Each node has an associated price. You are given an integer array `price`, where `price[i]` is the price of the `i<sup>th</sup>` node.
///
/// The **price sum** of a given path is the sum of the prices of all nodes lying on that path.
///
/// The tree can be rooted at any node `root` of your choice. The incurred **cost** after choosing `root` is the difference between the maximum and minimum **price sum** amongst all paths starting at `root`.
///
/// Return *the **maximum** possible **cost*** *amongst all possible root choices*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2022/12/01/example14.png)
///
/// ```
/// Input: n = 6, edges = [[0,1],[1,2],[1,3],[3,4],[3,5]], price = [9,8,7,6,10,5]
/// Output: 24
/// Explanation: The diagram above denotes the tree after rooting it at node 2. The first part (colored in red) shows the path with the maximum price sum. The second part (colored in blue) shows the path with the minimum price sum.
/// - The first path contains nodes [2,1,3,4]: the prices are [7,8,6,10], and the sum of the prices is 31.
/// - The second path contains the node [2] with the price [7].
/// The difference between the maximum and minimum price sum is 24. It can be proved that 24 is the maximum cost.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2022/11/24/p1_example2.png)
///
/// ```
/// Input: n = 3, edges = [[0,1],[1,2]], price = [1,1,1]
/// Output: 2
/// Explanation: The diagram above denotes the tree after rooting it at node 0. The first part (colored in red) shows the path with the maximum price sum. The second part (colored in blue) shows the path with the minimum price sum.
/// - The first path contains nodes [0,1,2]: the prices are [1,1,1], and the sum of the prices is 3.
/// - The second path contains node [0] with a price [1].
/// The difference between the maximum and minimum price sum is 2. It can be proved that 2 is the maximum cost.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 10<sup>5</sup>`
/// * `edges.length == n - 1`
/// * `0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1`
/// * `edges` represents a valid tree.
/// * `price.length == n`
/// * `1 <= price[i] <= 10<sup>5</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/difference-between-maximum-and-minimum-price-sum/
// discuss: https://leetcode.com/problems/difference-between-maximum-and-minimum-price-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;

        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            graph[a].push(b);
            graph[b].push(a);
        }

        fn _max_output(
            cur: usize,
            prev: usize,
            graph: &Vec<Vec<usize>>,
            price: &[i32],
        ) -> (i64, i64, i64) {
            let cur_price = price[cur] as i64;

            if graph[cur].is_empty() || graph[cur].len() == 1 && graph[cur][0] == prev {
                return (0, cur_price, 0);
            }

            let mut result_max = 0;

            let mut children_path_with = vec![];
            let mut children_path_without = vec![];

            for (i, &next) in graph[cur].iter().enumerate() {
                if next == prev {
                    continue;
                }

                let (child_max, child_path_with, child_path_without) =
                    _max_output(next, cur, graph, price);

                result_max = result_max.max(child_max);

                children_path_with.push((child_path_with, i));
                children_path_without.push((child_path_without, i));
            }

            children_path_with.sort_unstable_by(|a, b| b.cmp(a));
            children_path_without.sort_unstable_by(|a, b| b.cmp(a));

            let result_path_with = children_path_with[0].0 + cur_price;
            let result_path_without = children_path_without[0].0 + cur_price;

            if children_path_with.len() >= 2 {
                if children_path_with[0].1 != children_path_without[0].1 {
                    result_max = result_max
                        .max(children_path_with[0].0 + children_path_without[0].0 + cur_price);
                } else {
                    result_max = result_max
                        .max(children_path_with[0].0 + children_path_without[1].0 + cur_price);
                    result_max = result_max
                        .max(children_path_with[1].0 + children_path_without[0].0 + cur_price);
                }
            } else {
                result_max = result_max.max(children_path_with[0].0);
                result_max = result_max.max(children_path_without[0].0 + cur_price);
            }

            (result_max, result_path_with, result_path_without)
        }

        _max_output(0, 0, &graph, &price).0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2538() {
        let n = 6;
        let edges = nd_vec![[0, 1], [1, 2], [1, 3], [3, 4], [3, 5]];
        let price = vec![9, 8, 7, 6, 10, 5];
        let expected = 24;
        assert_eq!(Solution::max_output(n, edges, price), expected);
        let n = 3;
        let edges = nd_vec![[0, 1], [1, 2]];
        let price = vec![1, 1, 1];
        let expected = 2;
        assert_eq!(Solution::max_output(n, edges, price), expected);
    }
}
