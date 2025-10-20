extern crate walkdir;

use env_logger::DEFAULT_WRITE_STYLE_ENV;
use memmap2::MmapOptions;
use regex::bytes::Regex;
use std::fs::File;
use std::io;
// use std::io::{self, BufRead, BufReader};
// use std::path::Path;
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
    fn search_2(&self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path).expect("Failed to open file");
        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .expect("Failed to memory map file")
        };
        let data: &[u8] = &mmap; // The file content as a byte slice

        let re = Regex::new(&self.search_str).expect("Invalid regex pattern");

        // Iterate over all matches
        for mat in re.find_iter(data) {
            let line_number = Search::calculate_line_number(data, mat.start());

            println!(
                "Found match: {:?} in {:#?} line {}",
                std::str::from_utf8(mat.as_bytes()).unwrap(),
                file_path,
                line_number
            );
        }

        Ok(())
    }

    fn calculate_line_number(content: &[u8], match_offset: usize) -> usize {
        // 1. Ensure the offset is within the bounds of the content.
        if match_offset > content.len() {
            // This case shouldn't happen in a typical search, but it's good practice.
            return 0;
        }

        // 2. Take the slice of content from the start (0) up to the match_offset.
        // This is the segment we need to count newlines in.
        let preceding_content = &content[..match_offset];

        // 3. Count the number of newline characters ('\n') in the preceding content.
        // This uses an efficient iterator method.
        let newline_count = preceding_content
            .iter()
            // Check if the byte is the ASCII newline character (0xA).
            .filter(|&&byte| byte == b'\n')
            .count();

        // 4. The line number is (number of newlines found) + 1, because:
        //    - 0 newlines means we are on line 1.
        //    - 1 newline means line 1 has ended, and we are on line 2.
        newline_count + 1
    }

    // fn read_and_search(&self, file_path: &str) -> io::Result<()> {
    //     let path = Path::new(file_path);

    //     // Attempt to open the file. The '?' operator will return an Err if it fails.
    //     let file = File::open(&path)?;

    //     // Wrap the file in a BufReader for efficient, line-by-line reading.
    //     let reader = BufReader::new(file);
    //     let mut found = false;

    //     println!(
    //         "--- Searching for pattern '{}' in file: {} ---",
    //         self.search_str, file_path
    //     );

    //     // Iterate over the lines of the file. `lines()` returns a Result for each line.
    //     for (index, line_result) in reader.lines().enumerate() {
    //         let line = line_result.unwrap();
    //         // Check if the current line contains the pattern string.
    //         if line.contains(&self.search_str) {
    //             println!("✅ SUCCESS: Pattern found in line: \"{:#?}\" in {}", line, index);
    //             found = true;
    //             // Break early once found to save time and resources.
    //             break;
    //         }
    //     }

    //     if !found {
    //         eprintln!(
    //             "❌ NOT FOUND: The pattern '{}' was not found in the file.",
    //             self.search_str
    //         );
    //     }

    //     Ok(())
    // }

    pub fn sgrep(&self) {
        for entry in WalkDir::new(self.search_path.as_str()) // Or any other starting directory
            .into_iter()
            .filter_map(|e| e.ok())
        // Filter out errors
        {
            if entry.file_type().is_file() {
                if let Err(e) = self.search_2(&entry.path().to_string_lossy().into_owned()) {
                    eprintln!("Error during file search: {}", e);
                }
            }
        }
    }
}
