use anyhow::Result;

use crate::file_parser::parse_input_file;

fn is_repeating(elements: &[char], pattern_size: usize) -> bool {
    if pattern_size == 0 || elements.len() % pattern_size != 0 {
        return false;
    }

    let first_part = &elements[0..pattern_size];
    for start in (pattern_size..elements.len()).step_by(pattern_size) {
        if &elements[start..start + pattern_size] != first_part {
            return false;
        }
    }

    true
}

fn is_bad_id(id: i64) -> Option<i64> {
    let id_str = id.to_string();
    let id_str_length = id_str.len();
    let elements: Vec<char> = id_str.chars().collect();

    for pattern_size in 1..id_str_length {
        if is_repeating(&elements, pattern_size) {
            return Some(id);
        }
    }

    None
}

fn find_bad_ids(id_range: &str) -> Result<i64> {
    let ids: Vec<&str> = id_range.split("-").collect();
    if ids.len() != 2 {
        anyhow::bail!("invalid id range: {id_range}")
    };

    let first: i64 = ids[0].parse::<i64>()?;
    let last: i64 = ids[1].parse::<i64>()? + 1;

    let bad_ids = (first..last).filter_map(is_bad_id).sum();

    Ok(bad_ids)
}

pub fn day_02_part_02(file_name: &str) -> Result<i64> {
    let file_contents = parse_input_file(file_name)?;

    let ids: Vec<&str> = match file_contents.first() {
        Some(res) => res.split(',').collect(),
        None => anyhow::bail!("no contents in file?"),
    };

    let total: i64 = ids
        .iter()
        .map(|id_range| find_bad_ids(id_range))
        .try_fold(0i64, |acc, res| -> Result<i64> { Ok(acc + res?) })?;

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn example_1() {
        let result = day_02_part_02("data/day_02_test_2.txt").unwrap();
        assert_eq!(result, 33);
    }

    #[test]
    pub fn example_2() {
        let result = day_02_part_02("data/day_02_test_1.txt").unwrap();
        assert_eq!(result, 4174379265);
    }
}
