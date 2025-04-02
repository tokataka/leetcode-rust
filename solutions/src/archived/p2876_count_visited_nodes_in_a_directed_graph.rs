///
/// # 2876. Count Visited Nodes in a Directed Graph
///
/// There is a **directed** graph consisting of `n` nodes numbered from `0` to `n - 1` and `n` directed edges.
///
/// You are given a **0-indexed** array `edges` where `edges[i]` indicates that there is an edge from node `i` to node `edges[i]`.
///
/// Consider the following process on the graph:
///
/// * You start from a node `x` and keep visiting other nodes through edges until you reach a node that you have already visited before on this **same** process.
///
/// Return *an array* `answer` *where* `answer[i]` *is the number of **different** nodes that you will visit if you perform the process starting from node* `i`.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2023/08/31/graaphdrawio-1.png)
///
/// ```
/// Input: edges = [1,2,0,0]
/// Output: [3,3,3,4]
/// Explanation: We perform the process starting from each node in the following way:
/// - Starting from node 0, we visit the nodes 0 -> 1 -> 2 -> 0. The number of different nodes we visit is 3.
/// - Starting from node 1, we visit the nodes 1 -> 2 -> 0 -> 1. The number of different nodes we visit is 3.
/// - Starting from node 2, we visit the nodes 2 -> 0 -> 1 -> 2. The number of different nodes we visit is 3.
/// - Starting from node 3, we visit the nodes 3 -> 0 -> 1 -> 2 -> 0. The number of different nodes we visit is 4.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2023/08/31/graaph2drawio.png)
///
/// ```
/// Input: edges = [1,2,3,4,0]
/// Output: [5,5,5,5,5]
/// Explanation: Starting from any node we can visit every node in the graph in the process.
///
/// ```
///
/// **Constraints:**
///
/// * `n == edges.length`
/// * `2 <= n <= 10<sup>5</sup>`
/// * `0 <= edges[i] <= n - 1`
/// * `edges[i] != i`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/count-visited-nodes-in-a-directed-graph/
// discuss: https://leetcode.com/problems/count-visited-nodes-in-a-directed-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_visited_nodes(edges: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        let mut cache = vec![None; edges.len()];
        let mut visited = vec![false; edges.len()];
        let mut path = vec![];

        for i in 0..edges.len() {
            if let Some(x) = cache[i] {
                result.push(x);
                continue;
            }

            let mut cur = i;

            loop {
                if visited[cur] {
                    let mut cycle = vec![];

                    while let Some(x) = path.pop() {
                        cycle.push(x);
                        visited[x] = false;

                        if x == cur {
                            break;
                        }
                    }

                    let cycle_count = cycle.len() as i32;

                    for c in cycle {
                        cache[c] = Some(cycle_count);
                    }
                }

                if let Some(mut cur_count) = cache[cur] {
                    while let Some(x) = path.pop() {
                        cur_count += 1;
                        cache[x] = Some(cur_count);
                        visited[x] = false;
                    }

                    result.push(cur_count);
                    break;
                }

                visited[cur] = true;
                path.push(cur);
                cur = edges[cur] as usize;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2876() {
        let edges = vec![1, 2, 0, 0];
        let expected = vec![3, 3, 3, 4];
        assert_eq!(Solution::count_visited_nodes(edges), expected);
        let edges = vec![1, 2, 3, 4, 0];
        let expected = vec![5, 5, 5, 5, 5];
        assert_eq!(Solution::count_visited_nodes(edges), expected);
    }
}
