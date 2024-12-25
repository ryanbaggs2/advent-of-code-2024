use std::fs;
use std::io::prelude::*;

pub(crate) fn day1() {
    part_one();
    part_two();
}

// Total time complexity is O(n* log(k)) due to sorting requirement.
/// Thanks to Chris Biscardi for this solution:
/// https://www.youtube.com/watch?v=HXWnVnwqluQ
fn part_one() {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let input: String = fs::read_to_string("inputday1.txt")
        .unwrap();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(
            items.next().unwrap().parse::<i32>().unwrap(),
        );
        right.push(
            items.next().unwrap().parse::<i32>().unwrap(),
        );
    }

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left, right)
        .map(|(l, r)| {(l - r).abs()})
        .sum();

    println!("Result, Day 1 part 1: {}", result);
}

// I think this is actually an O(n^2) solution. Because filter worst case would need to 
// iterate through the whole right side for every value in left side.
// Still an idiomatic and working solution.
/// Thanks to Chris Biscardi for this solution:
/// https://www.youtube.com/watch?v=HXWnVnwqluQ
fn part_two() {
    let mut left = vec![];
    let mut right = vec![];
    
    let input: String = fs::read_to_string("inputday1.txt")
        .unwrap();
    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(
            items.next().unwrap().parse::<usize>().unwrap()
        );
        right.push(
            items.next().unwrap().parse::<usize>().unwrap()
        );
    }
    
    let result: usize = left
        .iter()
        .map(|number| {
            number * right
                .iter()
                .filter(|r| &number == r)
                .count()
        })
        .sum();
    
    println!("Result Day 1, part 2: {}", result);
}
