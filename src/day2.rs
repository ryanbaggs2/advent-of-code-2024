use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2() {
    part_one();
}

fn part_one() {
    println!("{:?}", parse_reports());
}

fn parse_reports() -> Vec<Vec<i8>> {
    let mut reports: Vec<Vec<i8>> = Vec::new();
    let file = File::open("inputday2.txt")
        .expect("Should be able to read");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        reports.push(Vec::new());
        // We aren't handling the error case here to keep it simple
        let line = line.expect("Should be able to read line");
        let report = line.split_ascii_whitespace();
        for level in report {
            let level = level
                .parse::<i8>()
                .expect("All data within i8 bounds");
            reports.last_mut().expect("Should not be empty").push(level);
        }
    }
    reports
}