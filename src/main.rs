pub mod builder;
pub mod requester;
pub mod selector;
pub mod varparser;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    pub path: PathBuf,
    pub select: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file = std::fs::read_to_string(args.path)?;

    let vars = varparser::parse_vars(&file);
    let file = selector::select(file, args.select);
    Ok(())
}
