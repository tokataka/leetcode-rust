///
/// # 3484. Design Spreadsheet
///
/// A spreadsheet is a grid with 26 columns (labeled from `'A'` to `'Z'`) and a given number of `rows`. Each cell in the spreadsheet can hold an integer value between 0 and 10<sup>5</sup>.
///
/// Implement the `Spreadsheet` class:
///
/// * `Spreadsheet(int rows)` Initializes a spreadsheet with 26 columns (labeled `'A'` to `'Z'`) and the specified number of rows. All cells are initially set to 0.
/// * `void setCell(String cell, int value)` Sets the value of the specified `cell`. The cell reference is provided in the format `"AX"` (e.g., `"A1"`, `"B10"`), where the letter represents the column (from `'A'` to `'Z'`) and the number represents a **1-indexed** row.
/// * `void resetCell(String cell)` Resets the specified cell to 0.
/// * `int getValue(String formula)` Evaluates a formula of the form `"=X+Y"`, where `X` and `Y` are **either** cell references or non-negative integers, and returns the computed sum.
///
/// **Note:** If `getValue` references a cell that has not been explicitly set using `setCell`, its value is considered 0.
///
/// **Example 1:**
///
/// **Input:**
/// ["Spreadsheet", "getValue", "setCell", "getValue", "setCell", "getValue", "resetCell", "getValue"]
/// [[3], ["=5+7"], ["A1", 10], ["=A1+6"], ["B2", 15], ["=A1+B2"], ["A1"], ["=A1+B2"]]
///
/// **Output:**
/// [null, 12, null, 16, null, 25, null, 15]
///
/// **Explanation**
///
/// Spreadsheet spreadsheet = new Spreadsheet(3); // Initializes a spreadsheet with 3 rows and 26 columns
/// spreadsheet.getValue("=5+7"); // returns 12 (5+7)
/// spreadsheet.setCell("A1", 10); // sets A1 to 10
/// spreadsheet.getValue("=A1+6"); // returns 16 (10+6)
/// spreadsheet.setCell("B2", 15); // sets B2 to 15
/// spreadsheet.getValue("=A1+B2"); // returns 25 (10+15)
/// spreadsheet.resetCell("A1"); // resets A1 to 0
/// spreadsheet.getValue("=A1+B2"); // returns 15 (0+15)
///
/// **Constraints:**
///
/// * `1 <= rows <= 10<sup>3</sup>`
/// * `0 <= value <= 10<sup>5</sup>`
/// * The formula is always in the format `"=X+Y"`, where `X` and `Y` are either valid cell references or **non-negative** integers with values less than or equal to `10<sup>5</sup>`.
/// * Each cell reference consists of a capital letter from `'A'` to `'Z'` followed by a row number between `1` and `rows`.
/// * At most `10<sup>4</sup>` calls will be made in **total** to `setCell`, `resetCell`, and `getValue`.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/design-spreadsheet/
// discuss: https://leetcode.com/problems/design-spreadsheet/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

struct Spreadsheet {
    sheet: HashMap<String, i32>,
}

impl Spreadsheet {
    fn new(_rows: i32) -> Self {
        Self {
            sheet: HashMap::new(),
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.sheet
            .entry(cell)
            .and_modify(|x| *x = value)
            .or_insert(value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.sheet.remove(&cell);
    }

    fn get_value(&self, formula: String) -> i32 {
        let mut result = 0;

        for x in formula.split_at(1).1.split('+') {
            match x.parse::<i32>() {
                Ok(x) => result += x,
                Err(_) => result += self.sheet.get(x).unwrap_or(&0),
            };
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3484() {
        let mut obj = Spreadsheet::new(3);
        assert_eq!(obj.get_value("=5+7".to_owned()), 12);
        obj.set_cell("A1".to_owned(), 10);
        assert_eq!(obj.get_value("=A1+6".to_owned()), 16);
        obj.set_cell("B2".to_owned(), 15);
        assert_eq!(obj.get_value("=A1+B2".to_owned()), 25);
        obj.reset_cell("A1".to_owned());
        assert_eq!(obj.get_value("=A1+B2".to_owned()), 15);
    }
}
