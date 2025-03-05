///
/// # 3444. Minimum Increments for Target Multiples in an Array
///
/// You are given two arrays, `nums` and `target`.
///
/// In a single operation, you may increment any element of `nums` by 1.
///
/// Return **the minimum number** of operations required so that each element in `target` has **at least** one multiple in `nums`.
///
/// **Example 1:**
///
/// **Input:** nums = [1,2,3], target = [4]
///
/// **Output:** 1
///
/// **Explanation:**
///
/// The minimum number of operations required to satisfy the condition is 1.
///
/// * Increment 3 to 4 with just one operation, making 4 a multiple of itself.
///
/// **Example 2:**
///
/// **Input:** nums = [8,4], target = [10,5]
///
/// **Output:** 2
///
/// **Explanation:**
///
/// The minimum number of operations required to satisfy the condition is 2.
///
/// * Increment 8 to 10 with 2 operations, making 10 a multiple of both 5 and 10.
///
/// **Example 3:**
///
/// **Input:** nums = [7,9,10], target = [7]
///
/// **Output:** 0
///
/// **Explanation:**
///
/// Target 7 already has a multiple in nums, so no additional operations are needed.
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 5 * 10<sup>4</sup>`
/// * `1 <= target.length <= 4`
/// * `target.length <= nums.length`
/// * `1 <= nums[i], target[i] <= 10<sup>4</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-increments-for-target-multiples-in-an-array/
// discuss: https://leetcode.com/problems/minimum-increments-for-target-multiples-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_increments(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if a % b == 0 {
                return b;
            }

            gcd(b, a % b)
        }

        let mut lcm_list = vec![];

        let mut target = target;
        target.sort_unstable();
        target.dedup();

        'outer: for x in 0..(1 << target.len()) {
            let mut lcm = 1;

            for (i, &t) in target.iter().enumerate() {
                if x & 1 << i > 0 {
                    let gcd = gcd(lcm, t);
                    lcm = lcm * t / gcd;

                    if lcm > 200000 {
                        lcm_list.push(None);
                        continue 'outer;
                    }
                }
            }

            lcm_list.push(Some(lcm));
        }

        let mut cache = vec![vec![i32::MAX; 1 << target.len()]; nums.len() + 1];
        cache[0][0] = 0;

        for (i, &num) in nums.iter().enumerate() {
            for (cur, &lcm) in lcm_list.iter().enumerate() {
                let lcm = match lcm {
                    Some(x) => x,
                    None => continue,
                };

                let increment = lcm - ((num - 1) % lcm + 1);

                for prev in 0..(1 << target.len()) {
                    let prev_increment = cache[i][prev];

                    if prev_increment == i32::MAX {
                        continue;
                    }

                    cache[i + 1][prev | cur] =
                        cache[i + 1][prev | cur].min(prev_increment.wrapping_add(increment));
                }
            }
        }

        cache[nums.len()][(1 << target.len()) - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3444() {
        // let nums = vec![1, 2, 3];
        // let target = vec![4];
        // let expected = 1;
        // assert_eq!(Solution::minimum_increments(nums, target), expected);
        // let nums = vec![8, 4];
        // let target = vec![10, 5];
        // let expected = 2;
        // assert_eq!(Solution::minimum_increments(nums, target), expected);
        // let nums = vec![7, 9, 10];
        // let target = vec![7];
        // let expected = 0;
        // assert_eq!(Solution::minimum_increments(nums, target), expected);
        let nums = vec![
            74, 19, 60, 20, 38, 2, 77, 22, 3, 98, 73, 49, 68, 42, 9, 19, 94, 69, 6, 73, 47, 24, 6,
            43, 76, 93, 57, 62, 73, 100, 19, 26, 73, 54, 14, 75, 39, 94, 72, 42, 55, 77, 81, 3, 75,
            83, 64, 89, 57, 49, 12, 1, 89, 38, 36, 1, 56, 84, 55, 37, 30, 68, 39, 36, 4, 53, 23,
            53, 72, 10, 81, 35, 5, 55, 64, 92, 84, 84, 54, 31, 83, 42, 50, 99, 45, 77, 20, 96, 36,
            60, 5, 85, 7, 88, 87, 96, 72, 73,
        ];
        let target = vec![5480, 9819, 6781];
        let expected = 21783;
        assert_eq!(Solution::minimum_increments(nums, target), expected);
    }
}
