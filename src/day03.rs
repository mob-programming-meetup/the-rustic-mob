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

fn most_frequent_digit(lines: &[&str], column: usize) -> char {}

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

    #[test]
    fn test_count_ones() {
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
        assert_eq!(7, count_ones(&lines, 0));
        assert_eq!(5, count_ones(&lines, 1));
        assert_eq!(7, count_ones(&lines, 3));

        assert_eq!('1', most_frequent_digit(&lines, 0));
        assert_eq!('0', most_frequent_digit(&lines, 1));
    }

    #[test]
    fn test_count_zeros() {
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
        assert_eq!(7, count_ones(&lines, 0));
        assert_eq!(5, count_ones(&lines, 1));
        assert_eq!(7, count_ones(&lines, 3));
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
