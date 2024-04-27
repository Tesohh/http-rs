use std::borrow::Borrow;

use anyhow::Context;
use fzf_wrapped::Fzf;

/// Separates a .http file into multiple "files", and selects the correct one.
///
/// If specified as an arg: choose the one that contains the arg
///
/// If not specified: call fzf to let the user choose
///
/// Make sure to call parse_vars before this, as this will purge all @ lines and assumes it's
/// already been done before
pub fn select(file: String, select_arg: Option<String>) -> anyhow::Result<String> {
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

    if files.len() == 1 {
        return Ok(files
            .get(0)
            .cloned()
            .expect("somehow there is one file but there is no file"));
    }

    let mut selection_idx: Option<usize> = None;
    let selections: Vec<_> = files.iter().filter_map(|f| f.lines().nth(0)).collect();

    if let Some(select) = select_arg {
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
            .layout(fzf_wrapped::Layout::Reverse)
            .custom_args(["--height=10%"])
            .build()?;

        fzf.run().expect("cannot run fzf. is it installed?");
        fzf.add_items(selections.clone())
            .expect("cannot add items to fzf.");
        let output = fzf.output().expect("cannot get the output from fzf.");

        selection_idx = selections.iter().position(|&s| s == output);
    }

    match selection_idx {
        Some(selection_idx) => files
            .get(selection_idx)
            .context("selection index doesn't match anything")
            .cloned(),

        None => anyhow::bail!("selection not provided."),
    }
}
