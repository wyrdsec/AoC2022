use std::fs::File;
use std::io::{self, BufRead};

enum Hand {
    Win,
    Loss,
    Tie
}

enum HandSign {
    Rock,
    Paper,
    Scissors
}

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    challenge1();
    challenge2();
}

fn challenge1() {
    let mut score = 0;
    if let Ok(lines) = read_lines("input/input1.txt") {
        for line in lines{
            let curline:String = line.expect("Failed to parse line");
            let cur_round = scan!(curline, char::is_whitespace, char, char);
            
            let op:char = cur_round.0.expect("Not Valid char read.");
            let me:char = cur_round.1.expect("Not Valid char read.");
            let result:Hand = round(op,me);

            score += me as u32 - 87;
            score += match result {
                Hand::Win => 6,
                Hand::Tie => 3,
                Hand::Loss => 0
            }
        }
    }
    println!("Score = {}",score);
}

fn challenge2() {
    let mut score = 0;
    if let Ok(lines) = read_lines("input/input1.txt") {
        for line in lines{
            let curline:String = line.expect("Failed to parse line");
            let cur_round = scan!(curline, char::is_whitespace, char, char);
            
            let op:char = cur_round.0.expect("Not Valid char read.");
            let res:char = cur_round.1.expect("Not Valid char read.");
            let result:HandSign = get_round(op,res);

            score += match res {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => panic!("Char {} not a valid result!", res)
            };
            score += match result {
                HandSign::Rock => 1,
                HandSign::Paper => 2,
                HandSign::Scissors => 3
            };
        }
    }
    println!("Score = {}",score);
}

fn round(op:char, me:char) -> Hand {
    match op {
        'A' => {
            match me {
                'X' => {Hand::Tie}
                'Y' => {Hand::Win}
                'Z' => {Hand::Loss}
                _ => {panic!("Char {} not valid hand!", op)}
            }
        }
        'B' => {
            match me {
                'X' => {Hand::Loss}
                'Y' => {Hand::Tie}
                'Z' => {Hand::Win}
                _ => {panic!("Char {} not valid hand!", op)}
            }
        }
        'C' => {
            match me {
                'X' => {Hand::Win}
                'Y' => {Hand::Loss}
                'Z' => {Hand::Tie}
                _ => {panic!("Char {} not valid hand!", op)}
            }
        }
        _ => {panic!("Char {} not valid hand!", op)}
    }
}

fn get_round(op:char, res:char) -> HandSign {
    match op {
        'A' => {
            match res {
                'X' => {HandSign::Scissors}
                'Y' => {HandSign::Rock}
                'Z' => {HandSign::Paper}
                _ => {panic!("Char {} not valid hand!", op)}
            }
        }
        'B' => {
            match res {
                'X' => {HandSign::Rock}
                'Y' => {HandSign::Paper}
                'Z' => {HandSign::Scissors}
                _ => {panic!("Char {} not valid hand!", op)}
            }
        }
        'C' => {
            match res {
                'X' => {HandSign::Paper}
                'Y' => {HandSign::Scissors}
                'Z' => {HandSign::Rock}
                _ => {panic!("Char {} not valid hand!", op)}
            }
        }
        _ => {panic!("Char {} not valid hand!", op)}
    }
}


fn read_lines<P: std::convert::AsRef<std::path::Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename).expect("File not found...");
    Ok(io::BufReader::new(file).lines())
}