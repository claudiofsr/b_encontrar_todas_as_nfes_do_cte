mod config;
mod error;
mod reg_ex;

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use rayon::prelude::*;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

pub use self::{
    config::Config,
    error::{MyError, MyResult},
    reg_ex::*,
};

const CODE_NFE: &str = "55";
const CODE_CTE: &str = "57";
const NUM_DIGITS: usize = 44;

/// Gets a vector of `DirEntry` for files with a specified format from the current directory and its subdirectories.
///
/// This function uses `walkdir` crate to traverse directories and filters entries based on file type and format.
///
/// # Arguments
///
/// * `file_format` - A string representing the file extension to filter by (e.g., "xml").
///
/// # Returns
///
/// A `Result` containing a `Vec<DirEntry>` if successful or an `io::Error` if an error occurs.
pub fn get_walkdir_entries(file_format: &str) -> MyResult<Vec<DirEntry>> {
    let dir_path: PathBuf = std::env::current_dir()?;

    let entries: Vec<DirEntry> = WalkDir::new(dir_path)
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case(file_format))
        })
        .collect();

    Ok(entries)
}

/// Get filtered files after analyzing their content
///
/// Filters files based on their content using a regular expression.
///
/// This function takes a slice of `DirEntry` and a regular expression, reads the content of each
/// file, and if the regex matches, adds the file path to the filtered vector of files.
///
/// # Arguments
///
/// * `xml_entries` - A slice of `DirEntry` representing the files to be processed.
/// * `regex` - A lazy-loaded regular expression used to match content in files.
///
/// # Returns
///
/// A `Result` containing a `Vec<PathBuf>` of filtered file paths if successful or a `MyError` if an error occurs.
pub fn get_filtered_files(
    xml_entries: &[DirEntry],
    regex: &LazyLock<Regex>,
) -> MyResult<Vec<PathBuf>> {
    xml_entries
        .into_par_iter()
        .map(|entry: &DirEntry| {
            let xml_path: PathBuf = entry.path().to_path_buf();

            // Reads the entire contents of a file into a string.
            fs::read_to_string(&xml_path)
                .map_err(|error| MyError::FileReadError(xml_path.clone(), error))
                .map(|contents| {
                    if regex.is_match(&contents) {
                        Some(xml_path)
                    } else {
                        None
                    }
                })
        })
        .collect::<Result<Vec<Option<PathBuf>>, MyError>>() // Coleta os resultados de cada thread, agrupando erros
        .map(|options| options.into_iter().flatten().collect()) // Filtra os None e coleta os Some(PathBuf)
}

/**
Reads the entire content of a file into a String.

1. impl AsRef<Path>: `slurp` aceita qualquer tipo que implemente AsRef<Path>, o que inclui:
   &Path,
   &str,
   String e
   PathBuf, entre outros.

2. path.as_ref(): Converte o parâmetro de entrada para &Path

3. to_path_buf(): realiza conversão evitando cópias desnecessárias quando o Path já é derivado de um PathBuf.

4. map_err: A função continua usando map_err para transformar um io::Error em um MyError::FileReadError com o PathBuf correto.
*/
fn slurp(path: impl AsRef<Path>) -> MyResult<String> {
    fs::read_to_string(path.as_ref()).map_err(|err| {
        let path_buf = path.as_ref().to_path_buf();
        MyError::FileReadError(path_buf, err)
    })
}

/// Verifies the format and code of a fiscal key.
///
/// This function takes a path, a fiscal key, and an expected code, verifies if the key
/// has the correct format and contains the expected code. It returns `Ok(true)` if the key
/// is valid or an `Err(MyError::InvalidFiscalKey)` if the key is invalid.
///
/// # Arguments
///
/// * `path` - A value that implements `AsRef<Path>`, representing the file path where the key was found.
/// * `key` - The fiscal key to be verified.
/// * `expected_code` - The expected code in the fiscal key.
///
/// # Returns
///
/// `Ok(true)` if the key is valid, or an `Err(MyError::InvalidFiscalKey)` if the key is invalid.
fn verify_key_code(path: impl AsRef<Path>, key: &str, expected_code: &str) -> MyResult<bool> {
    let key_numeric = REGEX_REMOVE_NON_DIGITS.replace_all(key, "");

    let code: &str = REGEX_CODE
        .captures(&key_numeric)
        .and_then(|cap| cap.get(1))
        .map_or("", |m| m.as_str());

    if (key_numeric.len() != NUM_DIGITS) || (code != expected_code) {
        return Err(MyError::InvalidFiscalKey(
            path.as_ref().to_path_buf(),
            key_numeric.to_string(),
            code.to_string(),
            expected_code.to_string(),
        ));
    }

    Ok(true)
}

