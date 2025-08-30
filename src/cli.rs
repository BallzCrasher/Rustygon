use crate::core::{
    add_source, create_problem_dir, is_valid_problem_name, reformat_valid_name, remove_source,
    ProblemConfig,
};

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
    Add(Element),
    /// Adds a component to the problem
    #[command(subcommand)]
    Remove(Element),
    /// Info
    Info,
}

#[derive(Subcommand)]
pub enum Element {
    Statement,
    Solution,
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
        Some(Command::Add(Element::Source { path })) => {
            add_source_command(&path);
        }
        Some(Command::Remove(Element::Source { path })) => {
            remove_source_command(&path);
        }
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
    let tags: Vec<String> = tags.split(',').map(str::trim).map(str::to_string).collect();

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
