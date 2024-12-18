use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub(crate) fn day1() {
    part_one();
    let (left_list, right_list) = build_lists();
    part_two(left_list, right_list);
}

// Total time complexity is O(n* log(k)) due to sorting requirement.
fn part_one() {
    let (mut left_list, mut right_list) = build_lists();

    left_list.sort();
    right_list.sort();

    print_distance(left_list, right_list);
}

// O(n) time complexity
fn part_two(left_list: Vec<u32>, right_list: Vec<u32>) {
    let mut counts = HashMap::new();

    // Count the number of times ID number shows in the right list
    // O(n) time complexity
    for id_num in right_list {
        if let Some(count) = counts.get_mut(&id_num) {
            *count += 1;
        } else {
            counts.insert(id_num, 1);
        }
    }

    // Get the similarity score for each ID number in left list
    // O(n) time complexity
    let mut similarity_scores = Vec::new();
    for id_num in left_list {
        if let Some(count) = counts.get(&id_num) {
            let similarity_score = id_num * count;
            similarity_scores.push(similarity_score);
        }
    }
    
    let sum: u32 = similarity_scores.iter().sum();
    println!("Similarity score of the lists is: {}", sum);
}

// Time complexity O(n)
fn build_lists() -> (Vec<u32>, Vec<u32>){
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let file = File::open("inputday1.txt")
        .expect("Should be able to read");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();
        let left = iter.next().unwrap().parse::<u32>().unwrap();
        let right = iter.next().unwrap().parse::<u32>().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    (left_list, right_list)
}

// Time complexity O(n)
fn print_distance(left_list: Vec<u32>, right_list: Vec<u32>) {
    let mut iter = right_list.iter();
    let mut total_distance = 0;
    
    for left in left_list {
        let right = iter.next().unwrap();
        total_distance += left.abs_diff(*right);
    }
    
    println!("Total distance: {}", total_distance);
}
