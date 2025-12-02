use std::io::Error;

use super::super::file_parser::parse_input_file;

struct Turn {
    direction: char,
    turns: i16,
}

fn new_turn(input: String) -> Result<Turn, Error> {
    let mut nums = input.clone();
    let direction = nums.remove(0);
    let turns = nums.parse().expect("should parse");
    Ok(Turn { direction, turns })
}

pub fn day_01() {
    let mut position: i16 = 50;
    let mut code = 0;

    let data = match parse_input_file("data/day_01.txt") {
        Ok(data) => data.into_iter().map(|x| new_turn(x).expect("didn't work")),
        Err(e) => panic!("{:}", e),
    };

    println!("starts at {}", position);

    for d in data {
        let sign = match d.direction {
            'L' => -1,
            'R' => 1,
            _ => 0,
        };

        position += sign * d.turns;

        while position > 99 {
            position -= 100
        }
        while position < 0 {
            position += 100
        }

        println!("got {} {}, turning to {}", d.direction, d.turns, position);

        if position == 0 {
            code += 1
        }
    }

    println!("code is {}", code);
}
