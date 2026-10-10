use crate::config::{cli, Config};

pub fn load() -> Config {
    let args = cli::args();
    Config::from(args)
}