mod modules;

use std::env;
use modules::search::Search;

// use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // Initialize the search module
    let mut search_module = Search {
        is_recursive: true,
        ignore_case: false,
        whole_word: false,
        search_str: String::new(),
        search_path: String::from("./"),
    };

    if args.len() < 2 {
        eprintln!("Usage: {} <search_string> [search_path]", args[0]);
        std::process::exit(1);
    } else if args.len() == 2 {
        search_module.search_str = args.remove(1);
    } else {

    /* 
    
        Parse the args: 
        Arg [0]: Program Name (ignore)
        Arg [1]: Search String
        Arg [2]: Search Path // Handle optional
    
        Any args that starts with - are optional args

    */

        // Remove the program name in abyss
        args.remove(0);
        
        search_module.search_str = args.remove(0);

        if !args[0].starts_with('-') {
            search_module.search_path = args.remove(0);
        } 

        for arg in args.iter() {
            match arg.as_str() {
                "--help" | "-h" => {
                    println!("Help message: This program processes command-line arguments.");
                    println!("Usage: program_name [OPTIONS] [ARGUMENTS]");
                    println!("  --help, -h: Show this help message.");
                    println!("  --version, -v: Show version information.");
                    // Add more options as needed
                } "--case-sensitive" | "-s" => {
                    search_module.ignore_case = false;
                } "--no-recursive" | "-r" => {
                    search_module.is_recursive = false;
                } "--whole-word" | "-w" => {
                    search_module.whole_word = true;
                } &_ => todo!()
            }
        }
        
        search_module.sgrep();
    }

    
}
