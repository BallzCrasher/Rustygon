use super::ProblemConfig;
use serde::{Deserialize, Serialize};
use std::fs::{copy, File};
use std::io;
use std::path::{Path, PathBuf};
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFile {
    pub source: PathBuf,
    pub build_command: String,
    pub exec_command: String,
}

impl SourceFile {
    pub fn from_filename(filename: &Path) -> Self {
        match filename.extension().map(|ext| ext.to_str().unwrap()) {
            Some("cpp") => Self {
                source: filename.to_path_buf(),
                build_command: "g++ %source% -o %bin%".to_string(),
                exec_command: String::new(),
            },
            None | Some(_) => Self {
                source: filename.to_path_buf(),
                build_command: String::new(),
                exec_command: String::new(),
            },
        }
    }
}

/// Adds a source file to the problem.
///
/// This will add the source file to "{cpd}/src/sources/{name}".
/// if `from` is not None. This will copy the content of `from` to the added file.
///
/// * `cpd`  - The problem directory to which we want to add the source file
/// * `name` - The name of the source file we add
/// * `from` - if not None. the content of the source file will be copied from this file.
pub fn add_source(cpd: &Path, name: &str, from: Option<&Path>) -> Result<(), io::Error> {
    let config_file = File::open(cpd.join("problem_config.json"))?;
    let mut config = ProblemConfig::from_file(config_file).unwrap();
    let source_path = cpd.join("src/sources").join(name);

    if let Some(path) = from {
        copy(path, &source_path)?;
    } else {
        File::create_new(&source_path)?;
    }

    config.sources.push(SourceFile::from_filename(&source_path));

    let config_file = File::create(cpd.join("problem_config.json"))?;
    config.save_to_file(config_file)?;
    Ok(())
}

pub fn remove_source(cpd: &Path, name: &str) -> Result<(), io::Error> {
    let config_file = File::open(cpd.join("problem_config.json"))?;
    let mut config = ProblemConfig::from_file(config_file).unwrap();

    let pos = config
        .sources
        .iter()
        .position(|x| x.source.file_name().unwrap().eq(name))
        .ok_or(io::ErrorKind::NotFound)?;

    config.sources.remove(pos);
    std::fs::remove_file(cpd.join("src/sources/").join(name))?;

    let config_file = File::create(cpd.join("problem_config.json"))?;
    config.save_to_file(config_file)?;
    Ok(())
}
