use crate::config::Config;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
    #[command(flatten)]
    pub config: ConfigArgs,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    ConfigWrite(ConfigArgs),
    ConfigOverride(ConfigArgs),
}

#[derive(clap::Args, Debug)]
pub struct ConfigArgs {
    #[arg(short, long, help = "where to save captures")]
    pub savedirectory: Option<PathBuf>,
    #[arg(short, long, help = "whether to save captures to the clipboard")]
    pub clipboard: Option<bool>,
}

impl ConfigArgs {
    pub fn apply(&self, config: &mut Config) {
        if let Some(save_directory) = self.savedirectory.clone() {
            config.save_directory = Some(save_directory);
        }
        if let Some(clipboard) = self.clipboard {
            config.clipboard = Some(clipboard);
        }
    }
}

pub fn args() -> Args {
    Args::parse()
}
