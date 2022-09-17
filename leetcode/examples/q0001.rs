// Leetcode #1 - Two Sum
// https://leetcode.com/problems/two-sum/
//
// Given an array of integers nums and an integer target,
// return indices of the two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
// You can return the answer in any order

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut cache = HashMap::<i32, usize>::new();
    let mut solution = Vec::new();

    for (index, num) in nums.iter().enumerate() {
        let matching = target - num;

        if cache.contains_key(&matching) {
            solution.push(index as i32);
            solution.push(cache.get(&matching).unwrap().to_owned() as i32);
            break;
        } else {
            cache.insert(num.clone(), index);
        }
    }

    solution
}

fn main() {}

#[test]
fn leetcode_testcase_1() {
    let mut result = two_sum(vec![2, 7, 11, 15], 9);
    result.sort(); // You can return the answer in any order
    let expected = vec![0, 1];

    assert_eq!(result, expected);
}

#[test]
fn leetcode_testcase_2() {
    let mut result = two_sum(vec![3, 2, 4], 6);
    result.sort(); // You can return the answer in any order
    let expected = vec![1, 2];

    assert_eq!(result, expected);
}

#[test]
fn leetcode_testcase_3() {
    let mut result = two_sum(vec![3, 3], 6);
    result.sort(); // You can return the answer in any order
    let expected = vec![0, 1];

    assert_eq!(result, expected);
}
