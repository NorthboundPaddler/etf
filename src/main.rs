use clap::{ArgAction, Parser};
use serde::Deserialize;
use serde_yml;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use the local context, rather than the global one
    #[arg(required = false)]
    #[arg(action=ArgAction::SetTrue)]
    #[arg(short, long)]
    local: Option<bool>,
}

// The structure of the etf.yaml file used to configure the tool in runtime
#[derive(Debug, Deserialize)]
struct Config {
    task_data: std::path::PathBuf,
    completed_data: std::path::PathBuf,
}

fn get_config_path(place: Option<bool>) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let is_local = place.unwrap();
    let path = if is_local {
        std::path::PathBuf::from("./.etf.yml")
    } else {
        std::path::PathBuf::from("$XDG_CONFIG_DIR/.etf/etf.yml")
    };
    Ok(path)
}

fn read_config(fp: std::path::PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(fp)?;
    let config: Config = serde_yml::from_reader(file)?;
    println!("{:?}", config); // TODO: Remove after debugging
    return Ok(config);
}

fn main() {
    let args = Args::parse();
    let config_path = get_config_path(args.local);
    let config = read_config(config_path.unwrap());
}
