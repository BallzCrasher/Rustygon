use crate::core::solution::*;
use crate::core::source::*;
use crate::core::{create_problem_dir, is_valid_problem_name, reformat_valid_name, ProblemConfig};

use std::env::current_dir;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::{fmt::Debug, str::FromStr};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Creates new problem
    New { name: String },

    /// Adds a component to the problem
    #[command(subcommand)]
    Add(AddArg),

    /// Removes a component from the problem
    #[command(subcommand)]
    Remove(RemoveArg),

    /// Sets a component or a setting
    #[command(subcommand)]
    Set(SetArg),
    /// Info
    Info,
}

#[derive(Subcommand)]
pub enum AddArg {
    Statement,
    Solution {
        path: PathBuf,
        #[arg(value_enum)]
        verdict: Option<Verdict>,
    },
    Source {
        path: PathBuf,
    }, // TODO: make it path: name
}

#[derive(Subcommand)]
pub enum SetArg {
    Statement,
    MainSolution { path: PathBuf },
    Validator { path: PathBuf },
    Title { title: String },
    Time { time: f32 },
    Checker { path: PathBuf },
    Tags { tags: String },
}

#[derive(Subcommand)]
pub enum RemoveArg {
    Statement,
    Solution { path: PathBuf },
    Source { path: PathBuf }, // TODO: make it path: name
}

pub fn handle_command(command: Option<Command>) {
    match command {
        Some(Command::New { name }) => {
            create_problem_command(name);
        }
        Some(Command::Info) => {
            print_problem_info();
        }
        Some(Command::Add(AddArg::Source { path })) => {
            add_source_command(&path);
        }
        Some(Command::Add(AddArg::Solution { path, verdict })) => {
            add_solution_command(&path, verdict);
        }
        Some(Command::Remove(RemoveArg::Source { path })) => {
            remove_source_command(&path);
        }
        Some(Command::Remove(RemoveArg::Solution { path })) => {
            // TODO: fix [verdict] showing
            // at the end
            remove_solution_command(&path);
        }
        Some(Command::Set(SetArg::Validator { path })) => set_validator_command(&path),
        None => {}
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

    print!("Problem Title (Default is {reformated_name}): ");
    stdout().flush().unwrap();
    let title = read_input(reformated_name);

    print!("Max Time in Seconds (Default is 1.0): ");
    stdout().flush().unwrap();
    let time = read_input(1.0);

    print!("Tags Saperated by commas (Default is Empty): ");
    stdout().flush().unwrap();
    let tags = read_input(String::new());
    let tags: Vec<String> = tags
        .split(',')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect();

    let path = current_dir().unwrap().join(&name);

    let config = ProblemConfig {
        title,
        time,
        tags,
        sources: Vec::new(),
        solutions: Vec::new(),
        testcases: Vec::new(),
        checker: None,
        validator: None,
    };

    println!("{config:?}");

    create_problem_dir(&path, &config).unwrap();
}

pub fn print_problem_info() {
    let problem_path = get_current_problem_directory();
    let problem_config_path = problem_path.join("problem_config.json");
    let problem_config = ProblemConfig::from_file(File::open(problem_config_path).unwrap());

    println!("{:?}", problem_config);
}

fn get_current_problem_directory() -> PathBuf {
    let cwd = current_dir().unwrap();
    let mut path = cwd.as_path();
    while !path.join("problem_config.json").exists() {
        path = path
            .parent()
            .expect("Reached Maximum parent depth and didn't find problem_config.json");
    }
    return path.to_owned();
}

fn add_source_command(path: &Path) {
    let cpd = get_current_problem_directory();

    if path.exists() && path.is_file() {
        let filename = path.file_name().unwrap().to_str().unwrap();
        add_source(&cpd, filename, Some(&path)).unwrap();
    } else if let Some(name) = path.file_name() {
        add_source(&cpd, name.to_str().unwrap(), None).unwrap();
    } else {
        eprintln!("Invalid input.");
        return;
    }

    println!("Done");
}

fn remove_source_command(path: &Path) {
    let cpd = get_current_problem_directory();
    remove_source(&cpd, path.file_name().unwrap().to_str().unwrap()).unwrap();
    println!("Done");
}

fn add_solution_command(path: &Path, verdict: Option<Verdict>) {
    let cpd = get_current_problem_directory();
    let verdict = verdict.unwrap_or_default();

    if path.exists() && path.is_file() {
        let filename = path.file_name().unwrap().to_str().unwrap();
        add_solution(&cpd, filename, Some(&path), verdict).unwrap();
    } else if let Some(name) = path.file_name() {
        add_solution(&cpd, name.to_str().unwrap(), None, verdict).unwrap();
    } else {
        eprintln!("Invalid input.");
        return;
    }

    println!("Done");
}

fn remove_solution_command(path: &Path) {
    let cpd = get_current_problem_directory();
    remove_solution(&cpd, path.file_name().unwrap().to_str().unwrap()).unwrap();
    println!("Done");
}

fn set_validator_command(name: &Path) {
    let cpd = get_current_problem_directory();
    set_validator(&cpd, name.file_name().unwrap().to_str().unwrap()).unwrap();
    println!("Done");
}
