use std::fs;

/// Reads a file
/// 
/// # Arguments
/// 
/// * `path` - The path of the file
/// 
/// # Returns
/// 
/// * `Result<String, std::io::Error>` - The result of the operation
/// 
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    return Ok(contents);
}

/// Checks if a file exists
/// 
/// # Arguments
/// 
/// * `path` - The path of the file
/// 
/// # Returns
/// 
/// * `bool` - true if the file exists, false otherwise
/// 
pub fn file_exists(path: &str) -> bool {
    return fs::metadata(path).is_ok();
}