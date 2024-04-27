use std::collections::HashMap;

use anyhow::Context;
use reqwest::{Client, Request, RequestBuilder};

pub fn build(file: String, vars: HashMap<String, String>, client: Client) -> anyhow::Result<()> {
    let mut lines = file.lines();

    let first = lines.nth(0).context("first line not found")?;
    let (method, url) = first
        .split_once(" ")
        .context("first line doesn't follow the method url pattern")?;

    let req: RequestBuilder = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "HEAD" => client.head(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        _ => anyhow::bail!("Unknown method"),
    };

    Ok(())
}
