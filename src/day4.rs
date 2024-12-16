use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day4() {
    part_one();
}

fn part_one() {
    let word_search = read_input();
    let mut total: u32 = 0;
    
    total += horizontal_count(&word_search);
    total += vertical_count(&word_search);
    
    // Check diagonals going this "\" direction
    total += diagonal_count(&word_search);
    
    // Reverse so we check the diagonals going this "/" direction 
    for mut line in word_search {
        line.reverse()
    }
    total += diagonal_count(&word_search);

    println!("Count total: {total}");
}

fn horizontal_count(word_search: &[[char; 140]; 140]) -> u32 {
    let mut count = 0;

    // Check for horizontal "XMAS" or "SAMX"
    for line in word_search {
        let mut j = 0;
        
        while j <= line.len() - 4 {
            count += increment_count(&line[j.. j + 4]);
            j += 1;
        }
    }

    count
}

fn vertical_count(word_search: &[[char; 140]; 140]) -> u32 {
    let mut count = 0;
    let mut i = 0;
    
    while i <= word_search.len() - 4 {
        let mut j = 0;
        
        while j < word_search[i].len() {
            let word:[char; 4] = [
                word_search[i][j], 
                word_search[i + 1][j],
                word_search[i + 2][j],
                word_search[i + 3][j], 
            ];

            count += increment_count(&word);
            
            j += 1;
        } 
        
        i += 1;
    }
    
    count
}

fn diagonal_count(word_search: &[[char; 140]; 140]) -> u32 {
    let mut count = 0;
    let mut i = 0;

    while i <= word_search.len() - 4 {
        let mut j = 0;

        while j <= word_search[i].len() - 4 {
            let word:[char; 4] = [
                word_search[i][j],
                word_search[i + 1][j + 1],
                word_search[i + 2][j + 2],
                word_search[i + 3][j + 3],
            ];

            count += increment_count(&word);

            j += 1;
        }

        i += 1;
    }

    count
}

fn increment_count(word: &[char]) -> u32 {
    match word {
        ['X', 'M', 'A', 'S'] => { 1 }
        ['S', 'A', 'M', 'X'] => { 1 }
        _ => { 0 }
    }
}

fn read_input() -> [[char; 140]; 140] {
    // I measured the number of lines and columns for the input, 
    // in this case we're leaving it inflexible to work with arrays
    let mut input: [[char; 140]; 140] = [['b'; 140]; 140];

    let file = File::open("inputday4.txt")
        .expect("Should be a file");
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Should be a line here");
        for (j, character) in line.chars().enumerate() {
            input[i][j] = character;
        }
    }

    input
}