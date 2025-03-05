///
/// # 1352. Product of the Last K Numbers
///
/// Design an algorithm that accepts a stream of integers and retrieves the product of the last `k` integers of the stream.
///
/// Implement the `ProductOfNumbers` class:
///
/// * `ProductOfNumbers()` Initializes the object with an empty stream.
/// * `void add(int num)` Appends the integer `num` to the stream.
/// * `int getProduct(int k)` Returns the product of the last `k` numbers in the current list. You can assume that always the current list has at least `k` numbers.
///
/// The test cases are generated so that, at any time, the product of any contiguous sequence of numbers will fit into a single 32-bit integer without overflowing.
///
/// **Example:**
///
/// ```
/// Input
/// ["ProductOfNumbers","add","add","add","add","add","getProduct","getProduct","getProduct","add","getProduct"]
/// [[],[3],[0],[2],[5],[4],[2],[3],[4],[8],[2]]
///
/// Output
/// [null,null,null,null,null,null,20,40,0,null,32]
///
/// Explanation
/// ProductOfNumbers productOfNumbers = new ProductOfNumbers();
/// productOfNumbers.add(3);        // [3]
/// productOfNumbers.add(0);        // [3,0]
/// productOfNumbers.add(2);        // [3,0,2]
/// productOfNumbers.add(5);        // [3,0,2,5]
/// productOfNumbers.add(4);        // [3,0,2,5,4]
/// productOfNumbers.getProduct(2); // return 20. The product of the last 2 numbers is 5 * 4 = 20
/// productOfNumbers.getProduct(3); // return 40. The product of the last 3 numbers is 2 * 5 * 4 = 40
/// productOfNumbers.getProduct(4); // return 0. The product of the last 4 numbers is 0 * 2 * 5 * 4 = 0
/// productOfNumbers.add(8);        // [3,0,2,5,4,8]
/// productOfNumbers.getProduct(2); // return 32. The product of the last 2 numbers is 4 * 8 = 32
///
/// ```
///
/// **Constraints:**
///
/// * `0 <= num <= 100`
/// * `1 <= k <= 4 * 10<sup>4</sup>`
/// * At most `4 * 10<sup>4</sup>` calls will be made to `add` and `getProduct`.
/// * The product of the stream at any point in time will fit in a **32-bit** integer.
///
/// **Follow-up:** Can you implement **both** `GetProduct` and `Add` to work in `O(1)` time complexity instead of `O(k)` time complexity?
///
pub struct Solution {}

// problem: https://leetcode.com/problems/product-of-the-last-k-numbers/
// discuss: https://leetcode.com/problems/product-of-the-last-k-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct ProductOfNumbers {
    prefix_product: Vec<i32>,
}

#[allow(dead_code)]
impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            prefix_product: vec![1],
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix_product.clear();
            self.prefix_product.push(1);
            return;
        }

        self.prefix_product
            .push(self.prefix_product.last().unwrap() * num);
    }

    fn get_product(&self, k: i32) -> i32 {
        let kth = match (self.prefix_product.len() - 1).checked_sub(k as usize) {
            Some(x) => x,
            None => return 0,
        };

        self.prefix_product.last().unwrap() / self.prefix_product[kth]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1352() {
        let mut obj = ProductOfNumbers::new();
        obj.add(3);
        obj.add(0);
        obj.add(2);
        obj.add(5);
        obj.add(4);
        assert_eq!(obj.get_product(2), 20);
        assert_eq!(obj.get_product(3), 40);
        assert_eq!(obj.get_product(4), 0);
        obj.add(8);
        assert_eq!(obj.get_product(2), 32);
    }
}
