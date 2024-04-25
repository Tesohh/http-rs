use anyhow::Context;
use reqwest::{Client, Request};

pub fn build(file: String, client: Client) -> anyhow::Result<()> {
    let mut lines = file.lines();
    let first = lines.nth(0).context("first line not found")?;
    Ok(())
}
