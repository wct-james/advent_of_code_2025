use std::cmp;

use anyhow::Result;

use crate::file_parser::parse_input_file;

struct Battery {
    index: usize,
    value: u32,
}

fn to_battery(index: usize, value_char: char) -> Result<Battery> {
    let value = match value_char.to_digit(10) {
        Some(v) => v,
        None => anyhow::bail!("couldn't convert {:?} to digit", value_char),
    };
    Ok(Battery { index, value })
}

fn max_joltage(row: &str) -> Result<i64> {
    // create vec of batteries to sort on
    let mut batteries: Vec<Battery> = Vec::new();
    for idx in 0..row.len() {
        let value = match row.chars().nth(idx) {
            Some(v) => v,
            None => anyhow::bail!("couldn't get nth battery"),
        };
        let battery = to_battery(idx, value)?;
        batteries.push(battery);
    }

    batteries.sort_by(|a, b| a.value.cmp(&b.value));
    let twelve_top = match batteries.first_chunk::<12>() {
        Some(twelve_top) => twelve_top,
        None => anyhow::bail!("couldn't get top twleve!"),
    };

    // twelve_top..sort_by(|a, b| b.index.cmp(&a.index));

    Ok(1)
}

pub fn day_03_part_02(file_name: &str) -> Result<i64> {
    let rows = parse_input_file(file_name)?;
    rows.iter()
        .try_fold(0, |acc, row| Ok(acc + max_joltage(row)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example_1() {
        let result = day_03_part_02("data/day_03_test.txt").unwrap();
        assert_eq!(result, 3121910778619);
    }
}
