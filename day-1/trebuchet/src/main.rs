use std::env;
use std::process::exit;

use common::read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file provided");
        exit(1);
    }
    let filename = &args[1];

    if let Ok(lines) = read_lines(filename) {
        let mut result: u32 = 0u32;
        for line in lines {
            if let Ok(value) = line.map(|txt| extract_calibration_value(txt.as_str())) {
                result += value;
            }
        }
        println!("The sum of all calibration values is {}", result);
    }
}

fn extract_calibration_value(line: &str) -> u32 {
    let digits: Vec<_> = line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .chars()
        .filter(|c| c.is_numeric())
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    first * 10 + last
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

    #[test]
    fn one_digit_and_one_letter() {
        assert_eq!(extract_calibration_value("two1nine"), 29);
    }

    #[test]
    fn three_letters() {
        assert_eq!(extract_calibration_value("eightwothree"), 83);
    }

    #[test]
    fn letters_and_numbers_in_the_middle() {
        assert_eq!(extract_calibration_value("abcone2threexyz"), 13);
    }

    #[test]
    fn letters_and_numbers_in_the_middle_2() {
        assert_eq!(extract_calibration_value("xtwone3four"), 24);
    }

    #[test]
    fn letters_and_numbers_in_the_middle_3() {
        assert_eq!(extract_calibration_value("4nineeightseven2"), 42);
    }

    #[test]
    fn letters_and_numbers_in_the_middle_4() {
        assert_eq!(extract_calibration_value("zoneight234"), 14);
    }

    #[test]
    fn letters_and_numbers_in_the_middle_5() {
        assert_eq!(extract_calibration_value("7pqrstsixteen"), 76);
    }

    #[test]
    fn sum_all_calibration_values() {
        let values = [
            "two1nine     ",
            "eightwothree     ",
            "abcone2threexyz  ",
            "xtwone3four      ",
            "4nineeightseven2 ",
            "zoneight234      ",
            "7pqrstsixteen    "
        ];

        let sum: u32 = values.iter().map(|v| extract_calibration_value(v.trim()))
            .sum();
        assert_eq!(sum, 281);
    }
}