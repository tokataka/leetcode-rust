use std::collections::HashMap;

///
/// # 1726. Tuple with Same Product
///
/// Given an array `nums` of **distinct** positive integers, return *the number of tuples* `(a, b, c, d)` *such that* `a * b = c * d` *where* `a`*,* `b`*,* `c`*, and* `d` *are elements of* `nums`*, and* `a != b != c != d`*.*
///
/// **Example 1:**
///
/// ```
/// Input: nums = [2,3,4,6]
/// Output: 8
/// Explanation: There are 8 valid tuples:
/// (2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
/// (3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: nums = [1,2,4,5,10]
/// Output: 16
/// Explanation: There are 16 valid tuples:
/// (1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
/// (2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
/// (2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,5,4)
/// (4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= nums.length <= 1000`
/// * `1 <= nums[i] <= 10<sup>4</sup>`
/// * All elements in `nums` are **distinct**.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/tuple-with-same-product/
// discuss: https://leetcode.com/problems/tuple-with-same-product/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut product_count: HashMap<i32, i32> = HashMap::new();

        for (i, &a) in nums.iter().enumerate() {
            for b in nums.iter().skip(i + 1) {
                product_count
                    .entry(a * b)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }

        product_count
            .into_values()
            .filter(|&x| x > 1)
            .map(|x| 4 * x * (x - 1))
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1726() {
        let nums = vec![2, 3, 4, 6];
        let expected = 8;
        assert_eq!(Solution::tuple_same_product(nums), expected);
        let nums = vec![1, 2, 4, 5, 10];
        let expected = 16;
        assert_eq!(Solution::tuple_same_product(nums), expected);
    }
}
