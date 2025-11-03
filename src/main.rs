use execution_time::ExecutionTime;
use std::path::PathBuf;
use walkdir::DirEntry;

use b_encontrar_todas_as_nfes_do_cte::*;

/*
cargo fmt
cargo run
cargo doc --open
cargo b -r && cargo install --path=.
cargo test -- --show-output read_xml_content
*/

fn main() -> MyResult<()> {
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
