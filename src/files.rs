use std::fs;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    return Ok(contents);
}