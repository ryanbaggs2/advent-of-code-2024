use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2() {
    part_one();
}

// Worst case O(n) time complexity
fn part_one() {
    let mut safe_reports_count = 0;
    let reports = parse_reports();
    
    for mut report in reports {
        if safe(&mut report) {
            safe_reports_count += 1;
        }
    }
    
    println!("Number of safe reports: {}", safe_reports_count);
}

// O(n) time complexity
fn parse_reports() -> Vec<VecDeque<i8>> {
    let mut reports: Vec<VecDeque<i8>> = Vec::new();
    let file = File::open("inputday2.txt")
        .expect("Should be able to read");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        reports.push(VecDeque::new());
        // We aren't handling the error case here to keep it simple
        let line = line.expect("Should be able to read line");
        let report = line.split_ascii_whitespace();
        for level in report {
            let level = level
                .parse::<i8>()
                .expect("All data within i8 bounds");
            reports.last_mut().expect("Should not be empty").push_back(level)
        }
    }
    reports
}

// O(n) time complexity
fn safe(report: &mut VecDeque<i8>) -> bool {
    let first_level = report[0];
    let second_level = report[1];
    let mut increasing = false;

    if first_level == second_level {
        return false
    }
    if first_level.abs_diff(second_level) > 3{
        return false
    }
    if first_level < second_level {
        increasing = true
    }
    
    safe_recursive(report, increasing)
}

fn safe_recursive(report: &mut VecDeque<i8>, increasing: bool) -> bool {
    // Base case
    if report.len() <= 1 {
        return true
    }
    
    let first_level = report[0];
    let second_level = report[1];
    
    if first_level == second_level {
        return false
    }
    if first_level.abs_diff(second_level) > 3 {
        return false
    }
    
    if first_level > second_level && increasing {
        return false
    }
    if first_level < second_level && !increasing {
        return false
    }
    
    // Remove value from report shrinking to base case
    report.pop_front();
    
    safe_recursive(report, increasing)
}