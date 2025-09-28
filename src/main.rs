use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ptr::read;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Print the program path
    println!("Program path: {}", args[0]);

    // Print other arguments
    if args.len() > 1 {
        println!("Other arguments:");
        for arg in args.iter().skip(1) {
            // Skip the first argument (program path)
            println!("  {}", arg);
        }
    } else {
        println!("No additional arguments provided.");
    }

    // Get directory entries
    get_directory_entries(&args[1]);
}

fn get_directory_entries(dir_path: &str) {
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        println!("{:?}", entry.path());
                        if (entry.file_type().unwrap().is_file()) {
                            let result =
                                read_and_search(entry.path().to_str().unwrap(), "some_search_term");
                            match result {
                                Ok(_) => println!("Search completed successfully."),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_and_search(filename: &str, search_term: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line?;
        if line_content.contains(search_term) {
            println!("Found '{}': {}", search_term, line_content);
        }
    }
    Ok(())
}
