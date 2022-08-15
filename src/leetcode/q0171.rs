// Leetcode #171 - Excel Sheet Column Number
// https://leetcode.com/problems/excel-sheet-column-number/
//
// Given a string columnTitle that represents the column title as appears in an
// Excel sheet, return its corresponding column number.
//
// For Example:
//
// A -> 1
// B -> 2
// C -> 3
// ...
// Z -> 26
// AA -> 27
// AB -> 28

pub fn title_to_number(column_title: String) -> i32 {
    let mut result = 0;
    let length = column_title.len();

    for (i, c) in column_title.chars().enumerate() {
        result += get_value_by_position(c, i, length)
    }

    result
}

fn letter_to_base_value(letter: char) -> i32 {
    (letter as i32) - 64
}

fn get_value_by_position(letter: char, index: usize, length: usize) -> i32 {
    let base: i32 = 26;
    let position = length - index - 1;
    base.pow(position as u32) * letter_to_base_value(letter)
}

#[test]
fn leetcode_testcase_1() {
    let result = title_to_number("A".to_string());
    let expected = 1;

    assert_eq!(result, expected);
}

#[test]
fn leetcode_testcase_2() {
    let result = title_to_number("AB".to_string());
    let expected = 28;

    assert_eq!(result, expected);
}

#[test]
fn leetcode_testcase_3() {
    let result = title_to_number("ZY".to_string());
    let expected = 701;

    assert_eq!(result, expected);
}
