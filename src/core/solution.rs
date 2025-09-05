use super::source::SourceFile;
use super::ProblemConfig;
use serde::{Deserialize, Serialize};
use std::fs::{copy, File};
use std::io;
use std::path::Path;

#[derive(Debug, Default, Clone, Serialize, Deserialize, clap::ValueEnum)]
pub enum Verdict {
    #[default]
    AC,
    TLE,
    WA,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    sourcefile: SourceFile,
    verdict: Verdict,
}

/// Adds a solution file to the problem.
///
/// This will add the solution file to "{cpd}/src/solutions/{name}".
/// if `from` is not None. This will copy the content of `from` to the added file.
///
/// * `cpd`  - The problem directory to which we want to add the file
/// * `name` - The name of the source file we add
/// * `from` - if not None. the content of the solution file will be copied from this file.
/// * `verdict` - The expected verdict of the solution.
pub fn add_solution(
    cpd: &Path,
    name: &str,
    from: Option<&Path>,
    verdict: Verdict,
) -> Result<(), io::Error> {
    let config_file = File::open(cpd.join("problem_config.json"))?;
    let mut config = ProblemConfig::from_file(config_file)?;
    let source_path = cpd.join("src/solutions").join(name);

    if let Some(path) = from {
        copy(path, &source_path)?;
    } else {
        File::create_new(&source_path)?;
    }

    config.solutions.push(Solution {
        sourcefile: SourceFile::from_filename(&source_path),
        verdict,
    });

    let config_file = File::create(cpd.join("problem_config.json"))?;
    config.save_to_file(config_file)?;
    Ok(())
}

pub fn remove_solution(cpd: &Path, name: &str) -> Result<(), io::Error> {
    let config_file = File::open(cpd.join("problem_config.json"))?;
    let mut config = ProblemConfig::from_file(config_file).unwrap();

    let pos = config
        .solutions
        .iter()
        .position(|x| x.sourcefile.source.file_name().unwrap().eq(name))
        .ok_or(io::ErrorKind::NotFound)?;

    config.solutions.remove(pos);
    std::fs::remove_file(cpd.join("src/solutions/").join(name))?;

    let config_file = File::create(cpd.join("problem_config.json"))?;
    config.save_to_file(config_file)?;
    Ok(())
}
