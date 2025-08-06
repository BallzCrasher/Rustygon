use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{
    fs::{create_dir, File, OpenOptions},
    path::{Path, PathBuf},
};

pub fn is_valid_problem_name(name: &str) -> bool {
    name.chars()
        .all(|c| c.is_lowercase() || c.is_ascii_digit() || c == '-')
}

pub fn reformat_valid_name(name: &str) -> String {
    assert!(is_valid_problem_name(name));
    name.split(|c| c == '-')
        .map(|s| {
            s.chars()
                .take(1)
                .flat_map(|f| f.to_uppercase())
                .chain(s.chars().skip(1))
                .collect::<String>()
        })
        .reduce(|x, y| x + " " + &y)
        .unwrap_or(String::new())
}

/// The Directory of the problem has this structure
/// problem/ # the problem directory
/// -- problem_config.json
/// -- src/  # the sources directory
/// -- -- files/ # contains source files for generator and validator and checker
/// -- -- solutions/ # contains the sources of the solutions
/// -- testcases/ # the testcases
/// -- -- input/ # the input of the testcases
/// -- -- output/ # the output of the testcases
/// -- text/ # contains all text files such as statement, tutorials, and testcases discriptions
/// -- bin/ # contains all binary compiled from the source files
pub fn create_problem_dir(path: &Path, config: &ProblemConfig) -> Result<(), Box<dyn Error>> {
    create_dir(path)?;
    create_dir(path.join("src"))?;
    create_dir(path.join("src").join("files"))?;
    create_dir(path.join("src").join("solutions"))?;
    create_dir(path.join("testcases"))?;
    create_dir(path.join("testcases").join("input"))?;
    create_dir(path.join("testcases").join("output"))?;
    create_dir(path.join("text"))?;
    create_dir(path.join("bin"))?;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path.join("problem_config.json"))?;
    Ok(config.save_to_file(file)?)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemConfig {
    pub title: String,
    pub time: f32,
    pub tags: Vec<String>,
    pub testcases: Vec<Testcase>,
    pub sources: Vec<SourceFile>,
    pub solutions: Vec<Solution>,
    pub validator: Option<usize>,
    pub checker: Option<usize>,
}

impl ProblemConfig {
    pub fn from_file(file: File) -> serde_json::Result<Self> {
        serde_json::from_reader(file)
    }

    pub fn save_to_file(&self, file: File) -> serde_json::Result<()> {
        serde_json::to_writer(file, self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Testcase {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub generate: bool,
    pub sample: bool,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub enum Verdict {
    AC,
    TLE,
    WA,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    sourcefile: SourceFile,
    verdict: Verdict,
}
