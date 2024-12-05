use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

pub fn day2() {
    part_one();
    part_two();
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

fn part_two() {
    let mut safe_reports_count = 0;
    let reports = parse_reports();

    for mut report in reports {
        if safe_p2(&mut report) {
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

// O(n) time complexity
fn safe_p2(report: &mut VecDeque<i8>) -> bool {
    let mut previous_level = report[0];
    report.pop_front();
    
    let mut differences: Vec<i8> = Vec::new();
    let mut positive_quantity = 0;
    let mut negative_quantity = 0;
    let mut bad_levels = 0;
    
    for current_level in report {
        differences.push(previous_level - *current_level);
        previous_level = *current_level;
    }
    
    for diff in differences {
        if diff == 0 || diff.abs() > 3 {
            bad_levels += 1;
            // It's already a bad level, we don't need to include it in the increasing or
            // decreasing checks
            continue;
        }
        if diff > 0 {
            positive_quantity += 1;
        }
        if diff < 0 {
            negative_quantity += 1;
        }
    }
    
    // Include the bad levels caught for increasing/decreasing
    bad_levels = bad_levels + cmp::min(positive_quantity, negative_quantity);
    
    if bad_levels > 1 {
        return false
    }
    
    true
}