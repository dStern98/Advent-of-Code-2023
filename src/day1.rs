use crate::{read_input_file, SolveAdvent};

pub struct Day1;

impl SolveAdvent for Day1 {
    fn solve_part1(path_to_file: &str) {
        let mut sum_of_calibration_values = 0;
        let files_as_str = read_input_file(path_to_file);
        for line in files_as_str.lines() {
            sum_of_calibration_values += process_line_part1(line);
        }
        println!("Sum of Calibration Values: {}", sum_of_calibration_values);
    }

    fn solve_part2(path_to_file: &str) {
        let mut sum_of_calibration_values = 0;
        let files_as_str = read_input_file(path_to_file);
        for line in files_as_str.lines() {
            sum_of_calibration_values += process_line_part2(line);
        }
        println!("Sum of Calibration Values: {}", sum_of_calibration_values);
    }
}

const DIGIT_MAPPER: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn process_line_part1(line: &str) -> usize {
    //Filter out the digits from the rest of the line contents.
    let digits = line
        .chars()
        .filter(|item| item.is_ascii_digit())
        .collect::<Vec<_>>();
    //Return the first and last digits concatenated together, converted to a number.
    vec![digits[0], digits[digits.len() - 1]]
        .into_iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn get_last_digit(line: &str) -> char {
    //! An inefficient but simple way to get the desired char.
    let mut line_copy = line.to_owned();
    while !line_copy.is_empty() {
        //If the line endswith with a digit as string, return the corresponding
        //digit.
        for (digit_name, digit_char) in DIGIT_MAPPER {
            if line_copy.ends_with(digit_name) {
                return digit_char;
            }
        }
        //Remove the last char from the string.
        let last_char = line_copy.remove(line_copy.len() - 1);
        if last_char.is_ascii_digit() {
            return last_char;
        }
    }
    panic!("There was no trailing digit in the line!");
}

fn get_first_digit(line: &str) -> char {
    let mut line_copy = line.to_owned();
    while !line_copy.is_empty() {
        //If the line starts with a digit as string, return the corresponding
        //digit.
        for (digit_name, digit_char) in DIGIT_MAPPER {
            if line_copy.starts_with(digit_name) {
                return digit_char;
            }
        }
        //Else, remove the first char, returning if an ascii digit.
        let first_char = line_copy.remove(0);
        if first_char.is_ascii_digit() {
            return first_char;
        }
    }
    panic!("Line did not contain any digits!");
}

fn process_line_part2(line: &str) -> usize {
    let mut concatenated_digits = String::new();
    concatenated_digits.push(get_first_digit(line));
    concatenated_digits.push(get_last_digit(line));
    concatenated_digits.parse::<usize>().unwrap()
}
