use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "sgrep",
    version,
    about = "Simple grep written in rust",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/sivaramsk/sgrep/issues",
    max_term_width = 98,
)]
pub struct Opts {
    /// The string to search for
    pub search_str: String,

    /// The path to search in
    #[arg(default_value = ".")]
    pub search_path: PathBuf,
    
    /// Case-sensitive search
    #[arg(short = 's', long)]
    pub case_sensitive: bool,

    /// Not recursive
    #[arg(short = 'r', long="no-recursive")]
    pub no_recursive: bool,

    /// Whole word search
    #[arg(short = 'w', long)]
    pub whole_word: bool,
}