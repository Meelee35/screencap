use crate::config::Config;
use crate::constants;
use std::path::PathBuf;

pub fn load() -> Result<Config, confy::ConfyError> {
    if !exists() {
        return Ok(Config::default());
    }
    confy::load_path(path()?)
}

pub fn save(config: &Config) -> Result<(), confy::ConfyError> {
    let path = path()?;
    confy::store_path(&path, config)?;
    log!("Wrote config to '{}'", path.display());
    Ok(())
}

pub fn exists() -> bool {
    path().map(|path| path.exists()).unwrap_or(false)
}

fn path() -> Result<PathBuf, confy::ConfyError> {
    confy::get_configuration_file_path(constants::APP_ID, Some("config"))
}
