use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

///
/// # 2349. Design a Number Container System
///
/// Design a number container system that can do the following:
///
/// * **Insert** or **Replace** a number at the given index in the system.
/// * **Return** the smallest index for the given number in the system.
///
/// Implement the `NumberContainers` class:
///
/// * `NumberContainers()` Initializes the number container system.
/// * `void change(int index, int number)` Fills the container at `index` with the `number`. If there is already a number at that `index`, replace it.
/// * `int find(int number)` Returns the smallest index for the given `number`, or `-1` if there is no index that is filled by `number` in the system.
///
/// **Example 1:**
///
/// ```
/// Input
/// ["NumberContainers", "find", "change", "change", "change", "change", "find", "change", "find"]
/// [[], [10], [2, 10], [1, 10], [3, 10], [5, 10], [10], [1, 20], [10]]
/// Output
/// [null, -1, null, null, null, null, 1, null, 2]
///
/// Explanation
/// NumberContainers nc = new NumberContainers();
/// nc.find(10); // There is no index that is filled with number 10. Therefore, we return -1.
/// nc.change(2, 10); // Your container at index 2 will be filled with number 10.
/// nc.change(1, 10); // Your container at index 1 will be filled with number 10.
/// nc.change(3, 10); // Your container at index 3 will be filled with number 10.
/// nc.change(5, 10); // Your container at index 5 will be filled with number 10.
/// nc.find(10); // Number 10 is at the indices 1, 2, 3, and 5. Since the smallest index that is filled with 10 is 1, we return 1.
/// nc.change(1, 20); // Your container at index 1 will be filled with number 20. Note that index 1 was filled with 10 and then replaced with 20.
/// nc.find(10); // Number 10 is at the indices 2, 3, and 5. The smallest index that is filled with 10 is 2. Therefore, we return 2.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= index, number <= 10<sup>9</sup>`
/// * At most `10<sup>5</sup>` calls will be made **in total** to `change` and `find`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/design-a-number-container-system/
// discuss: https://leetcode.com/problems/design-a-number-container-system/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumberContainers {
    number_map: HashMap<i32, i32>,
    pq_map: HashMap<i32, BinaryHeap<Reverse<i32>>>,
    invalidated: HashSet<(i32, i32)>,
}

#[allow(dead_code)]
impl NumberContainers {
    fn new() -> Self {
        Self {
            number_map: HashMap::new(),
            pq_map: HashMap::new(),
            invalidated: HashSet::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&prev_number) = self.number_map.get(&index) {
            self.invalidated.insert((index, prev_number));
        }

        self.number_map.insert(index, number);
        self.invalidated.remove(&(index, number));
        self.pq_map.entry(number).or_default().push(Reverse(index));
    }

    fn find(&mut self, number: i32) -> i32 {
        if let Some(pq) = self.pq_map.get_mut(&number) {
            while let Some(&Reverse(top)) = pq.peek() {
                if self.invalidated.contains(&(top, number)) {
                    pq.pop();
                } else {
                    return top;
                }
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2349() {
        let mut obj = NumberContainers::new();
        assert_eq!(obj.find(10), -1);
        obj.change(2, 10);
        obj.change(1, 10);
        obj.change(3, 10);
        obj.change(5, 10);
        assert_eq!(obj.find(10), 1);
        obj.change(1, 20);
        assert_eq!(obj.find(10), 2);
    }
}
