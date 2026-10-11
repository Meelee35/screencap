use crate::config::{Config, cli, file};

pub fn load() -> Config {
    let args = cli::args();
    let mut config = Config::default();
    match file::load() {
        Ok(cfg) => config = cfg,
        Err(err) => {
            warn!("Unable to load config file: {err}");
        }
    }
    if let Some(save_directory) = args.savedirectory {
        config.save_directory = Some(save_directory);
    }
    if let Some(clipboard) = args.clipboard {
        config.clipboard = Some(clipboard);
    }
    config
}
