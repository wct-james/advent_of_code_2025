use advent_of_code_2025::days::day_01::{day_01, day_01_part_2};

fn main() {
    let day_one_file = "data/day_01.txt";
    let day_one_part_one = day_01(day_one_file).unwrap();
    println!("day 01, part 1: {:}", day_one_part_one);
    let day_one_part_two = day_01_part_2(day_one_file).unwrap();
    println!("day 01, part 2: {:}", day_one_part_two);
}
