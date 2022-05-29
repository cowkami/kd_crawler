mod api;

use anyhow::Result;
use api::KdApi;
use chrono::NaiveDate;

#[tokio::main]
async fn main() -> Result<()> {
    let kdapi = KdApi::new();
    let date = NaiveDate::from_ymd(2020, 10, 5);
    let body = kdapi.get_race_by_date(date).await?;
    println!("{:?}", body);
    Ok(())
}
