mod api;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = "https://keirin.kdreams.jp/kaisai/2022/05/01/";
    let body = reqwest::get(base_url).await?.text().await?;
    println!("{:?}", body);
    Ok(())
}
