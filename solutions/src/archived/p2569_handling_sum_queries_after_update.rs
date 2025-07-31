///
/// # 2569. Handling Sum Queries After Update
///
/// You are given two **0-indexed** arrays `nums1` and `nums2` and a 2D array `queries` of queries. There are three types of queries:
///
/// 1. For a query of type 1, `queries[i] = [1, l, r]`. Flip the values from `0` to `1` and from `1` to `0` in `nums1` from index `l` to index `r`. Both `l` and `r` are **0-indexed**.
/// 2. For a query of type 2, `queries[i] = [2, p, 0]`. For every index `0 <= i < n`, set `nums2[i] = nums2[i] + nums1[i] * p`.
/// 3. For a query of type 3, `queries[i] = [3, 0, 0]`. Find the sum of the elements in `nums2`.
///
/// Return *an array containing all the answers to the third type queries.*
///
/// **Example 1:**
///
/// ```
/// Input: nums1 = [1,0,1], nums2 = [0,0,0], queries = [[1,1,1],[2,1,0],[3,0,0]]
/// Output: [3]
/// Explanation: After the first query nums1 becomes [1,1,1]. After the second query, nums2 becomes [1,1,1], so the answer to the third query is 3. Thus, [3] is returned.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums1 = [1], nums2 = [5], queries = [[2,0,0],[3,0,0]]
/// Output: [5]
/// Explanation: After the first query, nums2 remains [5], so the answer to the second query is 5. Thus, [5] is returned.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums1.length,nums2.length <= 10<sup>5</sup>`
/// * `nums1.length = nums2.length`
/// * `1 <= queries.length <= 10<sup>5</sup>`
/// * `queries[i].length = 3`
/// * `0 <= l <= r <= nums1.length - 1`
/// * `0 <= p <= 10<sup>6</sup>`
/// * `0 <= nums1[i] <= 1`
/// * `0 <= nums2[i] <= 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/handling-sum-queries-after-update/
// discuss: https://leetcode.com/problems/handling-sum-queries-after-update/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

struct SegTree {
    n: usize,
    data: Vec<i64>,
    need_flip: Vec<bool>,
}

impl SegTree {
    fn build(n: usize, nums: &[i32]) -> Self {
        let mut seg_tree = SegTree {
            n,
            data: vec![0; 4 * n],
            need_flip: vec![false; 4 * n],
        };

        seg_tree._build(1, 0, n - 1, nums);

        seg_tree
    }

    fn _build(&mut self, node: usize, left: usize, right: usize, nums: &[i32]) {
        if left == right {
            self.data[node] = nums[left] as i64;
            return;
        }

        let mid = (left + right) / 2;

        self._build(2 * node, left, mid, nums);
        self._build(2 * node + 1, mid + 1, right, nums);

        self.data[node] = self.data[2 * node] + self.data[2 * node + 1];
    }

    fn update(&mut self, node: usize, left: usize, right: usize, start: usize, end: usize) {
        if left == right {
            self.data[node] ^= 1;
            return;
        }

        if left >= start && right <= end {
            self.need_flip[node] = !self.need_flip[node];
            self.data[node] = (right + 1 - left) as i64 - self.data[node];
            return;
        }

        let mid = (left + right) / 2;

        if self.need_flip[node] {
            self.need_flip[node] = false;

            self.need_flip[2 * node] = !self.need_flip[2 * node];
            self.data[2 * node] = (mid + 1 - left) as i64 - self.data[2 * node];

            self.need_flip[2 * node + 1] = !self.need_flip[2 * node + 1];
            self.data[2 * node + 1] = (right - mid) as i64 - self.data[2 * node + 1];
        }

        if start <= mid {
            self.update(2 * node, left, mid, start, end);
        }

        if end > mid {
            self.update(2 * node + 1, mid + 1, right, start, end);
        }

        self.data[node] = self.data[2 * node] + self.data[2 * node + 1];
    }

    fn sum(&self) -> i64 {
        self.data[1]
    }
}

impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = nums1.len();

        let mut nums1_seg_tree = SegTree::build(n, &nums1);
        let mut nums2_sum = nums2.iter().map(|&x| x as i64).sum::<i64>();

        let mut result = vec![];

        for query in queries {
            match query[..] {
                [1, l, r] => nums1_seg_tree.update(1, 0, n - 1, l as usize, r as usize),
                [2, p, _] => nums2_sum += nums1_seg_tree.sum() * p as i64,
                [3, _, _] => result.push(nums2_sum),
                _ => unreachable!(),
            };
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2569() {
        let nums1 = vec![1, 0, 1];
        let nums2 = vec![0, 0, 0];
        let queries = nd_vec![[1, 1, 1], [2, 1, 0], [3, 0, 0]];
        let expected = vec![3];
        assert_eq!(Solution::handle_query(nums1, nums2, queries), expected);
        let nums1 = vec![1];
        let nums2 = vec![5];
        let queries = nd_vec![[2, 0, 0], [3, 0, 0]];
        let expected = vec![5];
        assert_eq!(Solution::handle_query(nums1, nums2, queries), expected);
    }
}
