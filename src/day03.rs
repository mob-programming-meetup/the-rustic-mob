struct Counter {
    zeros: usize,
    ones: usize,
}

impl Counter {
    fn new() -> Self {
        Self { zeros: 0, ones: 0 }
    }
}

fn get_digit_at_position(input: &str, column: usize) -> char {
    input.chars().nth(column).unwrap()
}

/*
return number of lines with bit in position column set.
*/
fn count_ones(lines: &[&str], column: usize) -> usize {
    lines
        .iter()
        .filter(|&&line| get_digit_at_position(line, column) == '1')
        .count()
}

use std::cmp::Ordering;

fn most_frequent_digit(lines: &[&str], column: usize) -> char {
    let number_of_ones = count_ones(lines, column);
    match number_of_ones.cmp(&(lines.len() / 2)) {
        Ordering::Greater => '1',
        Ordering::Less => '0',
        Ordering::Equal => unimplemented!("Same occurrence of zeros and ones."),
    }
}

fn number_of_columns(lines: &[&str]) -> usize {
    lines[0].len()
}

fn gamma(lines: &[&str]) -> usize {
    let mut gamma = 0;
    let number_of_columns = number_of_columns(lines);
    for column in 0..number_of_columns {
        let digit = most_frequent_digit(lines, column);
        let digit = if digit == '1' {
            1
        } else if digit == '0' {
            0
        } else {
            unimplemented!("Same occurrence of zeros and ones.");
        };
        gamma = gamma * 2 + digit;
    }

    gamma
}

fn epsilon(lines: &[&str], gamma: usize) -> usize {
    let number_of_columns = number_of_columns(lines);
    let mask = 2_usize.pow(number_of_columns as u32) - 1; // e.g. for 5 columns : 0b00000111111
    gamma ^ mask
}

pub fn power_consumption(lines: &[&str]) -> usize {
    let gamma = gamma(lines);
    let epsilon = epsilon(lines, gamma);
    gamma * epsilon
}

//   lines: Vec<&str>        Vec<&str> { size, capacity, pointer: Box<&[&str]> }       pointer -> [element1, element2]             element1 -> "hello"     element2 -> "world"
//
//
//
//   "hello"                          "world"
//
//   lines: &[element1, element2]  --> Vec<[element2]>
//

fn filter_column_by_most_common_digit<'a>(lines: &[&'a str], column: usize) -> Vec<&'a str> {
    let filteredLines: Vec<&str> = vec![];

    filteredLines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_ones() {
        // "000\n001\n010"
        // let mut counter = Counter::new();
        // counter.accept("0");
        // assert_eq!(0, counter.count_ones(0));
    }

    #[test]
    fn create_counter() {
        // let counter = Counter::new();
        // assert_eq!(0, counter.count_ones(0));
    }

    #[test]
    fn test_get_digit_at_position() {
        assert_eq!('0', get_digit_at_position("010", 2));
        assert_eq!('1', get_digit_at_position("010", 1));
    }

    fn test_input() -> Vec<&'static str> {
        let lines = r##"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"##
        .lines()
        .collect::<Vec<_>>();
        lines
    }

    #[test]
    fn test_count_ones() {
        let lines = test_input();
        assert_eq!(7, count_ones(&lines, 0));
        assert_eq!(5, count_ones(&lines, 1));
        assert_eq!(7, count_ones(&lines, 3));
    }

    #[test]
    fn test_most_frequent_digit() {
        let lines = test_input();
        assert_eq!('1', most_frequent_digit(&lines, 0));
        assert_eq!('0', most_frequent_digit(&lines, 1));
    }

    #[test]
    fn test_count_zeros() {
        let lines = test_input();
        assert_eq!(7, count_ones(&lines, 0));
        assert_eq!(5, count_ones(&lines, 1));
        assert_eq!(7, count_ones(&lines, 3));
    }

    #[test]
    fn test_number_of_columns() {
        let lines = test_input();
        assert_eq!(5, number_of_columns(&lines));
    }

    #[test]
    fn test_gamma() {
        let lines = test_input();
        assert_eq!(22, gamma(&lines));
    }

    #[test]
    fn test_epsilon() {
        let lines = test_input();
        assert_eq!(9, epsilon(&lines, 22));
    }

    #[test]
    fn test_power_consumption() {
        let lines = test_input();
        assert_eq!(198, power_consumption(&lines));
    }

    #[test]
    fn test_power_consumption_input_file() {
        let lines = include_str!("../input/day3.txt")
            .lines()
            .collect::<Vec<_>>();
        assert_eq!(3277364, power_consumption(&lines));
    }

    #[test]
    fn test_filter_column_by_most_common_digit() {
        let lines = test_input();
        assert_eq!(7, filter_column_by_most_common_digit(&lines, 0).len());
    }
}

/* Ideas for counting:
 * dimitry: vec of counters, each counter is responsible for a column, Counters are fed linewise
 * henning: shell around vec of counters => counter_manager
 *
 * maybe initialze counter with column index
 * just have an accumulator which is a Vec of values, iterate over each line of input, process left2right of each line
 * no struct, just functions
 *
 * koushik: have a function that returns the number of samples for which a certain bit/digit/column is set
 *
 *
 * How to make a number from the counts?
 *
 */
