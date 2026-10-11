use crate::config::{Config, cli};

pub fn load() -> Config {
    let args = cli::args();
    Config::from(args)
}
