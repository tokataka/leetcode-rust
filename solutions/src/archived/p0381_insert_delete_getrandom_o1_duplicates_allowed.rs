///
/// # 381. Insert Delete GetRandom O(1) - Duplicates allowed
///
/// `RandomizedCollection` is a data structure that contains a collection of numbers, possibly duplicates (i.e., a multiset). It should support inserting and removing specific elements and also reporting a random element.
///
/// Implement the `RandomizedCollection` class:
///
/// * `RandomizedCollection()` Initializes the empty `RandomizedCollection` object.
/// * `bool insert(int val)` Inserts an item `val` into the multiset, even if the item is already present. Returns `true` if the item is not present, `false` otherwise.
/// * `bool remove(int val)` Removes an item `val` from the multiset if present. Returns `true` if the item is present, `false` otherwise. Note that if `val` has multiple occurrences in the multiset, we only remove one of them.
/// * `int getRandom()` Returns a random element from the current multiset of elements. The probability of each element being returned is **linearly related** to the number of the same values the multiset contains.
///
/// You must implement the functions of the class such that each function works on **average** `O(1)` time complexity.
///
/// **Note:** The test cases are generated such that `getRandom` will only be called if there is **at least one** item in the `RandomizedCollection`.
///
/// **Example 1:**
///
/// ```
/// Input
/// ["RandomizedCollection", "insert", "insert", "insert", "getRandom", "remove", "getRandom"]
/// [[], [1], [1], [2], [], [1], []]
/// Output
/// [null, true, false, true, 2, true, 1]
///
/// Explanation
/// RandomizedCollection randomizedCollection = new RandomizedCollection();
/// randomizedCollection.insert(1);   // return true since the collection does not contain 1.
///                                   // Inserts 1 into the collection.
/// randomizedCollection.insert(1);   // return false since the collection contains 1.
///                                   // Inserts another 1 into the collection. Collection now contains [1,1].
/// randomizedCollection.insert(2);   // return true since the collection does not contain 2.
///                                   // Inserts 2 into the collection. Collection now contains [1,1,2].
/// randomizedCollection.getRandom(); // getRandom should:
///                                   // - return 1 with probability 2/3, or
///                                   // - return 2 with probability 1/3.
/// randomizedCollection.remove(1);   // return true since the collection contains 1.
///                                   // Removes 1 from the collection. Collection now contains [1,2].
/// randomizedCollection.getRandom(); // getRandom should return 1 or 2, both equally likely.
///
/// ```
///
/// **Constraints:**
///
/// * `-2<sup>31</sup> <= val <= 2<sup>31</sup> - 1`
/// * At most `2 * 10<sup>5</sup>` calls **in total** will be made to `insert`, `remove`, and `getRandom`.
/// * There will be **at least one** element in the data structure when `getRandom` is called.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/
// discuss: https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    val_idx: HashMap<i32, HashSet<usize>>,
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl RandomizedCollection {
    fn new() -> Self {
        Self {
            val_idx: HashMap::new(),
            data: Vec::with_capacity(200_000),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let idx = self.data.len();

        self.data.push(val);

        if let Some(x) = self.val_idx.get_mut(&val) {
            x.insert(idx);

            false
        } else {
            self.val_idx.insert(val, HashSet::from([idx]));

            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.data.is_empty() {
            return false;
        }

        let remove_idx;

        let swap_idx = self.data.len() - 1;
        let swap_val = self.data[swap_idx];

        if swap_val == val {
            remove_idx = swap_idx;

            self.data.pop().unwrap();
        } else {
            remove_idx = if let Some(x) = self.val_idx.get(&val) {
                *x.iter().next().unwrap()
            } else {
                return false;
            };

            self.data[remove_idx] = self.data.pop().unwrap();

            if let Some(s) = self.val_idx.get_mut(&swap_val) {
                s.remove(&swap_idx);
                s.insert(remove_idx);
            }
        }

        let mut remove_flag = false;

        if let Some(s) = self.val_idx.get_mut(&val) {
            s.remove(&remove_idx);
            if s.is_empty() {
                remove_flag = true;
            }
        } else {
            unreachable!()
        }

        if remove_flag {
            self.val_idx.remove(&val);
        }

        true
    }

    fn get_random(&self) -> i32 {
        self.data[thread_rng().gen_range(0..self.data.len())]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_381() {
        // let mut obj = RandomizedCollection::new();
        // assert!(obj.insert(1));
        // assert!(!obj.insert(1));
        // assert!(obj.insert(2));
        // assert_eq!(obj.get_random(), 2);
        // assert!(obj.remove(1));
        // assert_eq!(obj.get_random(), 1);
        let mut obj = RandomizedCollection::new();
        assert!(obj.insert(1));
        assert!(obj.insert(10));
        assert!(!obj.insert(10));
        assert!(obj.insert(100));
        println!("{}", obj.get_random());
        println!("{}", obj.get_random());
        println!("{}", obj.get_random());
        println!("{}", obj.get_random());
        println!("{}", obj.get_random());
        println!("{}", obj.get_random());
    }
}
