mod modules;
mod cli;

use clap::Parser;
use cli::Opts;
use modules::search::Search;

fn main() {
    let opts = Opts::parse();

    let search_module = Search {
        is_recursive: !opts.no_recursive,
        ignore_case: !opts.case_sensitive,
        whole_word: opts.whole_word,
        search_str: opts.search_str,
        search_path: opts.search_path.to_str().unwrap().to_string(),
    };

    search_module.sgrep();
}
