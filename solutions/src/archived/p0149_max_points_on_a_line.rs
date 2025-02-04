use std::collections::HashMap;

///
/// # 149. Max Points on a Line
///
/// Given an array of `points` where `points[i] = [x<sub>i</sub>, y<sub>i</sub>]` represents a point on the **X-Y** plane, return *the maximum number of points that lie on the same straight line*.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2021/02/25/plane1.jpg)
///
/// ```
/// Input: points = [[1,1],[2,2],[3,3]]
/// Output: 3
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2021/02/25/plane2.jpg)
///
/// ```
/// Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
/// Output: 4
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= points.length <= 300`
/// * `points[i].length == 2`
/// * `-10<sup>4</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup>`
/// * All the `points` are **unique**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/max-points-on-a-line/
// discuss: https://leetcode.com/problems/max-points-on-a-line/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Debug)]
struct UnionFind {
    pub data: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::from_iter(0..size),
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        if self.data[a] == a {
            return a;
        }

        self.data[a] = self.find(self.data[a]);

        self.data[a]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);

        if a != b {
            self.data[b] = a;
        }
    }

    pub fn max_count(&self) -> i32 {
        let mut count = vec![0; self.data.len()];

        for &x in &self.data {
            let mut x = x;

            let x = loop {
                if self.data[x] == x {
                    break x;
                }

                x = self.data[x];
            };

            count[x] += 1;
        }

        count.into_iter().max().unwrap()
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut slope_map: HashMap<(i32, i32), UnionFind> = HashMap::new();

        fn gcd(a: i32, b: i32) -> i32 {
            if a % b == 0 {
                return b;
            }

            gcd(b, a % b)
        }

        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points.iter().enumerate().skip(i + 1) {
                let dx = p1[0] - p2[0];
                let dy = p1[1] - p2[1];

                let (dx, dy) = match (dx, dy) {
                    (0, _) => (0, 1),
                    (_, 0) => (1, 0),
                    (dx, dy) => {
                        let slope_gcd = gcd(dx, dy);
                        (dx / slope_gcd, dy / slope_gcd)
                    }
                };

                slope_map
                    .entry((dx, dy))
                    .or_insert(UnionFind::new(points.len()))
                    .union(i, j);
            }
        }

        slope_map.values().map(|x| x.max_count()).max().unwrap_or(1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_149() {
        let points = nd_vec![[1, 1], [2, 2], [3, 3]];
        let expected = 3;
        assert_eq!(Solution::max_points(points), expected);
        let points = nd_vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
        let expected = 4;
        assert_eq!(Solution::max_points(points), expected);
    }
}
