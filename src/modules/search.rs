extern crate walkdir;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Search {
    pub(crate) is_recursive: bool,
    pub(crate) ignore_case: bool,
    pub(crate) whole_word: bool,
    pub(crate) search_str: String,
    pub(crate) search_path: String,
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
            let line = line_result?;
            // Check if the current line contains the pattern string.
            if line.contains(&self.search_str) {
                println!("✅ SUCCESS: Pattern found in line: \"{:#?}\"", line);
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

    pub fn sgrep(&self) {
        for entry in WalkDir::new(self.search_path.as_str()) // Or any other starting directory
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
