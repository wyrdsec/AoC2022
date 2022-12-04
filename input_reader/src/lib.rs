use std::fs::File;
use std::io::{self, BufRead};

#[macro_export]
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

pub fn read_lines<P: std::convert::AsRef<std::path::Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename).expect("File not found...");
    Ok(io::BufReader::new(file).lines())
}