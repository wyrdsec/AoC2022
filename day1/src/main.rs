use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main(){
    challenge_1();
    challenge_2();
}

fn challenge_1() {
    let mut max:u64 = 0;
    let mut sum:u64 = 0;
    if let Ok(lines) = read_lines("input/input1.txt") {
        for line in lines{
            let curline:String = line.unwrap();
            if curline == "" {
                max = if sum > max { sum } else { max };
                sum = 0;
            }
            else {
                sum += curline.parse::<u64>().expect(format!("Failed to parse integer: {}", curline).as_str());
            }
        }
    }
    println!("Maximal Calories: {}", max)
}

fn challenge_2() {
    let mut calories:Vec<u64> = Vec::new();
    let mut sum:u64 = 0;
    if let Ok(lines) = read_lines("input/input1.txt") {
        for line in lines{
            let curline:String = line.unwrap();
            if curline == "" {
                calories.push(sum);
                sum = 0;
            }
            else {
                sum += curline.parse::<u64>().expect(format!("Failed to parse integer: {}", curline).as_str());
            }
        }
    }
    calories.sort();
    calories.reverse();
    let max:u64 = calories[0] + calories[1] + calories[2];
    println!("Maximal Calories: {}", max);
}


fn read_lines<P: std::convert::AsRef<std::path::Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename).expect("File not found...");
    Ok(io::BufReader::new(file).lines())
}