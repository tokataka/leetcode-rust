///
/// # 2353. Design a Food Rating System
///
/// Design a food rating system that can do the following:
///
/// * **Modify** the rating of a food item listed in the system.
/// * Return the highest-rated food item for a type of cuisine in the system.
///
/// Implement the `FoodRatings` class:
///
/// * `FoodRatings(String[] foods, String[] cuisines, int[] ratings)` Initializes the system. The food items are described by `foods`, `cuisines` and `ratings`, all of which have a length of `n`.
///   * `foods[i]` is the name of the `i<sup>th</sup>` food,
///   * `cuisines[i]` is the type of cuisine of the `i<sup>th</sup>` food, and
///   * `ratings[i]` is the initial rating of the `i<sup>th</sup>` food.
///
/// * `void changeRating(String food, int newRating)` Changes the rating of the food item with the name `food`.
/// * `String highestRated(String cuisine)` Returns the name of the food item that has the highest rating for the given type of `cuisine`. If there is a tie, return the item with the **lexicographically smaller** name.
///
/// Note that a string `x` is lexicographically smaller than string `y` if `x` comes before `y` in dictionary order, that is, either `x` is a prefix of `y`, or if `i` is the first position such that `x[i] != y[i]`, then `x[i]` comes before `y[i]` in alphabetic order.
///
/// **Example 1:**
///
/// ```
/// Input
/// ["FoodRatings", "highestRated", "highestRated", "changeRating", "highestRated", "changeRating", "highestRated"]
/// [[["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], ["korean", "japanese", "japanese", "greek", "japanese", "korean"], [9, 12, 8, 15, 14, 7]], ["korean"], ["japanese"], ["sushi", 16], ["japanese"], ["ramen", 16], ["japanese"]]
/// Output
/// [null, "kimchi", "ramen", null, "sushi", null, "ramen"]
///
/// Explanation
/// FoodRatings foodRatings = new FoodRatings(["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], ["korean", "japanese", "japanese", "greek", "japanese", "korean"], [9, 12, 8, 15, 14, 7]);
/// foodRatings.highestRated("korean"); // return "kimchi"
///                                     // "kimchi" is the highest rated korean food with a rating of 9.
/// foodRatings.highestRated("japanese"); // return "ramen"
///                                       // "ramen" is the highest rated japanese food with a rating of 14.
/// foodRatings.changeRating("sushi", 16); // "sushi" now has a rating of 16.
/// foodRatings.highestRated("japanese"); // return "sushi"
///                                       // "sushi" is the highest rated japanese food with a rating of 16.
/// foodRatings.changeRating("ramen", 16); // "ramen" now has a rating of 16.
/// foodRatings.highestRated("japanese"); // return "ramen"
///                                       // Both "sushi" and "ramen" have a rating of 16.
///                                       // However, "ramen" is lexicographically smaller than "sushi".
///
/// ```
///
/// **Constraints:**
///
/// * `1 <= n <= 2 * 10<sup>4</sup>`
/// * `n == foods.length == cuisines.length == ratings.length`
/// * `1 <= foods[i].length, cuisines[i].length <= 10`
/// * `foods[i]`, `cuisines[i]` consist of lowercase English letters.
/// * `1 <= ratings[i] <= 10<sup>8</sup>`
/// * All the strings in `foods` are **distinct**.
/// * `food` will be the name of a food item in the system across all calls to `changeRating`.
/// * `cuisine` will be a type of cuisine of **at least one** food item in the system across all calls to `highestRated`.
/// * At most `2 * 10<sup>4</sup>` calls **in total** will be made to `changeRating` and `highestRated`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/design-a-food-rating-system/
// discuss: https://leetcode.com/problems/design-a-food-rating-system/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use itertools::Itertools;

struct FoodRatings {
    highest_ratings: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>,
    latest_ratings: HashMap<String, i32>,
    food_cuisine_map: HashMap<String, String>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut highest_ratings: HashMap<String, BinaryHeap<(i32, Reverse<String>)>> =
            HashMap::new();
        let mut latest_ratings = HashMap::new();
        let mut food_cuisine_map = HashMap::new();

        for ((food, cuisine), rating) in foods.into_iter().zip(cuisines).zip(ratings) {
            highest_ratings
                .entry(cuisine.clone())
                .or_default()
                .push((rating, Reverse(food.clone())));
            latest_ratings.insert(food.clone(), rating);
            food_cuisine_map.insert(food, cuisine);
        }

        Self {
            highest_ratings,
            latest_ratings,
            food_cuisine_map,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        self.latest_ratings.insert(food.clone(), new_rating);
        self.highest_ratings
            .get_mut(self.food_cuisine_map.get(&food).unwrap())
            .unwrap()
            .push((new_rating, Reverse(food)));
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let highest_rating = self.highest_ratings.get_mut(&cuisine).unwrap();

        while let Some(x) = highest_rating.peek() {
            let latest_rating = *self.latest_ratings.get(&x.1 .0).unwrap();

            if latest_rating == x.0 {
                return x.1 .0.clone();
            }

            highest_rating.pop();
        }

        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2353() {
        let mut obj = FoodRatings::new(
            vec_string!["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"],
            vec_string!["korean", "japanese", "japanese", "greek", "japanese", "korean"],
            vec![9, 12, 8, 15, 14, 7],
        );
        assert_eq!(obj.highest_rated("korean".to_owned()), "kimchi");
        assert_eq!(obj.highest_rated("japanese".to_owned()), "ramen");
        obj.change_rating("sushi".to_owned(), 16);
        assert_eq!(obj.highest_rated("japanese".to_owned()), "sushi");
        obj.change_rating("ramen".to_owned(), 16);
        assert_eq!(obj.highest_rated("japanese".to_owned()), "ramen");
    }
}
