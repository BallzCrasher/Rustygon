use super::{modify_config, GenericResult};
use serde::{Deserialize, Serialize};
use std::fs::{copy, File};
use std::path::{Path, PathBuf};
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SourceFile {
    pub source: PathBuf,
    pub compiler: PathBuf,
    pub compiler_args: Vec<String>,
    pub bin: PathBuf,
    pub bin_args: Vec<String>
}

impl SourceFile {
    pub fn from_filename(filename: &Path) -> Self {
        match filename.extension().map(|ext| ext.to_str().unwrap()) {
            Some("cpp") => Self {
                source: filename.to_path_buf(),
                compiler: "g++".into(),
                compiler_args: vec!["%source%".into(), "-o".into(), "%bin%".into()],
                bin: filename.with_extension(".exe"),
                bin_args: vec![]
            },
            None | Some(_) => Self::default()
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
pub fn add_source(cpd: &Path, name: &str, from: Option<&Path>) -> GenericResult {
    modify_config(cpd, |config| {
        let source_path = cpd.join("src/sources").join(name);

        if let Some(path) = from {
            copy(path, &source_path)?;
        } else {
            File::create_new(&source_path)?;
        }

        config.sources.push(SourceFile::from_filename(&source_path));
        Ok(())
    })
}

pub fn remove_source(cpd: &Path, name: &str) -> GenericResult {
    modify_config(cpd, |config| {
        let pos = config
            .sources
            .iter()
            .position(|x| x.source.file_name().unwrap().eq(name))
            .ok_or(format!("{name} was not found in problem_config.json"))?;

        config.sources.remove(pos);
        std::fs::remove_file(cpd.join("src/sources/").join(name))?;
        Ok(())
    })
}

pub fn set_validator(cpd: &Path, name: &str) -> GenericResult {
    modify_config(cpd, |config| {
        let pos = config
            .sources
            .iter()
            .position(|x| x.source.file_name().unwrap().eq(name))
            .ok_or(format!("{name} was not found in problem_config.json"))?;

        config.validator = Some(pos);
        Ok(())
    })
}
