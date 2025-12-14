use std::cmp;

use anyhow::Result;

use crate::file_parser::parse_input_file;

fn max_joltage(row: &str) -> Result<i64> {
    let mut maximum_joltage: i64 = 0;

    for first_char_idx in 0..row.len() {
        for second_char_idx in first_char_idx + 1..row.len() {
            let first_char = match row.chars().nth(first_char_idx) {
                Some(char) => char,
                None => anyhow::bail!("invalid index!!"),
            };
            let second_char = match row.chars().nth(second_char_idx) {
                Some(char) => char,
                None => anyhow::bail!("invalid index!!"),
            };
            let value: i64 = format!("{first_char}{second_char}").parse::<i64>()?;
            maximum_joltage = cmp::max(value, maximum_joltage);
        }
    }

    Ok(maximum_joltage)
}

pub fn day_03_part_01(file_name: &str) -> Result<i64> {
    let rows = parse_input_file(file_name)?;
    rows.iter()
        .try_fold(0, |acc, row| Ok(acc + max_joltage(row)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example_1() {
        let result = day_03_part_01("data/day_03_test.txt").unwrap();
        assert_eq!(result, 357);
    }
}
