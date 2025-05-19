///
/// # 1627. Graph Connectivity With Threshold
///
/// We have `n` cities labeled from `1` to `n`. Two different cities with labels `x` and `y` are directly connected by a bidirectional road if and only if `x` and `y` share a common divisor **strictly greater** than some `threshold`. More formally, cities with labels `x` and `y` have a road between them if there exists an integer `z` such that all of the following are true:
///
/// * `x % z == 0`,
/// * `y % z == 0`, and
/// * `z > threshold`.
///
/// Given the two integers, `n` and `threshold`, and an array of `queries`, you must determine for each `queries[i] = [a<sub>i</sub>, b<sub>i</sub>]` if cities `a<sub>i</sub>` and `b<sub>i</sub>` are connected directly or indirectly. (i.e. there is some path between them).
///
/// Return *an array* `answer`*, where* `answer.length == queries.length` *and* `answer[i]` *is* `true` *if for the* `i<sup>th</sup>` *query, there is a path between* `a<sub>i</sub>` *and* `b<sub>i</sub>`*, or* `answer[i]` *is* `false` *if there is no path.*
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/10/09/ex1.jpg)
///
/// ```
/// Input: n = 6, threshold = 2, queries = [[1,4],[2,5],[3,6]]
/// Output: [false,false,true]
/// Explanation: The divisors for each number:
/// 1:   1
/// 2:   1, 2
/// 3:   1, 3
/// 4:   1, 2, 4
/// 5:   1, 5
/// 6:   1, 2, 3, 6
/// Using the underlined divisors above the threshold, only cities 3 and 6 share a common divisor, so they are the
/// only ones directly connected. The result of each query:
/// [1,4]   1 is not connected to 4
/// [2,5]   2 is not connected to 5
/// [3,6]   3 is connected to 6 through path 3--6
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/10/10/tmp.jpg)
///
/// ```
/// Input: n = 6, threshold = 0, queries = [[4,5],[3,4],[3,2],[2,6],[1,3]]
/// Output: [true,true,true,true,true]
/// Explanation: The divisors for each number are the same as the previous example. However, since the threshold is 0,
/// all divisors can be used. Since all numbers share 1 as a divisor, all cities are connected.
///
/// ```
///
/// **Example 3:**
///
/// ![](https://assets.leetcode.com/uploads/2020/10/17/ex3.jpg)
///
/// ```
/// Input: n = 5, threshold = 1, queries = [[4,5],[4,5],[3,2],[2,3],[3,4]]
/// Output: [false,false,false,false,false]
/// Explanation: Only cities 2 and 4 share a common divisor 2 which is strictly greater than the threshold 1, so they are the only ones directly connected.
/// Please notice that there can be multiple queries for the same pair of nodes [x, y], and that the query [x, y] is equivalent to the query [y, x].
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= n <= 10<sup>4</sup>`
/// * `0 <= threshold <= n`
/// * `1 <= queries.length <= 10<sup>5</sup>`
/// * `queries[i].length == 2`
/// * `1 <= a<sub>i</sub>, b<sub>i</sub> <= cities`
/// * `a<sub>i</sub> != b<sub>i</sub>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/graph-connectivity-with-threshold/
// discuss: https://leetcode.com/problems/graph-connectivity-with-threshold/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Debug)]
struct UnionFind {
    data: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            data: (0..=n).collect(),
            size: vec![1; n + 1],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if a == self.data[a] {
            return a;
        }

        self.data[a] = self.find(self.data[a]);

        self.data[a]
    }

    fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return;
        }

        let (a, b) = match self.size[a].cmp(&self.size[b]) {
            std::cmp::Ordering::Less => (b, a),
            _ => (a, b),
        };

        self.size[b] += self.size[a];
        self.data[a] = b;
    }
}

impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;

        let mut sieve = vec![false; n + 1];
        let mut uf = UnionFind::new(n);

        for x in threshold as usize + 1..=n / 2 {
            if !sieve[x] {
                for y in (2 * x..=n).step_by(x) {
                    sieve[y] = true;
                    uf.union(x, y);
                }
            }
        }

        queries
            .iter()
            .map(|x| uf.find(x[0] as usize) == uf.find(x[1] as usize))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1627() {
        let n = 6;
        let threshold = 2;
        let queries = nd_vec![[1, 4], [2, 5], [3, 6]];
        let expected = vec![false, false, true];
        assert_eq!(Solution::are_connected(n, threshold, queries), expected);
        let n = 6;
        let threshold = 0;
        let queries = nd_vec![[4, 5], [3, 4], [3, 2], [2, 6], [1, 3]];
        let expected = vec![true, true, true, true, true];
        assert_eq!(Solution::are_connected(n, threshold, queries), expected);
        let n = 5;
        let threshold = 1;
        let queries = nd_vec![[4, 5], [4, 5], [3, 2], [2, 3], [3, 4]];
        let expected = vec![false, false, false, false, false];
        assert_eq!(Solution::are_connected(n, threshold, queries), expected);
    }
}
