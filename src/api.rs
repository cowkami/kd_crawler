use anyhow::Result;
use chrono::NaiveDate;
use reqwest::Url;
struct KdUrl {
    url: Url,
}

impl KdUrl {
    fn new() -> Result<KdUrl> {
        Ok(Self {
            url: Url::parse("https://keirin.kdreams.jp/")?,
        })
    }
    fn base(base_url: &str) -> Result<KdUrl> {
        Ok(Self {
            url: Url::parse(base_url)?,
        })
    }
    fn kaisai(self) -> Result<KdUrl> {
        let url = self.url.join("kaisai")?;
        Ok(Self {
            url: Url::parse(url.as_str())?,
        })
    }
    fn schedule(self) -> Result<KdUrl> {
        let url = self.url.join("schedule")?;
        Ok(Self {
            url: Url::parse(url.as_str())?,
        })
    }
    fn date(self, date: NaiveDate) -> Result<KdUrl> {
        let date_str = date.format("%Y/%m/%d").to_string();
        let url = self.url.join(date_str.as_str())?;
        Ok(Self {
            url: Url::parse(url.as_str())?,
        })
    }
}

struct KdApi {
    base_url: KdUrl,
}

impl KdApi {
    fn new() -> Self {
        Self {
            base_url: KdUrl::new().unwrap(),
        }
    }
    async fn get_race_by_date(self, date: NaiveDate) -> Result<String> {
        let url = self.base_url.kaisai()?.date(date)?.url;
        let body = reqwest::get(url).await?.text().await?;
        Ok(body)
    }
}

#[cfg(test)]
mod kdurl_tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(
            KdUrl::new().unwrap().url.as_str(),
            KdUrl::base("https://keirin.kdreams.jp/")
                .unwrap()
                .url
                .as_str()
        );
    }
    #[test]
    fn kaisai() {
        assert_eq!(
            KdUrl::new().unwrap().kaisai().unwrap().url.as_str(),
            "https://keirin.kdreams.jp/kaisai"
        )
    }
    #[test]
    fn schedule() {
        assert_eq!(
            KdUrl::new().unwrap().schedule().unwrap().url.as_str(),
            "https://keirin.kdreams.jp/schedule"
        )
    }

    #[test]
    fn date() {
        let date = NaiveDate::from_ymd(2022, 5, 16);
        assert_eq!(
            KdUrl::new().unwrap().date(date).unwrap().url.as_str(),
            "https://keirin.kdreams.jp/2022/05/16"
        )
    }
}
