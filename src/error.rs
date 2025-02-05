use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Could not open <{0}> for reading: {1}")]
    FileReadError(PathBuf, io::Error),

    #[error("Could not open <{0}> for writing: {1}")]
    FileWriteError(PathBuf, io::Error),

    #[error("Erro encontrado no arquivo <{0}>: \n chave_doc_fiscal '{1}' com codigo = {2} diferente de {3}")]
    InvalidFiscalKey(PathBuf, String, String, String),

    #[error("Error listing files: {0}")]
    FileListError(#[from] io::Error),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("No XML files found in the current directory!")]
    NoXmlFilesFound,

    #[error("No complementary CTes found for the given main CTes.")]
    NoComplementaryCtesFound,
}
