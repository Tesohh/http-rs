use anyhow::Context;
use reqwest::{Client, RequestBuilder};

pub fn build(file: String, client: Client) -> anyhow::Result<reqwest::Request> {
    let first = &file.lines().nth(0).context("first line not found")?;
    let (method, url) = first
        .split_once(" ")
        .context("first line doesn't follow the method url pattern")?;

    let mut req: RequestBuilder = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "HEAD" => client.head(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        _ => anyhow::bail!("Unknown method"),
    };

    // add headers...
    let mut json_start: Option<usize> = None;
    for (i, line) in &mut file.lines().skip(1).enumerate() {
        if line.starts_with("{") {
            // start json parsing...
            json_start = Some(i);
            break;
        }

        if line.len() == 0 {
            // skip empty lines...
            continue;
        }

        let (header, value) = line
            .split_once(":")
            .context("header line doesn't follow the key value pattern")?;
        let (header, value) = (header.trim(), value.trim());
        req = req.header(header, value);
    }

    // at this point, we either return or add a body
    // in case the json has a start.. do that
    if let Some(start_idx) = json_start {
        let body: Vec<&str> = (&file).lines().skip(start_idx + 1).collect();
        let body = body.join("\n");

        req = req.body(body);
    }

    Ok(req.build()?)
}
