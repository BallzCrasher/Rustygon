use super::source::SourceFile;
use super::{modify_config, GenericResult};
use serde::{Deserialize, Serialize};
use std::fs::{copy, File};
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
) -> GenericResult {
    modify_config(cpd, |config| {
        let source_path = cpd.join("src/solutions").join(name);
        if let Some(path) = from {
            copy(path, &source_path)?;
        } else {
            File::create_new(&source_path)?;
        }

        config.solutions.push(Solution {
            sourcefile: SourceFile::from_filename(&source_path),
            verdict: verdict.clone(),
        });

        Ok(())
    })
}

pub fn remove_solution(cpd: &Path, name: &str) -> GenericResult {
    return modify_config(cpd, |config| {
        let pos = config
            .solutions
            .iter()
            .position(|x| x.sourcefile.source.file_name().unwrap().eq(name))
            .ok_or(format!("{name} was not found in problem_config.json"))?;

        config.solutions.remove(pos);
        std::fs::remove_file(cpd.join("src/solutions/").join(name))?;

        Ok(())
    });
}
