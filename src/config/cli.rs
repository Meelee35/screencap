use std::path::PathBuf;
use clap::Parser;
use crate::config::Config;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub savedirectory: Option<PathBuf>,
}



pub fn args() -> Args {
    Args::parse()
}