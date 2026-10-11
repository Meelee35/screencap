use crate::config::Config;
use crate::constants;

pub fn load() -> Result<Config, confy::ConfyError> {
    confy::load(constants::APP_NAME, Some("config"))
}
