use std::{fs::File, io, io::BufRead};

pub fn parse_input_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);

    reader.lines().collect()
}
