///
/// # 3508. Implement Router
///
/// Design a data structure that can efficiently manage data packets in a network router. Each data packet consists of the following attributes:
///
/// * `source`: A unique identifier for the machine that generated the packet.
/// * `destination`: A unique identifier for the target machine.
/// * `timestamp`: The time at which the packet arrived at the router.
///
/// Implement the `Router` class:
///
/// `Router(int memoryLimit)`: Initializes the Router object with a fixed memory limit.
///
/// * `memoryLimit` is the **maximum** number of packets the router can store at any given time.
/// * If adding a new packet would exceed this limit, the **oldest** packet must be removed to free up space.
///
/// `bool addPacket(int source, int destination, int timestamp)`: Adds a packet with the given attributes to the router.
///
/// * A packet is considered a duplicate if another packet with the same `source`, `destination`, and `timestamp` already exists in the router.
/// * Return `true` if the packet is successfully added (i.e., it is not a duplicate); otherwise return `false`.
///
/// `int[] forwardPacket()`: Forwards the next packet in FIFO (First In First Out) order.
///
/// * Remove the packet from storage.
/// * Return the packet as an array `[source, destination, timestamp]`.
/// * If there are no packets to forward, return an empty array.
///
/// `int getCount(int destination, int startTime, int endTime)`:
///
/// * Returns the number of packets currently stored in the router (i.e., not yet forwarded) that have the specified destination and have timestamps in the inclusive range `[startTime, endTime]`.
///
/// **Note** that queries for `addPacket` will be made in increasing order of `timestamp`.
///
/// **Example 1:**
///
/// **Input:**
/// ["Router", "addPacket", "addPacket", "addPacket", "addPacket", "addPacket", "forwardPacket", "addPacket", "getCount"]
/// [[3], [1, 4, 90], [2, 5, 90], [1, 4, 90], [3, 5, 95], [4, 5, 105], [], [5, 2, 110], [5, 100, 110]]
///
/// **Output:**
/// [null, true, true, false, true, true, [2, 5, 90], true, 1]
///
/// **Explanation**
///
/// Router router = new Router(3); // Initialize Router with memoryLimit of 3.
/// router.addPacket(1, 4, 90); // Packet is added. Return True.
/// router.addPacket(2, 5, 90); // Packet is added. Return True.
/// router.addPacket(1, 4, 90); // This is a duplicate packet. Return False.
/// router.addPacket(3, 5, 95); // Packet is added. Return True
/// router.addPacket(4, 5, 105); // Packet is added, `[1, 4, 90]` is removed as number of packets exceeds memoryLimit. Return True.
/// router.forwardPacket(); // Return `[2, 5, 90]` and remove it from router.
/// router.addPacket(5, 2, 110); // Packet is added. Return True.
/// router.getCount(5, 100, 110); // The only packet with destination 5 and timestamp in the inclusive range `[100, 110]` is `[4, 5, 105]`. Return 1.
///
/// **Example 2:**
///
/// **Input:**
/// ["Router", "addPacket", "forwardPacket", "forwardPacket"]
/// [[2], [7, 4, 90], [], []]
///
/// **Output:**
/// [null, true, [7, 4, 90], []]
///
/// **Explanation**
///
/// Router router = new Router(2); // Initialize `Router` with `memoryLimit` of 2.
/// router.addPacket(7, 4, 90); // Return True.
/// router.forwardPacket(); // Return `[7, 4, 90]`.
/// router.forwardPacket(); // There are no packets left, return `[]`.
///
/// **Constraints:**
///
/// * `2 <= memoryLimit <= 10<sup>5</sup>`
/// * `1 <= source, destination <= 2 * 10<sup>5</sup>`
/// * `1 <= timestamp <= 10<sup>9</sup>`
/// * `1 <= startTime <= endTime <= 10<sup>9</sup>`
/// * At most `10<sup>5</sup>` calls will be made to `addPacket`, `forwardPacket`, and `getCount` methods altogether.
/// * queries for `addPacket` will be made in increasing order of `timestamp`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-router/
// discuss: https://leetcode.com/problems/implement-router/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use itertools::Itertools;

struct Router {
    memory_limit: usize,
    q: VecDeque<[i32; 3]>,
    packets: HashSet<[i32; 3]>,
    dest_timestamp_map: HashMap<i32, VecDeque<i32>>,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            memory_limit: memory_limit as usize,
            q: VecDeque::new(),
            packets: HashSet::new(),
            dest_timestamp_map: HashMap::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = [source, destination, timestamp];

        if !self.packets.insert(packet) {
            return false;
        }

        self.q.push_back(packet);
        self.dest_timestamp_map
            .entry(destination)
            .or_default()
            .push_back(timestamp);

        if self.q.len() > self.memory_limit {
            let popped = self.q.pop_front().unwrap();
            self.packets.remove(&popped);
            self.dest_timestamp_map
                .get_mut(&popped[1])
                .unwrap()
                .pop_front();
        }

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        let popped = match self.q.pop_front() {
            Some(x) => x,
            None => return vec![],
        };

        self.packets.remove(&popped);
        self.dest_timestamp_map
            .get_mut(&popped[1])
            .unwrap()
            .pop_front();

        popped.to_vec()
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        self.dest_timestamp_map
            .get(&destination)
            .map(|q| q.partition_point(|&x| x <= end_time) - q.partition_point(|&x| x < start_time))
            .unwrap_or(0) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3508() {
        let mut obj = Router::new(3);
        assert!(obj.add_packet(1, 4, 90));
        assert!(obj.add_packet(2, 5, 90));
        assert!(!obj.add_packet(1, 4, 90));
        assert!(obj.add_packet(3, 5, 95));
        assert!(obj.add_packet(4, 5, 105));
        assert_eq!(obj.forward_packet(), [2, 5, 90]);
        assert!(obj.add_packet(5, 2, 110));
        assert_eq!(obj.get_count(5, 100, 110), 1);
        let mut obj = Router::new(2);
        assert!(obj.add_packet(7, 4, 90));
        assert_eq!(obj.forward_packet(), [7, 4, 90]);
        assert_eq!(obj.forward_packet(), []);
    }
}