type CteInfo = (HashSet<String>, HashMap<String, HashSet<String>>);

/// Extracts relevant information from an XML file related to fiscal keys.
///
/// This function reads the content of an XML file, extracts specific fiscal keys for "CTe" (transport document)
/// and their complementary keys, and organizes them into a `HashSet` and a `HashMap`.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` representing the path to the XML file.
///
/// # Returns
///
/// A `Result` containing a tuple of `HashSet<String>` for "chave_cte" keys and `HashMap<String, HashSet<String>>`
/// for complementary "cte" keys if successful or a `MyError` if an error occurs.
pub fn extract_info_from_xml(file_path: &PathBuf) -> MyResult<CteInfo> {
    let content: String = slurp(file_path)?;
    let all_data = REGEX_REMOVE_NEWLINES_OR_SPACES.replace_all(&content, "");

    let mut chaves_cte: HashSet<String> = HashSet::new();
    let mut chaves_nfe: HashMap<String, HashSet<String>> = HashMap::new();

    for cap in REGEX_CHAVE_CTE_COMPL.captures_iter(&all_data) {
        let chave_cte: &str = cap.get(1).map_or("", |m| m.as_str());

        if chave_cte.is_empty() {
            continue;
        }

        if verify_key_code(file_path, chave_cte, CODE_CTE)? {
            chaves_cte.insert(chave_cte.to_string());
        }
    }

    for cap in REGEX_CHAVE_NFE.captures_iter(&all_data) {
        let origin: &str = cap.get(1).map_or("", |m| m.as_str());
        let chave_nfe: &str = cap.get(2).map_or("", |m| m.as_str());
        let fechamento: &str = cap.get(3).map_or("", |m| m.as_str());

        if (origin != fechamento) || origin.is_empty() || chave_nfe.is_empty() {
            continue;
        }

        if verify_key_code(file_path, chave_nfe, CODE_NFE)? {
            chaves_nfe
                .entry(chave_nfe.to_string())
                .or_default()
                .insert(origin.to_string());
        }
    }

    /*
    // check some files
    if chaves_cte.contains("352109123456789...1234...") {
        println!(
            "file_path: {}\nchaves_cte: {:#?}\nchaves_nfe: {:#?}\n",
            file_path.display(),
            chaves_cte,
            chaves_nfe
        );
    }
    */

    /*
    println!(
        "file_path: {}\nchaves_cte: {:#?}\nchaves_nfe: {:#?}\n",
        file_path.display(),
        chaves_cte,
        chaves_nfe
    );
    */

    Ok((chaves_cte, chaves_nfe)) // (key, value)
}

type MapCteComp = BTreeMap<String, BTreeMap<String, BTreeSet<String>>>;

/// Processes an array of file paths to extract and aggregate information about fiscal keys.
///
/// This function iterates over a collection of file paths, extracts relevant information from
/// each file using `extract_info_from_xml`, and aggregates this information into a
/// `BTreeMap` containing relationships between main keys and their complementary keys along with their origins.
///
/// # Arguments
///
/// * `array_of_files` - A slice of `PathBuf` representing the file paths to be processed.
///
/// # Returns
///
/// A `BTreeMap` where the keys are the main fiscal keys and values are `BTreeMap` where the keys
/// are the complementary fiscal keys and the values are `BTreeSet` of the origins.
pub fn fold_reduce(array_of_files: &[PathBuf]) -> MapCteComp {
    array_of_files
        .par_iter() // rayon: parallel iterator
        .flat_map(extract_info_from_xml) // obter (key, value)
        .filter(|(key, value)| !key.is_empty() && !value.is_empty())
        .fold(BTreeMap::new, |mut acc: MapCteComp, (key, value)| {
            for chave_cte in key {
                for (chave_nfe, origins) in &value {
                    acc.entry(chave_cte.clone())
                        .or_default()
                        .entry(chave_nfe.clone())
                        .or_default()
                        .extend(origins.clone());
                }
            }
            acc
        })
        .reduce(BTreeMap::new, |mut acc, map| {
            map.into_iter().for_each(|(key, values)| {
                for (chave_nfe, origins) in values {
                    acc.entry(key.clone())
                        .or_default()
                        .entry(chave_nfe)
                        .or_default()
                        .extend(origins);
                }
            });
            acc
        })
}

