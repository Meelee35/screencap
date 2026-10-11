use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub savedirectory: Option<PathBuf>,
    #[arg(short, long)]
    pub clipboard: bool,
}



pub fn args() -> Args {
    Args::parse()
}