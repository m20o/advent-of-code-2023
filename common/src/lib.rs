use std::fs;
use std::io::{self, BufRead};


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<std::path::Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
