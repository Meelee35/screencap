use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, help = "where to save captures")]
    pub savedirectory: Option<PathBuf>,
    #[arg(short, long, help = "whether to save captures to the clipboard")]
    pub clipboard: Option<bool>,
}

pub fn args() -> Args {
    Args::parse()
}
