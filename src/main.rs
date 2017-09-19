#[macro_use]
extern crate log;

extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate toml;

use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::env::Args;

#[derive(Debug, Deserialize)]
struct Config {
    path: String,
    default_args: Option<Vec<String>>,
}

fn main() {
    env_logger::init().unwrap();

    info!("Hello");

    debug!("args={}", get_args().fold(String::new(), |sum, entity| sum + &entity + " "));
    debug!("config_path={:?}", get_config_path());

    let mut config_file = File::open(get_config_path()).unwrap();
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str).unwrap();
    debug!("file={}", config_str);

    let config_toml: Config = toml::from_str(&config_str).unwrap();

    debug!("config={:?}", config_toml);

    let mut command = std::process::Command::new(config_toml.path);
    if let Some(args) = config_toml.default_args {
        debug!("set arg={:?}", args);
        command.args(args.iter()
            .map(|entity| entity.replace("{exe_dir}", get_exe_path().to_str().unwrap()))
            .collect::<Vec<_>>());
    }

    command.args(get_args())
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();

    info!("Bye");
}

fn get_args() -> Args {
    let mut args = std::env::args();
    args.next();
    args
}

fn get_config_path() -> PathBuf {
    let mut path = get_exe_path();
    path.push(get_config_name());
    path
}

fn get_exe_path() -> PathBuf {
    std::env::current_exe().unwrap().parent().unwrap().to_owned()
}

fn get_config_name() -> String {
    std::env::current_exe().unwrap().file_stem().unwrap().to_str().unwrap().to_string() + ".toml"
}
