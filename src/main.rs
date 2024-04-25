pub mod builder;
pub mod requester;

use std::{borrow::Borrow, path::PathBuf};

use clap::Parser;
use fzf_wrapped::{Fzf, Layout};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    pub path: PathBuf,
    pub select: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file = std::fs::read_to_string(args.path)?;
    let client = reqwest::Client::new();

    let mut files: Vec<String> = vec![];
    let mut accumulator: Vec<String> = vec![];
    for i in file.lines() {
        if i.len() == 0 || i.starts_with("@") {
            continue;
        }

        if i.starts_with("###") {
            if accumulator.len() == 0 {
                continue;
            }
            files.push(accumulator.clone().join("\n"));
            accumulator = vec![];
            continue;
        }
        accumulator.push(i.trim().to_string());
    }

    if accumulator.len() != 0 {
        files.push(accumulator.clone().join("\n"));
    }

    let mut selection_idx: Option<usize> = None;
    let selections: Vec<_> = files.iter().filter_map(|f| f.lines().nth(0)).collect();

    if let Some(select) = args.select {
        for (idx, v) in selections.iter().enumerate() {
            let words: Vec<_> = v.split(|c| c == ' ' || c == '/').collect();
            let s = &select.borrow();
            if words.contains(s) {
                selection_idx = Some(idx);
                break;
            }
        }
    } else {
        let mut fzf = Fzf::builder()
            .layout(Layout::Reverse)
            .custom_args(["--height=10%"])
            .build()?;

        fzf.run().expect("cannot run fzf. is it installed?");
        fzf.add_items(selections.clone())
            .expect("cannot add items to fzf.");
        let output = fzf.output().expect("cannot get the output from fzf.");

        selection_idx = selections.iter().position(|&s| s == output);
    }

    if selection_idx.is_none() {
        anyhow::bail!("selection not provided.")
    }

    // let req = builder::build(file, client);
    // requester::run(req)
    Ok(())
}
