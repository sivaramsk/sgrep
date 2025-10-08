extern crate walkdir;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Search {
    pub(crate) is_recursive: bool,
    pub(crate) is_case_sensitive: bool,
    pub(crate) whole_word: bool,
    pub(crate) search_str: String,
}

impl Search {
    fn read_and_search(&self, file_path: &str) -> io::Result<()> {
        let path = Path::new(file_path);

        // Attempt to open the file. The '?' operator will return an Err if it fails.
        let file = File::open(&path)?;

        // Wrap the file in a BufReader for efficient, line-by-line reading.
        let reader = BufReader::new(file);
        let mut found = false;

        println!(
            "--- Searching for pattern '{}' in file: {} ---",
            self.search_str, file_path
        );

        // Iterate over the lines of the file. `lines()` returns a Result for each line.
        for line_result in reader.lines() {
            // Handle potential IO error while reading a line.
            let line = &self.search_str;

            // Check if the current line contains the pattern string.
            if line.contains(&self.search_str) {
                println!("✅ SUCCESS: Pattern found in line: \"{}\"", line.trim());
                found = true;
                // Break early once found to save time and resources.
                break;
            }
        }

        if !found {
            println!(
                "❌ NOT FOUND: The pattern '{}' was not found in the file.",
                self.search_str
            );
        }

        Ok(())
    }

    pub fn sgrep(&self, _dir: String) {
        for entry in WalkDir::new(_dir) // Or any other starting directory
            .into_iter()
            .filter_map(|e| e.ok())
        // Filter out errors
        {
            if entry.file_type().is_file() {
                if let Err(e) = self.read_and_search(&entry.path().to_string_lossy().into_owned()) {
                   eprintln!("Error during file search: {}", e);
                }
            }
        }
    }
}
