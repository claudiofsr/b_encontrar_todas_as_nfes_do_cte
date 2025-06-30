use std::path::PathBuf;

use crate::MyError;

const OUTPUT_FILENAME: &str = "cte_nfes.txt";
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Config {
    pub output_filename: PathBuf,
    pub name: String,
    pub author: String,
    pub version: String,
    pub num_files: usize,
    pub path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            output_filename: OUTPUT_FILENAME.into(),
            name: NAME.to_string(),
            author: AUTHOR.to_string(),
            version: VERSION.to_string(),
            num_files: 0,
            path: PathBuf::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_initial_info(&self) {
        let date = "05 de Fevereiro de 2025 (inicio: 10 de Janeiro de 2018)";
        let description = [
            "Este programa busca informações de chaves NFes/CTes em documentos fiscais de formato xml.",
            "Em seguida, retém todas as chaves de NFes vinculadas aos CTes correspondentes no arquivo:"
        ].join("\n");

        let msg = format!(
            "\n{name}\n{description}\n<{output:#?}>\n{author}\n{date}\nversão: {version}\n",
            name = &self.name,
            output = &self.output_filename,
            author = &self.author,
            version = &self.version,
        );

        println!("{msg}");
    }

    pub fn get_num_files(&mut self, files: &[PathBuf]) -> Result<(), MyError> {
        let output_filename = &self.output_filename;
        let num_files = files.len();

        self.num_files = num_files;

        if num_files < 1 {
            println!("Não foi gerado o arquivo <{output_filename:?}>.\n");
            Err(MyError::NoXmlFilesFound)
        } else {
            println!("Foram encontrados neste diretório {num_files} arquivos de formato '.xml'.");
            println!("Estes arquivos XML contém informações sobre CTe (cteProc|procEventoCTe).\n");
            Ok(())
        }
    }
}
