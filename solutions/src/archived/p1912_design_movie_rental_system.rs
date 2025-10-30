///
/// # 1912. Design Movie Rental System
///
/// You have a movie renting company consisting of `n` shops. You want to implement a renting system that supports searching for, booking, and returning movies. The system should also support generating a report of the currently rented movies.
///
/// Each movie is given as a 2D integer array `entries` where `entries[i] = [shop<sub>i</sub>, movie<sub>i</sub>, price<sub>i</sub>]` indicates that there is a copy of movie `movie<sub>i</sub>` at shop `shop<sub>i</sub>` with a rental price of `price<sub>i</sub>`. Each shop carries **at most one** copy of a movie `movie<sub>i</sub>`.
///
/// The system should support the following functions:
///
/// * **Search**: Finds the **cheapest 5 shops** that have an **unrented copy** of a given movie. The shops should be sorted by **price** in ascending order, and in case of a tie, the one with the **smaller** `shop<sub>i</sub>` should appear first. If there are less than 5 matching shops, then all of them should be returned. If no shop has an unrented copy, then an empty list should be returned.
/// * **Rent**: Rents an **unrented copy** of a given movie from a given shop.
/// * **Drop**: Drops off a **previously rented copy** of a given movie at a given shop.
/// * **Report**: Returns the **cheapest 5 rented movies** (possibly of the same movie ID) as a 2D list `res` where `res[j] = [shop<sub>j</sub>, movie<sub>j</sub>]` describes that the `j<sup>th</sup>` cheapest rented movie `movie<sub>j</sub>` was rented from the shop `shop<sub>j</sub>`. The movies in `res` should be sorted by **price** in ascending order, and in case of a tie, the one with the **smaller** `shop<sub>j</sub>` should appear first, and if there is still tie, the one with the **smaller** `movie<sub>j</sub>` should appear first. If there are fewer than 5 rented movies, then all of them should be returned. If no movies are currently being rented, then an empty list should be returned.
///
/// Implement the `MovieRentingSystem` class:
///
/// * `MovieRentingSystem(int n, int[][] entries)` Initializes the `MovieRentingSystem` object with `n` shops and the movies in `entries`.
/// * `List<Integer> search(int movie)` Returns a list of shops that have an **unrented copy** of the given `movie` as described above.
/// * `void rent(int shop, int movie)` Rents the given `movie` from the given `shop`.
/// * `void drop(int shop, int movie)` Drops off a previously rented `movie` at the given `shop`.
/// * `List<List<Integer>> report()` Returns a list of cheapest **rented** movies as described above.
///
/// **Note:** The test cases will be generated such that `rent` will only be called if the shop has an **unrented** copy of the movie, and `drop` will only be called if the shop had **previously rented** out the movie.
///
/// **Example 1:**
///
/// ```
/// Input
/// ["MovieRentingSystem", "search", "rent", "rent", "report", "drop", "search"]
/// [[3, [[0, 1, 5], [0, 2, 6], [0, 3, 7], [1, 1, 4], [1, 2, 7], [2, 1, 5]]], [1], [0, 1], [1, 2], [], [1, 2], [2]]
/// Output
/// [null, [1, 0, 2], null, null, [[0, 1], [1, 2]], null, [0, 1]]
///
/// Explanation
/// MovieRentingSystem movieRentingSystem = new MovieRentingSystem(3, [[0, 1, 5], [0, 2, 6], [0, 3, 7], [1, 1, 4], [1, 2, 7], [2, 1, 5]]);
/// movieRentingSystem.search(1);  // return [1, 0, 2], Movies of ID 1 are unrented at shops 1, 0, and 2. Shop 1 is cheapest; shop 0 and 2 are the same price, so order by shop number.
/// movieRentingSystem.rent(0, 1); // Rent movie 1 from shop 0. Unrented movies at shop 0 are now [2,3].
/// movieRentingSystem.rent(1, 2); // Rent movie 2 from shop 1. Unrented movies at shop 1 are now [1].
/// movieRentingSystem.report();   // return [[0, 1], [1, 2]]. Movie 1 from shop 0 is cheapest, followed by movie 2 from shop 1.
/// movieRentingSystem.drop(1, 2); // Drop off movie 2 at shop 1. Unrented movies at shop 1 are now [1,2].
/// movieRentingSystem.search(2);  // return [0, 1]. Movies of ID 2 are unrented at shops 0 and 1. Shop 0 is cheapest, followed by shop 1.
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 3 * 10<sup>5</sup>`
/// * `1 <= entries.length <= 10<sup>5</sup>`
/// * `0 <= shop<sub>i</sub> < n`
/// * `1 <= movie<sub>i</sub>, price<sub>i</sub> <= 10<sup>4</sup>`
/// * Each shop carries **at most one** copy of a movie `movie<sub>i</sub>`.
/// * At most `10<sup>5</sup>` calls **in total** will be made to `search`, `rent`, `drop` and `report`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/design-movie-rental-system/
// discuss: https://leetcode.com/problems/design-movie-rental-system/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use itertools::Itertools;

struct MovieRentingSystem {
    prices: HashMap<(i32, i32), i32>,
    unrented: Vec<BTreeSet<(i32, i32)>>,
    rented: BTreeSet<(i32, i32, i32)>,
}

impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut prices = HashMap::new();
        let mut unrented = vec![BTreeSet::new(); 10001];

        for entry in entries {
            let (shop, movie, price) = (entry[0], entry[1], entry[2]);

            prices.insert((shop, movie), price);
            unrented[movie as usize].insert((price, shop));
        }

        Self {
            prices,
            unrented,
            rented: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.unrented[movie as usize]
            .iter()
            .take(5)
            .map(|x| x.1)
            .collect()
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = *self.prices.get(&(shop, movie)).unwrap();

        self.unrented[movie as usize].remove(&(price, shop));
        self.rented.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = *self.prices.get(&(shop, movie)).unwrap();

        self.unrented[movie as usize].insert((price, shop));
        self.rented.remove(&(price, shop, movie));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented.iter().take(5).map(|x| vec![x.1, x.2]).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1912() {
        let mut obj = MovieRentingSystem::new(
            3,
            nd_vec![
                [0, 1, 5],
                [0, 2, 6],
                [0, 3, 7],
                [1, 1, 4],
                [1, 2, 7],
                [2, 1, 5]
            ],
        );
        assert_eq!(obj.search(1), [1, 0, 2]);
        obj.rent(0, 1);
        obj.rent(1, 2);
        assert_eq!(obj.report(), [[0, 1], [1, 2]]);
        obj.drop(1, 2);
        assert_eq!(obj.search(2), [0, 1]);
    }
}
