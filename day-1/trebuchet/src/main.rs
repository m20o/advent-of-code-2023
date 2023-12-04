use std::{env, fs};
use std::io::{self, BufRead};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file provided");
        exit(1);
    }
    let filename = &args[1];

    if let Ok(lines) = read_lines(filename) {
        let mut result = 0u32;
        for line in lines {
            if let Ok(value) = line.map(|txt| extract_calibration_value(txt.as_str())) {
                result += value;
            }
        }
        println!("The sum of all calibration value is {}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<std::path::Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn extract_calibration_value(text: &str) -> u32 {
    let digits: Vec<_> = text.chars()
        .filter(|c| c.is_numeric())
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    if digits.len() > 1 {
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        first * 10 + last
    } else {
        let digit = digits.first().unwrap();
        digit * 10 + digit
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_digits_at_beginning_and_end() {
        assert_eq!(extract_calibration_value("1abc2"), 12);
    }

    #[test]
    fn two_digits_in_the_middle() {
        assert_eq!(extract_calibration_value("pqr3stu8vwx"), 38);
    }

    #[test]
    fn multiple_digits() {
        assert_eq!(extract_calibration_value("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn one_digit() {
        assert_eq!(extract_calibration_value("treb7uchet"), 77);
    }
}