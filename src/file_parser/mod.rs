use anyhow::{Context, Result};
use std::{fs::File, io, io::BufRead};

pub fn parse_input_file(file_name: &str) -> Result<Vec<String>> {
    let file =
        File::open(file_name).with_context(|| format!("couldn't open file {}", file_name))?;
    let reader = io::BufReader::new(file);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}
