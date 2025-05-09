use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 1172. Dinner Plate Stacks
///
/// You have an infinite number of stacks arranged in a row and numbered (left to right) from `0`, each of the stacks has the same maximum capacity.
///
/// Implement the `DinnerPlates` class:
///
/// * `DinnerPlates(int capacity)` Initializes the object with the maximum capacity of the stacks `capacity`.
/// * `void push(int val)` Pushes the given integer `val` into the leftmost stack with a size less than `capacity`.
/// * `int pop()` Returns the value at the top of the rightmost non-empty stack and removes it from that stack, and returns `-1` if all the stacks are empty.
/// * `int popAtStack(int index)` Returns the value at the top of the stack with the given index `index` and removes it from that stack or returns `-1` if the stack with that given index is empty.
///
/// **Example 1:**
///
/// ```
/// Input
/// ["DinnerPlates", "push", "push", "push", "push", "push", "popAtStack", "push", "push", "popAtStack", "popAtStack", "pop", "pop", "pop", "pop", "pop"]
/// [[2], [1], [2], [3], [4], [5], [0], [20], [21], [0], [2], [], [], [], [], []]
/// Output
/// [null, null, null, null, null, null, 2, null, null, 20, 21, 5, 4, 3, 1, -1]
///
/// Explanation:
/// DinnerPlates D = DinnerPlates(2);  // Initialize with capacity = 2
/// D.push(1);
/// D.push(2);
/// D.push(3);
/// D.push(4);
/// D.push(5);         // The stacks are now:  2  4
///                                            1  3  5
///                                            ﹈ ﹈ ﹈
/// D.popAtStack(0);   // Returns 2.  The stacks are now:     4
///                                                        1  3  5
///                                                        ﹈ ﹈ ﹈
/// D.push(20);        // The stacks are now: 20  4
///                                            1  3  5
///                                            ﹈ ﹈ ﹈
/// D.push(21);        // The stacks are now: 20  4 21
///                                            1  3  5
///                                            ﹈ ﹈ ﹈
/// D.popAtStack(0);   // Returns 20.  The stacks are now:     4 21
///                                                         1  3  5
///                                                         ﹈ ﹈ ﹈
/// D.popAtStack(2);   // Returns 21.  The stacks are now:     4
///                                                         1  3  5
///                                                         ﹈ ﹈ ﹈
/// D.pop()            // Returns 5.  The stacks are now:      4
///                                                         1  3
///                                                         ﹈ ﹈
/// D.pop()            // Returns 4.  The stacks are now:   1  3
///                                                         ﹈ ﹈
/// D.pop()            // Returns 3.  The stacks are now:   1
///                                                         ﹈
/// D.pop()            // Returns 1.  There are no stacks.
/// D.pop()            // Returns -1.  There are still no stacks.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= capacity <= 2 * 10<sup>4</sup>`
/// * `1 <= val <= 2 * 10<sup>4</sup>`
/// * `0 <= index <= 10<sup>5</sup>`
/// * At most `2 * 10<sup>5</sup>` calls will be made to `push`, `pop`, and `popAtStack`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/dinner-plate-stacks/
// discuss: https://leetcode.com/problems/dinner-plate-stacks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct DinnerPlates {
    stacks: Vec<Vec<i32>>,
    capacity: usize,
    next_push_index: BinaryHeap<Reverse<usize>>,
    cur_len: usize,
}

#[allow(dead_code)]
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            stacks: vec![],
            capacity: capacity as usize,
            next_push_index: BinaryHeap::new(),
            cur_len: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let idx = match self.next_push_index.pop() {
            Some(Reverse(idx)) if idx < self.cur_len => idx,
            _ => {
                self.next_push_index.clear();
                if self.cur_len == self.stacks.len() {
                    self.stacks.push(vec![]);
                }
                self.cur_len += 1;
                self.cur_len - 1
            }
        };

        self.stacks[idx].push(val);

        if self.stacks[idx].len() < self.capacity {
            self.next_push_index.push(Reverse(idx));
        }
    }

    fn pop(&mut self) -> i32 {
        self.pop_at_stack(self.cur_len as i32 - 1)
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let idx = index as usize;

        let res = match self.stacks.get_mut(idx) {
            Some(st) => match st.pop() {
                Some(x) => x,
                None => return -1,
            },
            None => return -1,
        };

        if self.stacks[idx].len() == self.capacity - 1 {
            self.next_push_index.push(Reverse(idx));
        }

        while self.cur_len > 0 && self.stacks[self.cur_len - 1].is_empty() {
            self.cur_len -= 1;
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1172() {
        // let mut obj = DinnerPlates::new(2);
        // obj.push(1);
        // obj.push(2);
        // obj.push(3);
        // obj.push(4);
        // obj.push(5);
        // assert_eq!(obj.pop_at_stack(0), 2);
        // obj.push(20);
        // obj.push(21);
        // assert_eq!(obj.pop_at_stack(0), 20);
        // assert_eq!(obj.pop_at_stack(2), 21);
        // assert_eq!(obj.pop(), 5);
        // assert_eq!(obj.pop(), 4);
        // assert_eq!(obj.pop(), 3);
        // assert_eq!(obj.pop(), 1);
        // assert_eq!(obj.pop(), -1);
        let mut obj = DinnerPlates::new(1);
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.pop_at_stack(1), 2);
        assert_eq!(obj.pop(), 1);
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.pop(), 1);
    }
}
