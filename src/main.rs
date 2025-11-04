use execution_time::ExecutionTime;
use std::{path::PathBuf, process};
use walkdir::DirEntry;

use b_encontrar_todas_as_nfes_do_cte::*;

/*
cargo fmt
cargo run
cargo doc --open
cargo b -r && cargo install --path=.
cargo test -- --show-output read_xml_content
*/

fn main() {
    // Call the separate function that contains the main logic and can return Result
    let run_result = run();

    // Now handle the result returned by the 'run' function
    match run_result {
        Ok(_) => {
            println!("All requested operations finished.");
            process::exit(0); // Explicitly exit with success code
        }
        Err(error) => {
            eprintln!("Operation failed:");
            eprintln!("Error: {}", error); // Using Display prints the #[error] message
            process::exit(1); // Explicitly exit with failure code
        }
    }
}

fn run() -> MyResult<()> {
    let timer = ExecutionTime::start();

    let mut config = Config::new();
    config.print_initial_info();

    let xml_entries: Vec<DirEntry> = get_walkdir_entries("xml")?;

    let array_of_files: Vec<PathBuf> = get_filtered_files(&xml_entries, &REGEX_CTE_PROC)?;

    config.get_num_files(&array_of_files)?;

    let complementary_key_for_cte = fold_reduce(&array_of_files);

    print_output(&complementary_key_for_cte, &config)?;

    timer.print_elapsed_time();

    Ok(())
}
