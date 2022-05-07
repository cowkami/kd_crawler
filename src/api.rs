use chrono::NaiveDate;
use reqwest::Url;
use std::error::Error;
struct KdUrl {
    url: Url,
}

impl KdUrl {
    fn new() -> Result<KdUrl, Box<dyn Error>> {
        Ok(Self {
            url: Url::parse("https://keirin.kdreams.jp/")?,
        })
    }
    fn base(base_url: &str) -> Result<KdUrl, Box<dyn Error>> {
        Ok(Self {
            url: Url::parse(base_url)?,
        })
    }
    fn kaisai(self) -> Result<KdUrl, Box<dyn Error>> {
        let url = self.url.join("kaisai")?;
        Ok(Self {
            url: Url::parse(url.as_str())?,
        })
    }
    fn date(self, date: NaiveDate) -> Result<KdUrl, Box<dyn Error>> {
        let date_str = date.format("%Y/%m/%d").to_string();
        let url = self.url.join(date_str.as_str())?;
        Ok(Self {
            url: Url::parse(url.as_str())?,
        })
    }
}

struct KdApi {
    base_url: Url,
}

impl KdApi {
    fn get_race_by_date(date: &str) {}
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
    fn date() {
        let date = NaiveDate::from_ymd(2022, 5, 16);
        assert_eq!(
            KdUrl::new().unwrap().date(date).unwrap().url.as_str(),
            "https://keirin.kdreams.jp/2022/05/16"
        )
    }
}
