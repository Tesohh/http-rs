pub mod builder;
pub mod requester;
pub mod selector;
pub mod varparser;
pub mod varreplacer;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    pub path: PathBuf,
    pub select: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file = std::fs::read_to_string(args.path)?;
    let client = reqwest::Client::new();

    let vars = varparser::parse_vars(&file);
    let file = varreplacer::replace_vars(file, &vars);
    let file = selector::select(file, args.select)?;
    let req = builder::build(file, &client)?;

    requester::run(req, client).await
}
