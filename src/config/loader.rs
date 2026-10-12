use crate::config::Config;
use crate::config::cli::ConfigArgs;
use crate::config::file;

pub fn load(overrides: &ConfigArgs) -> Config {
    let mut config = match file::load() {
        Ok(config) => config,
        Err(err) => {
            warn!("Unable to load config file: {err}");
            Config::default()
        }
    };
    overrides.apply(&mut config);
    config
}
