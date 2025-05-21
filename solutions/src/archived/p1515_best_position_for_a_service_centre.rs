///
/// # 1515. Best Position for a Service Centre
///
/// A delivery company wants to build a new service center in a new city. The company knows the positions of all the customers in this city on a 2D-Map and wants to build the new center in a position such that **the sum of the euclidean distances to all customers is minimum**.
///
/// Given an array `positions` where `positions[i] = [x<sub>i</sub>, y<sub>i</sub>]` is the position of the `ith` customer on the map, return *the minimum sum of the euclidean distances* to all customers.
///
/// In other words, you need to choose the position of the service center `[x<sub>centre</sub>, y<sub>centre</sub>]` such that the following formula is minimized:
///
/// ![](https://assets.leetcode.com/uploads/2020/06/25/q4_edited.jpg)
///
/// Answers within `10<sup>-5</sup>` of the actual value will be accepted.
///
/// **Example 1:**
///
/// ![](https://assets.leetcode.com/uploads/2020/06/25/q4_e1.jpg)
///
/// ```
/// Input: positions = [[0,1],[1,0],[1,2],[2,1]]
/// Output: 4.00000
/// Explanation: As shown, you can see that choosing [xcentre, ycentre] = [1, 1] will make the distance to each customer = 1, the sum of all distances is 4 which is the minimum possible we can achieve.
///
/// ```
///
/// **Example 2:**
///
/// ![](https://assets.leetcode.com/uploads/2020/06/25/q4_e3.jpg)
///
/// ```
/// Input: positions = [[1,1],[3,3]]
/// Output: 2.82843
/// Explanation: The minimum possible sum of distances = sqrt(2) + sqrt(2) = 2.82843
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= positions.length <= 50`
/// * `positions[i].length == 2`
/// * `0 <= x<sub>i</sub>, y<sub>i</sub> <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/best-position-for-a-service-centre/
// discuss: https://leetcode.com/problems/best-position-for-a-service-centre/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let positions = positions
            .into_iter()
            .map(|x| (x[0] as f64, x[1] as f64))
            .collect::<Vec<_>>();

        fn sum_distances_square(cur_pos: (f64, f64), positions: &[(f64, f64)]) -> f64 {
            positions
                .iter()
                .map(|p| ((p.0 - cur_pos.0).powi(2) + (p.1 - cur_pos.1).powi(2)).sqrt())
                .sum()
        }

        let mut cur_pos = (0.0, 0.0);
        let mut delta = 100.0;
        let mut cur_distances = sum_distances_square(cur_pos, &positions);

        while delta > 0.0000001 {
            let mut next_pos = cur_pos;
            let mut next_distances = cur_distances;

            for (dx, dy) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
                let new_pos = (cur_pos.0 + dx * delta, cur_pos.1 + dy * delta);
                let new_distances = sum_distances_square(new_pos, &positions);

                if new_distances < next_distances {
                    next_pos = new_pos;
                    next_distances = new_distances;
                }
            }

            if cur_pos == next_pos {
                delta /= 2.0;
            } else {
                cur_pos = next_pos;
                cur_distances = next_distances;
            }
        }

        cur_distances
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1515() {
        let positions = nd_vec![[0, 1], [1, 0], [1, 2], [2, 1]];
        let expected = 4.00000;
        assert_eq!(Solution::get_min_dist_sum(positions), expected);
        let positions = nd_vec![[1, 1], [3, 3]];
        let expected = 2.82843;
        assert_eq!(Solution::get_min_dist_sum(positions), expected);
    }
}
