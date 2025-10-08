mod modules;

use std::env;
use modules::search::Search;

// use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Print the program path
    println!("Program path: {}", args[0]);

    if args.len() < 2 {
            eprintln!("Usage: {} <search_pattern> <search_path>", args[0]);
            std::process::exit(1); // Exit with an error code
        }

    let search_str: String = args.get(1)
        .map(|s| s.trim().to_string()) // Trim whitespace from the argument
        .filter(|s| !s.is_empty())
        .unwrap();

    let search_module = Search{ is_recursive: true, is_case_sensitive: false, whole_word: false, search_str: search_str };

    // Define the default value
    let default_path = String::from("./");

    // Get the second argument (index 2)
    let search_path: String = args.get(2)
        .map(|s| s.trim().to_string()) // Trim whitespace from the argument
        .filter(|s| !s.is_empty()) // Filter out empty strings
        .unwrap_or(default_path); 

    search_module.sgrep(search_path);
}
