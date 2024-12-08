use std::fs::File;
use std::io::Read;
use regex::Regex;

pub fn day3() {
    part_one();
    part_two();
}

fn part_one() {
    // This regular expression should work out to be accepting this style mul(0-999,0-999)
    // example: mul(12,654)
    // We use capture groups, the parenthesis, to get the values, which extract() can
    // get the &str of the values from.
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let hay = read_input();

    // Get the values matching the pattern, parse the values, multiply the pairs, get the sum of all
    // products.
    let result: u32 = sum_of_products(re, &*hay);

    println!("Sum of the products is: {}", result);
}

fn part_two() {
    // Checks for do()
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut hay = read_input();
    let mut current_mul_index: usize;
    let mut end_of_mul: usize;
    let mut multiply_inst: String;
    let mut multiply = true;
    let mut hay_substring;
    let mut instructions_to_execute: Vec<String> = Vec::new();
    
    loop {
        if let Some(mat) = re.find(&*hay) {
            current_mul_index = mat.start();
            end_of_mul = mat.end();
            multiply_inst = mat.as_str().parse().unwrap();
        } else {
            println!("happens");
            break; 
        }
        
        hay_substring = &hay[0..=current_mul_index];
        
        let do_index = hay_substring.rfind("do()");
        let dont_index = hay_substring.rfind("don\'t()");
        
        match (do_index, dont_index) {
            (Some(d), Some(dn)) => {
                // Set the multiply flag to the last do or don't instruction.
                multiply = d > dn;
            }
            (Some(_), None) => {
                multiply = true;
            }
            (None, Some(_)) => {
                multiply = false;
            }
            _ => {}
        }
        
        if multiply {
            instructions_to_execute.push(multiply_inst.clone());
        }
        
        // Last as it drops the reference for hay_substring
        hay.replace_range(0..end_of_mul, "");
    }
    
    let merged_inst: String = instructions_to_execute
        .iter()
        .flat_map(|s| {s.chars()})
        .collect();
    
    let result: u32 = sum_of_products(re, &*merged_inst);
    
    println!("Day 3 part two result: {}", result);
}

fn sum_of_products(re: Regex, hay: &str) -> u32 {
    re.captures_iter(hay).map(|captures| {
        let (_, [first, second]) = captures.extract();
        (first, second)
    }).map(|(first, second)| {
        let first = first.parse::<u32>().expect("Should be a value here");
        let second = second.parse::<u32>().expect("Should be a value here");
        (first, second)
    }).map(|(first, second)| {
        first * second
    }).sum()
}

fn read_input() -> String {
    let mut contents = String::new();
    let mut file = File::open("inputday3.txt").expect("Should be able to read file");
    file.read_to_string(&mut contents).expect("Should be able to get contents");
    contents
}