use clap::Parser;
use rustygon::cli::{handle_command, Command};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help=true)]
pub struct Cli {
    // Sets a custom config file
    //#[arg(short, long, value_name = "FILE")]
    //pub config: Option<PathBuf>,
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Command>,
} 
// TODO config
//#[derive(Default,Serialize, Deserialize)]
//struct Config {
//}

fn main() {
    let args = Cli::parse();
    //let config_path = args.config.unwrap_or(
    //    current_dir()
    //        .unwrap()
    //        .to_path_buf()
    //        .join("rustygon_config.json"),
    //);

    //let config = match config_path.exists() {
    //    true => Config::load_from_json(File::open(config_path)?)?,
    //    false => Config::default(),
    //};
    handle_command(args.command);
}