/// Prints the extracted and processed information to an output file.
///
/// This function iterates over the provided `BTreeMap`, formats the information into a specific
/// output format, and prints it to a specified output file. It also provides a summary of the
/// processed data at the end.
///
/// # Arguments
///
/// * `complementary_key_for_cte` - A reference to the `BTreeMap` containing the processed data.
/// * `config` - A reference to the `Config` struct containing the output file path.
///
/// # Returns
///
/// A `Result` that is `Ok(())` if the printing succeeds or `MyError` if an error occurs.
pub fn print_output(complementary_key_for_cte: &MapCteComp, config: &Config) -> MyResult<()> {
    let mut output_file = fs::File::create(&config.output_filename)
        .map_err(|e| MyError::FileWriteError(config.output_filename.clone(), e))?;

    let mut count_cte: HashSet<&String> = HashSet::new();
    let mut count_nfe: HashSet<&String> = HashSet::new();

    for (cte, btree_map) in complementary_key_for_cte {
        let mut nfes: BTreeSet<&String> = BTreeSet::new();
        let mut orig: BTreeSet<&String> = BTreeSet::new();

        btree_map
            .iter()
            .filter(|(chave_nfe, _origins)| *chave_nfe != cte)
            .try_for_each(|(chave_nfe, origins)| -> Result<(), MyError> {
                nfes.insert(chave_nfe);
                orig.extend(origins);
                Ok(())
            })?;

        let size = nfes.len();
        let keys: Vec<String> = nfes.iter().map(|s| s.to_string()).collect();

        writeln!(
            output_file,
            "cte: {cte}, {size} nfes: [{}]",
            keys.join(", "), // Imprime vetor sem aspas
        )
        .map_err(|e| MyError::FileWriteError(config.output_filename.clone(), e))?;

        count_cte.insert(cte);
        count_nfe.extend(nfes);
    }

    println!("Número de chaves CTe = {}", count_cte.len());
    println!("Número de chaves NFe = {}", count_nfe.len());
    println!("Todas as chaves de NFes foram vinculadas aos CTes correspondentes.");
    println!("Arquivo final: <{:#?}>\n", config.output_filename);

    Ok(())
}

#[cfg(test)]
mod lib_functions {
    use super::*;
    use std::path::PathBuf;

    // cargo test -- --help
    // cargo test -- --show-output

    type Error = Box<dyn std::error::Error>;

    #[test]
    /// `cargo test -- --show-output read_xml_content`
    fn read_xml_content() -> Result<(), Error> {
        let pathbuf_01 = PathBuf::from("./src/tests/teste_cte01.xml");
        let pathbuf_02 = PathBuf::from("./src/tests/teste_cte02.xml");
        let pathbuf_03 = PathBuf::from("./src/tests/teste_cte03.xml");

        //println!("file: {:?}", fs::read_to_string(&pathbuf_01)?);

        let array_of_files: Vec<PathBuf> = vec![pathbuf_01, pathbuf_02, pathbuf_03];

        let complementary_key_for_cte = fold_reduce(&array_of_files);

        println!("complementary_key_for_cte: {complementary_key_for_cte:#?}");

        let btree_set = BTreeSet::from(["infNFe".to_string()]);

        let mut btree_map: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        btree_map.insert(
            "35210123456789012345550000000112233444444444".to_string(),
            btree_set.clone(),
        );

        btree_map.insert(
            "35210123456789012345550000000112233446666666".to_string(),
            btree_set,
        );

        assert_eq!(
            complementary_key_for_cte["35210123456789012345570000000112233445566111"],
            btree_map
        );

        Ok(())
    }
}
