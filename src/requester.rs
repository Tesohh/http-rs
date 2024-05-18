use anyhow::Context;
use reqwest::Request;

pub async fn run(request: Request, client: reqwest::Client) -> anyhow::Result<()> {
    let res = client.execute(request).await?;
    let headers = res.headers();

    for (key, value) in headers {
        println!("{}: {}", key, value.to_str()?);
    }
    println!("{}", res.text().await?);
    Ok(())
}
