// Leetcode #1
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

#[test]
fn test_two_sum() {
    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        expecting: Vec<i32>,
    }

    vec![
        TestCase {
            nums: vec![2, 7, 11, 15],
            target: 9,
            expecting: vec![0, 1],
        },
        TestCase {
            nums: vec![3, 2, 4],
            target: 6,
            expecting: vec![1, 2],
        },
        TestCase {
            nums: vec![3, 3],
            target: 6,
            expecting: vec![0, 1],
        },
    ]
    .iter()
    .for_each(|testcase| {
        let mut result = two_sum(testcase.nums.to_owned(), testcase.target);
        result.sort();
        assert_eq!(result, testcase.expecting)
    });
}
