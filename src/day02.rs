// forward 5
// down 5
// forward 8
// up 3
// down 8
// forward 2
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, Copy, Clone, Default)]
struct Location {
    horizontal_position: usize,
    depth: usize,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let command = iter.next().unwrap();
        let distance = iter.next().unwrap().parse::<usize>().unwrap();

        match command {
            "forward" => Ok(Self::Forward(distance)),
            "up" => Ok(Self::Up(distance)),
            "down" => Ok(Self::Down(distance)),
            _ => Err(()),
        }
    }
}

fn navigate(commands: &[&str]) -> Location {
    let mut location = Location::default();

    // parse vector element
    // split with whitespace
    for cmd in commands {
        let cmd = Command::from_str(cmd).unwrap();
        match cmd {
            Command::Forward(n) => location.horizontal_position += n,
            Command::Up(n) => location.depth -= n,
            Command::Down(n) => location.depth += n,
        }
    }
    location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_command_from_string() {
        assert_eq!(Command::from_str("forward 1").unwrap(), Command::Forward(1));
    }

    #[test]
    fn going_forward_by_1() {
        let commands = vec!["forward 1"];
        let loc = navigate(&commands);
        assert_eq!(loc.horizontal_position, 1);
        assert_eq!(loc.depth, 0);
    }

    #[test]
    fn example_input() {
        let input = r##"forward 5
down 5
forward 8
up 3
down 8
forward 2"##;
        let commands = input.lines().collect::<Vec<_>>();
        let loc = navigate(&commands);
        assert_eq!(loc.horizontal_position, 15);
        assert_eq!(loc.depth, 10);
    }

    #[test]
    fn puzzle_input() {
        let input = include_str!("../input/day2.txt"); // "198\n201\n208\n..."
        let commands = input.lines().collect::<Vec<_>>();
        let loc = navigate(&commands);
        assert_eq!(loc.horizontal_position, 1967);
        assert_eq!(loc.depth, 1031);
        assert_eq!(loc.depth * loc.horizontal_position, 2027977);
    }
}