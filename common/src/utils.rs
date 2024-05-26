use crate::types::Result;

pub fn read_lines_from_file(filename: &str) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(filename)?.lines().map(String::from).collect::<Vec<String>>())
}