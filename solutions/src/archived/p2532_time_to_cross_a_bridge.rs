use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # 2532. Time to Cross a Bridge
///
/// There are `k` workers who want to move `n` boxes from the right (old) warehouse to the left (new) warehouse. You are given the two integers `n` and `k`, and a 2D integer array `time` of size `k x 4` where `time[i] = [right<sub>i</sub>, pick<sub>i</sub>, left<sub>i</sub>, put<sub>i</sub>]`.
///
/// The warehouses are separated by a river and connected by a bridge. Initially, all `k` workers are waiting on the left side of the bridge. To move the boxes, the `i<sup>th</sup>` worker can do the following:
///
/// * Cross the bridge to the right side in `right<sub>i</sub>` minutes.
/// * Pick a box from the right warehouse in `pick<sub>i</sub>` minutes.
/// * Cross the bridge to the left side in `left<sub>i</sub>` minutes.
/// * Put the box into the left warehouse in `put<sub>i</sub>` minutes.
///
/// The `i<sup>th</sup>` worker is **less efficient** than the j`<sup>th</sup>` worker if either condition is met:
///
/// * `left<sub>i</sub> + right<sub>i</sub> > left<sub>j</sub> + right<sub>j</sub>`
/// * `left<sub>i</sub> + right<sub>i</sub> == left<sub>j</sub> + right<sub>j</sub>` and `i > j`
///
/// The following rules regulate the movement of the workers through the bridge:
///
/// * Only one worker can use the bridge at a time.
/// * When the bridge is unused prioritize the **least efficient** worker (who have picked up the box) on the right side to cross. If not, prioritize the **least efficient** worker on the left side to cross.
/// * If enough workers have already been dispatched from the left side to pick up all the remaining boxes, **no more** workers will be sent from the left side.
///
/// Return the **elapsed minutes** at which the last box reaches the **left side of the bridge**.
///
/// **Example 1:**
///
/// **Input:** n = 1, k = 3, time = [[1,1,2,1],[1,1,3,1],[1,1,4,1]]
///
/// **Output:** 6
///
/// **Explanation:**
///
/// ```
/// From 0 to 1 minutes: worker 2 crosses the bridge to the right.
/// From 1 to 2 minutes: worker 2 picks up a box from the right warehouse.
/// From 2 to 6 minutes: worker 2 crosses the bridge to the left.
/// From 6 to 7 minutes: worker 2 puts a box at the left warehouse.
/// The whole process ends after 7 minutes. We return 6 because the problem asks for the instance of time at which the last worker reaches the left side of the bridge.
///
/// ```
///
/// **Example 2:**
///
/// **Input:** n = 3, k = 2, time = [[1,5,1,8],[10,10,10,10]]
///
/// **Output:** 37
///
/// **Explanation:**
///
/// ```
///
/// ```
///
/// The last box reaches the left side at 37 seconds. Notice, how we **do not** put the last boxes down, as that would take more time, and they are already on the left with the workers.
///
/// **Constraints:**
///
/// * `1 <= n, k <= 10<sup>4</sup>`
/// * `time.length == k`
/// * `time[i].length == 4`
/// * `1 <= left<sub>i</sub>, pick<sub>i</sub>, right<sub>i</sub>, put<sub>i</sub> <= 1000`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/time-to-cross-a-bridge/
// discuss: https://leetcode.com/problems/time-to-cross-a-bridge/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_crossing_time(n: i32, _k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut left_pool: BinaryHeap<(i32, usize)> =
            BinaryHeap::from_iter(time.iter().enumerate().map(|(i, t)| (t[0] + t[2], i)));

        let mut right_pool: BinaryHeap<(i32, usize)> = BinaryHeap::new();

        let mut pq: BinaryHeap<(Reverse<i32>, State)> = BinaryHeap::new();

        #[derive(PartialEq, PartialOrd, Eq, Ord)]
        enum State {
            MovingRight(usize),
            Picking(usize),
            MovingLeft(usize),
            Putting(usize),
        }

        use State::*;

        let mut dispatched = 0;
        let mut bridge_using = false;
        let mut cur_time = 0;

        loop {
            if !bridge_using {
                if !right_pool.is_empty() {
                    let (_, worker) = right_pool.pop().unwrap();

                    pq.push((Reverse(cur_time + time[worker][2]), MovingLeft(worker)));
                    bridge_using = true;
                } else if dispatched < n && !left_pool.is_empty() {
                    let (_, worker) = left_pool.pop().unwrap();

                    pq.push((Reverse(cur_time + time[worker][0]), MovingRight(worker)));
                    bridge_using = true;
                    dispatched += 1;
                }
            }

            if let Some((Reverse(t), state)) = pq.pop() {
                cur_time = t;

                match state {
                    State::MovingRight(worker) => {
                        pq.push((Reverse(cur_time + time[worker][1]), Picking(worker)));
                        bridge_using = false;
                    }
                    State::Picking(worker) => {
                        right_pool.push((time[worker][0] + time[worker][2], worker));
                    }
                    State::MovingLeft(worker) => {
                        if dispatched == n && right_pool.is_empty() {
                            return cur_time;
                        }

                        pq.push((Reverse(cur_time + time[worker][3]), Putting(worker)));
                        bridge_using = false;
                    }
                    State::Putting(worker) => {
                        left_pool.push((time[worker][0] + time[worker][2], worker));
                    }
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2532() {
        let n = 1;
        let k = 3;
        let time = nd_vec![[1, 1, 2, 1], [1, 1, 3, 1], [1, 1, 4, 1]];
        let expected = 6;
        assert_eq!(Solution::find_crossing_time(n, k, time), expected);
        let n = 3;
        let k = 2;
        let time = nd_vec![[1, 5, 1, 8], [10, 10, 10, 10]];
        let expected = 37;
        assert_eq!(Solution::find_crossing_time(n, k, time), expected);
    }
}
