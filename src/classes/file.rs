use std::fs;
use std::io::prelude::*;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


pub struct File {
    name: String,
    file: Option<fs::File>,
}

impl File {
    pub fn new(name: Option<String>) -> File {
        let file_name = name.unwrap_or(Self::random_name());

        File {
            name: file_name,
            file: None
        }
    }

    /// Generates a random name for the file
    /// 
    /// # Returns
    /// 
    /// * `String` - The random name
    /// 
    fn random_name() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    /// Checks if the file is created
    /// 
    /// # Returns
    /// 
    /// * `bool` - true if the file is created, false otherwise
    /// 
    pub fn is_created(&self) -> bool {
        return self.file.is_some();
    }

    /// Creates the file
    pub fn create(&mut self) {
        let file = fs::File::create(self.name.clone())
            .expect("Error on file creation");
        self.file = Some(file);
    }

    /// get the path of the file
    pub fn get_path(&self) -> String {
        return self.name.clone();
    }

    /// Clears the file content
    pub fn clear(&mut self) {
        if let Some(file) = &mut self.file {
            file.set_len(0)
                .expect("Error on file clear");
        }
    }

    /// Writes the contents to the file (overwrites the file)
    /// 
    /// # Arguments
    /// 
    /// * `contents` - The contents to write
    /// 
    pub fn write_all(&mut self, contents: &str) {
        if let Some(file) = &mut self.file {
            file.write_all(contents.as_bytes())
                .expect("Error on file write");
        }
    }

    /// Appends the contents to the file
    /// 
    /// # Arguments
    /// 
    /// * `contents` - The contents to append
    /// 
    pub fn write(&mut self, contents: &str) {
        if let Some(file) = &mut self.file {
            file.write(contents.as_bytes())
                .expect("Error on file append");
        }
    }

    /// Writes the contents to the file and adds a newline
    /// 
    /// # Arguments
    /// 
    /// * `contents` - The contents to write
    /// 
    pub fn writeln(&mut self, contents: &str) {
        self.write(&format!("{}\n", contents));
    }

    /// Renames the file
    /// 
    /// # Arguments
    /// 
    /// * `new_name` - The new name
    /// 
    pub fn rename(&mut self, new_name: String) {
        if self.is_created() {
            fs::rename(&self.name, &new_name)
                .expect("Error on file rename");
            self.name = new_name;
        }
    }

    /// Randomizes the file name
    pub fn randomize_name(&mut self) {
        let new_name = Self::random_name();
        self.rename(new_name);
    }

    /// Deletes the file
    pub fn delete(&mut self) {
        if self.is_created() {
            fs::remove_file(&self.name)
                .expect("Error on file delete");
            self.file = None;
        }
    }
}