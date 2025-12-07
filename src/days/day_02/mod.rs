use anyhow::Result;

use crate::file_parser::parse_input_file;

fn is_bad_id(id: i64) -> Option<i64> {
    let id_str: Vec<char> = id.to_string().chars().collect();

    if id_str.len() % 2 != 0 {
        return None;
    }

    let (first, second) = id_str.split_at(id_str.len() / 2);

    if first == second {
        return Some(id);
    }

    None
}

fn find_bad_ids(id_range: &str) -> Result<i64> {
    let ids: Vec<i64> = id_range
        .split('-')
        .map(|x| x.parse::<i64>())
        .collect::<Result<_, _>>()?;
    if ids.len() != 2 {
        anyhow::bail!(format!("invalid id range: {}", id_range));
    }

    let first = ids[0];
    let last = ids[1] + 1;

    let bad_ids = (first..last).filter_map(is_bad_id).sum();

    Ok(bad_ids)
}

pub fn day_02(file_name: &str) -> Result<i64> {
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
        let result = day_02("data/day_02_test_1.txt").unwrap();
        assert_eq!(result, 1227775554);
    }
}
