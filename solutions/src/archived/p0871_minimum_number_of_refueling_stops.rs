///
/// # 871. Minimum Number of Refueling Stops
///
/// A car travels from a starting position to a destination which is `target` miles east of the starting position.
///
/// There are gas stations along the way. The gas stations are represented as an array `stations` where `stations[i] = [position<sub>i</sub>, fuel<sub>i</sub>]` indicates that the `i<sup>th</sup>` gas station is `position<sub>i</sub>` miles east of the starting position and has `fuel<sub>i</sub>` liters of gas.
///
/// The car starts with an infinite tank of gas, which initially has `startFuel` liters of fuel in it. It uses one liter of gas per one mile that it drives. When the car reaches a gas station, it may stop and refuel, transferring all the gas from the station into the car.
///
/// Return *the minimum number of refueling stops the car must make in order to reach its destination*. If it cannot reach the destination, return `-1`.
///
/// Note that if the car reaches a gas station with `0` fuel left, the car can still refuel there. If the car reaches the destination with `0` fuel left, it is still considered to have arrived.
///
/// **Example 1:**
///
/// ```
/// Input: target = 1, startFuel = 1, stations = []
/// Output: 0
/// Explanation: We can reach the target without refueling.
///
/// ```
///
/// **Example 2:**
///
/// ```
/// Input: target = 100, startFuel = 1, stations = [[10,100]]
/// Output: -1
/// Explanation: We can not reach the target (or even the first gas station).
///
/// ```
///
/// **Example 3:**
///
/// ```
/// Input: target = 100, startFuel = 10, stations = [[10,60],[20,30],[30,30],[60,40]]
/// Output: 2
/// Explanation: We start with 10 liters of fuel.
/// We drive to position 10, expending 10 liters of fuel.  We refuel from 0 liters to 60 liters of gas.
/// Then, we drive from position 10 to position 60 (expending 50 liters of fuel),
/// and refuel from 10 liters to 50 liters of gas.  We then drive to and reach the target.
/// We made 2 refueling stops along the way, so we return 2.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= target, startFuel <= 10<sup>9</sup>`
/// * `0 <= stations.length <= 500`
/// * `1 <= position<sub>i</sub> < position<sub>i+1</sub> < target`
/// * `1 <= fuel<sub>i</sub> < 10<sup>9</sup>`
///
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-refueling-stops/
// discuss: https://leetcode.com/problems/minimum-number-of-refueling-stops/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use itertools::Itertools;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut total_fuel = start_fuel;
        let mut station_fuel_pq = BinaryHeap::new();

        let mut stations = stations;
        stations.push(vec![target, 0]);

        for station in &stations {
            let (position, fuel) = (station[0], station[1]);

            while position > total_fuel {
                match station_fuel_pq.pop() {
                    Some(x) => total_fuel += x,
                    None => return -1,
                };
            }

            station_fuel_pq.push(fuel);
        }

        (stations.len() - station_fuel_pq.len()) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_871() {
        assert_eq!(Solution::min_refuel_stops(1, 1, nd_vec![]), 0);
        assert_eq!(Solution::min_refuel_stops(100, 1, nd_vec![[10, 100]]), -1);
        assert_eq!(
            Solution::min_refuel_stops(100, 10, nd_vec![[10, 60], [20, 30], [30, 30], [60, 40]]),
            2
        );
    }
}
