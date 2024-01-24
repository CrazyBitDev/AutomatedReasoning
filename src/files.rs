use std::fs;
use std::fs::File;


pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    return Ok(contents);
}