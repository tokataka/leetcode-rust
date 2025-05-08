///
/// # 403. Frog Jump
///
/// A frog is crossing a river. The river is divided into some number of units, and at each unit, there may or may not exist a stone. The frog can jump on a stone, but it must not jump into the water.
///
/// Given a list of `stones` positions (in units) in sorted **ascending order**, determine if the frog can cross the river by landing on the last stone. Initially, the frog is on the first stone and assumes the first jump must be `1` unit.
///
/// If the frog's last jump was `k` units, its next jump must be either `k - 1`, `k`, or `k + 1` units. The frog can only jump in the forward direction.
///
/// **Example 1:**
///
/// ```
/// Input: stones = [0,1,3,5,6,8,12,17]
/// Output: true
/// Explanation: The frog can jump to the last stone by jumping 1 unit to the 2nd stone, then 2 units to the 3rd stone, then 2 units to the 4th stone, then 3 units to the 6th stone, 4 units to the 7th stone, and 5 units to the 8th stone.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: stones = [0,1,2,3,4,8,9,11]
/// Output: false
/// Explanation: There is no way to jump to the last stone as the gap between the 5th and 6th stone is too large.
///
/// ```
///
/// **Constraints:**
///
/// * `2 <= stones.length <= 2000`
/// * `0 <= stones[i] <= 2<sup>31</sup> - 1`
/// * `stones[0] == 0`
/// * `stones` is sorted in a strictly increasing order.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/frog-jump/
// discuss: https://leetcode.com/problems/frog-jump/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones[1] != 1 {
            return false;
        }

        fn _can_cross(
            stones: &Vec<i32>,
            prev: usize,
            cur: usize,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if cur == stones.len() - 1 {
                return true;
            }

            if visited[prev][cur] {
                return false;
            }

            let diff = stones[cur] - stones[prev];

            for d in (diff - 1).max(1)..=diff + 1 {
                if let Ok(idx) = (stones[cur..]).binary_search(&(stones[cur] + d)) {
                    if _can_cross(stones, cur, cur + idx, visited) {
                        return true;
                    }
                }
            }

            visited[prev][cur] = true;

            false
        }

        _can_cross(
            &stones,
            0,
            1,
            &mut vec![vec![false; stones.len()]; stones.len()],
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_403() {
        let stones = vec![0, 1, 3, 5, 6, 8, 12, 17];
        let expected = true;
        assert_eq!(Solution::can_cross(stones), expected);
        let stones = vec![0, 1, 2, 3, 4, 8, 9, 11];
        let expected = false;
        assert_eq!(Solution::can_cross(stones), expected);
    }
}
