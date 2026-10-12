use crate::config::cli::ConfigArgs;
use crate::config::{Config, file};

pub fn write(args: &ConfigArgs) -> anyhow::Result<()> {
    let mut config = Config::default();
    args.apply(&mut config);
    file::save(&config)?;
    Ok(())
}

pub fn override_values(args: &ConfigArgs) -> anyhow::Result<()> {
    let mut config = file::load()?;
    args.apply(&mut config);
    file::save(&config)?;
    Ok(())
}
