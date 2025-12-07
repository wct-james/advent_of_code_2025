use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::{Error, Result};

use super::super::file_parser::parse_input_file;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Turn {
    direction: Direction,
    number_of_turns: i16,
}

impl FromStr for Turn {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (direction_str, turns_str) = s.split_at(1);
        let direction: Direction = match direction_str {
            "L" => Direction::Left,
            "R" => Direction::Right,
            e => anyhow::bail!("unexpected character: {}", e),
        };

        let number_of_turns: i16 = match turns_str.parse() {
            Ok(t) => t,
            Err(e) => anyhow::bail!("couldn't parse string as number: {:}", e),
        };

        Ok(Turn {
            direction,
            number_of_turns,
        })
    }
}

pub fn day_01(file_name: &str) -> Result<String> {
    let mut position: i16 = 50;
    let mut code = 0;

    let turns: Vec<Turn> = match parse_input_file(file_name) {
        Ok(lines) => lines
            .into_iter()
            .map(|line| line.parse::<Turn>().map_err(Error::msg))
            .collect::<Result<Vec<Turn>, _>>()?,
        Err(e) => return Err(e.context("error parsing input file")),
    };

    for turn in turns {
        let sign = match turn.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        let increment = position + sign * turn.number_of_turns;

        position = increment.rem_euclid(100);

        if position == 0 {
            code += 1;
        }
    }

    Ok(format!("{}", code))
}

pub fn day_01_part_2(file_name: &str) -> Result<String> {
    let mut position: i16 = 50;
    let mut code = 0;

    let turns: Vec<Turn> = match parse_input_file(file_name) {
        Ok(lines) => lines
            .into_iter()
            .map(|line| line.parse::<Turn>().map_err(Error::msg))
            .collect::<Result<Vec<Turn>, _>>()?,
        Err(e) => return Err(e.context("error parsing input file")),
    };

    for turn in turns {
        let sign = match turn.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        for _ in 0..turn.number_of_turns {
            position = (position + sign).rem_euclid(100);
            if position == 0 {
                code += 1;
            }
        }
    }

    Ok(format!("{}", code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let answer = day_01("data/day_01_test.txt").expect("should work");
        assert_eq!(answer, "3");
    }

    #[test]
    fn part_2_example_1() {
        let answer = day_01_part_2("data/day_01_test.txt").expect("should work");
        assert_eq!(answer, "6");
    }
    #[test]
    fn part_2_example_2() {
        let answer = day_01_part_2("data/day_01_test_2.txt").expect("should work");
        assert_eq!(answer, "10")
    }
}
