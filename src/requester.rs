use reqwest::Request;

pub async fn run(request: Request, client: reqwest::Client) -> anyhow::Result<()> {
    let res = client.execute(request).await?;
    Ok(())
}
