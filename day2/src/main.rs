use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    challenge1();
}

fn challenge1() {
    let mut score = 0;
    if let Ok(lines) = read_lines("input/input1.txt") {
        for line in lines{
            let curline:String = line.expect("Failed to parse line");
        }
    }
}

fn read_lines<P: std::convert::AsRef<std::path::Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename).expect("File not found...");
    Ok(io::BufReader::new(file).lines())
}