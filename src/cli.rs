use crate::core::{ProblemConfig, SourceFile};
use crate::core::{is_valid_problem_name, reformat_valid_name, create_problem_dir};

use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::{fmt::Debug, str::FromStr};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Creates new problem
    New { name: String },
    /// Info
    Info,
}

pub fn handle_command(command: Command) {
    match command {
        Command::New { name } => {
            create_problem_command(name);
        }
        _ => unimplemented!(),
    }
}

fn read_input<T: FromStr>(default: T) -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();
    if input.is_empty() {
        return default;
    }

    input.parse().expect("Invalid String entered")
}

pub fn create_problem_command(name: String) {
    assert!(is_valid_problem_name(&name));
    let reformated_name = reformat_valid_name(&name);
    print!("Problem Name (Default is {reformated_name}): ");
    let name = read_input(reformated_name);
    print!("Max Time in Seconds (Default is 1.0: ");
    let time = read_input(1.0);
    print!("Tags Saperated by commas (Default is 1.0: ");
    let tags = read_input(String::new());
    let tags: Vec<String> = tags.split(',').map(str::trim).map(str::to_string).collect();

    let path = current_dir().unwrap().join(&name);

    let config = ProblemConfig {
        name,
        time,
        tags,
        sources: Vec::new(),
        solutions: Vec::new(),
        testcases: Vec::new(),
        checker: None,
        validator: None,
    };

    create_problem_dir(&path, &config).unwrap();
}

fn get_current_problem_directory(mut cwd: &Path) -> &Path {
    while !cwd.join("problem_config.json").exists() {
        cwd = cwd
            .parent()
            .expect("Reached Maximum parent depth and didn't find problem_config.json");
    }
    return cwd;
}
