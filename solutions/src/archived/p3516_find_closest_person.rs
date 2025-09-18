///
/// # 3516. Find Closest Person
///
/// You are given three integers `x`, `y`, and `z`, representing the positions of three people on a number line:
///
/// * `x` is the position of Person 1.
/// * `y` is the position of Person 2.
/// * `z` is the position of Person 3, who does **not** move.
///
/// Both Person 1 and Person 2 move toward Person 3 at the **same** speed.
///
/// Determine which person reaches Person 3 **first**:
///
/// * Return 1 if Person 1 arrives first.
/// * Return 2 if Person 2 arrives first.
/// * Return 0 if both arrive at the **same** time.
///
/// Return the result accordingly.
///
/// **Example 1:**
///
/// **Input:** x = 2, y = 7, z = 4
///
/// **Output:** 1
///
/// **Explanation:**
///
/// * Person 1 is at position 2 and can reach Person 3 (at position 4) in 2 steps.
/// * Person 2 is at position 7 and can reach Person 3 in 3 steps.
///
/// Since Person 1 reaches Person 3 first, the output is 1.
///
/// **Example 2:**
///
/// **Input:** x = 2, y = 5, z = 6
///
/// **Output:** 2
///
/// **Explanation:**
///
/// * Person 1 is at position 2 and can reach Person 3 (at position 6) in 4 steps.
/// * Person 2 is at position 5 and can reach Person 3 in 1 step.
///
/// Since Person 2 reaches Person 3 first, the output is 2.
///
/// **Example 3:**
///
/// **Input:** x = 1, y = 5, z = 3
///
/// **Output:** 0
///
/// **Explanation:**
///
/// * Person 1 is at position 1 and can reach Person 3 (at position 3) in 2 steps.
/// * Person 2 is at position 5 and can reach Person 3 in 2 steps.
///
/// Since both Person 1 and Person 2 reach Person 3 at the same time, the output is 0.
///
/// **Constraints:**
///
/// * `1 <= x, y, z <= 100`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/find-closest-person/
// discuss: https://leetcode.com/problems/find-closest-person/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match x.abs_diff(z).cmp(&y.abs_diff(z)) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 2,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3516() {
        let x = 2;
        let y = 7;
        let z = 4;
        let expected = 1;
        assert_eq!(Solution::find_closest(x, y, z), expected);
        let x = 2;
        let y = 5;
        let z = 6;
        let expected = 2;
        assert_eq!(Solution::find_closest(x, y, z), expected);
        let x = 1;
        let y = 5;
        let z = 3;
        let expected = 0;
        assert_eq!(Solution::find_closest(x, y, z), expected);
    }
}
