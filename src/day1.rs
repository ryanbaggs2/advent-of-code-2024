use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub(crate) fn day1() {
    let (mut left_list, mut right_list) = build_lists();

    left_list.sort();
    right_list.sort();

    print_distance(left_list, right_list);
}

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

fn print_distance(left_list: Vec<u32>, right_list: Vec<u32>) {
    let mut iter = right_list.iter();
    let mut total_distance = 0;
    
    for left in left_list {
        let right = iter.next().unwrap();
        total_distance += left.abs_diff(*right);
    }
    
    println!("Total distance: {}", total_distance);
}
