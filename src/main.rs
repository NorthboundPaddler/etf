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

fn check_for_git_repo(
    inpath: std::path::PathBuf,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    match git2::Repository::discover(inpath) {
        Ok(repo) => {
            let outpath = std::path::PathBuf::from(repo.path());
            return Ok(outpath);
        }
        Err(err) => return Err(Box::new(err)),
    }
}

fn get_config_path(place: Option<bool>) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let is_local = place.unwrap();
    let path: std::path::PathBuf = if is_local {
        let mut checked_path = check_for_git_repo(".".into()).unwrap();
        checked_path.pop(); //Remove the .git folder from the path
        checked_path.push(".etf-config.yaml");
        checked_path
    } else {
        // TODO: Make this check for a missing env var more graceful
        let xdg_dir = std::env::var_os("XDG_CONFIG_HOME");
        let mut global_path = std::path::PathBuf::new();
        global_path.push(xdg_dir.expect("The $XDG_CONFIG_HOME environment variable is not set"));
        global_path.push("etf");
        global_path.push("etf-config.yaml");
        global_path
    };
    Ok(path)
}

fn read_config(fp: std::path::PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    println!("Reading config located at: {:?}", &fp); // TODO Remove this after testing 
    let file = std::fs::File::open(fp)?;
    let config: Config = serde_yml::from_reader(file)?;
    return Ok(config);
}

fn main() {
    let args = Args::parse();
    let config_path = get_config_path(args.local);
    let config = read_config(config_path.unwrap());
    println!("{:?}", config.unwrap()); // TODO: Remove after debugging
}
