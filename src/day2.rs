use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2() {
    part_one();
    part_two();
}

// Worst case O(n) time complexity
fn part_one() {
    let mut safe_reports_count = 0;
    let reports = parse_reports();
    
    for mut report in reports {
        if safe_better(&mut report) {
            safe_reports_count += 1;
        }
    }
    println!("Number of safe reports: {}", safe_reports_count);
}

fn part_two() {
    let mut safe_reports_count = 0;
    let reports = parse_reports();

    for mut report in reports {
        if safe_dampened(&mut report) {
            safe_reports_count += 1;
        }
    }
    println!(
        "Number of safe reports with 1 bad level or less: {}",
        safe_reports_count
    );
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
#[allow(dead_code)]
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

// Thank you so much to CS Jackie on YT for this solution:
// https://www.youtube.com/watch?v=sghAbg0WKt8
fn safe_better(report: &mut VecDeque<i8>) -> bool {
    if report.len() <= 1 {
        return true
    }

    // Get the differences
    let mut diffs = Vec::new();
    let mut previous = report.pop_front().expect("Should be a value");
    
    for current in report {
        diffs.push(previous - *current);
        previous = *current;
    }
    
    let ascending = diffs.iter().all(
        |level| *level >= 1 && *level <= 3
    );
    let descending = diffs.iter().all(
        |level| *level <=-1 && *level >= -3
    );
    
    ascending || descending
}

// Thank you so much to CS Jackie on YouTube for this solution:
// https://www.youtube.com/watch?v=sghAbg0WKt8
// This solution has an O(n^2) worst case time complexity.
fn safe_dampened(report: &mut VecDeque<i8>) -> bool {
    // Need to make a deep copy, as safe_better pops one of the values from 
    // the report.
    let mut report_copy = report.clone();
    let safe = safe_better(&mut report_copy);
    
    if !safe {
        for (index, _level) in report.iter().enumerate() {
            let mut dampened_report = report.clone();
            dampened_report.remove(index);
            
            if safe_better(&mut dampened_report) {
                return true
            }
        }
        
        return false
    }
    
    safe
}