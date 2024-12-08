use std::fs::File;
use std::io::Read;
use regex::Regex;

pub fn day3() {
    part_one();
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
    let result: u32 = re.captures_iter(&*hay).map(|captures| {
        let (_, [first, second]) = captures.extract();
        (first, second)
    }).map(|(first, second)| {
        let first = first.parse::<u32>().expect("Should be a value here");
        let second = second.parse::<u32>().expect("Should be a value here");
        (first, second)
    }).map(|(first, second)| {
        first * second
    }).sum();
    
    println!("Sum of the products is: {}", result);
}

fn read_input() -> String {
    let mut contents = String::new();
    let mut file = File::open("inputday3.txt").expect("Should be able to read file");
    file.read_to_string(&mut contents).expect("Should be able to get contents");
    contents
}